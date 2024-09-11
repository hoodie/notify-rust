#![allow(missing_docs)]
use super::server::*;
use event_listener::Event;
/// Notification Server
///
/// ## Todo:
/// * [ ] handle _multiple_ actions from "UI"
/// * [x] handle close from "UI"
/// * [ ] handle update from "UI"
///
use futures_util::{future::pending, select, FutureExt};
use std::{collections::HashMap, marker::PhantomData, sync::Arc, time::Duration};
use zbus::{dbus_interface, interface, Connection, SignalContext};

use crate::{
    xdg::NotificationBus, CloseReason, Hint, ServerInformation, Timeout, NOTIFICATION_DEFAULT_BUS,
    NOTIFICATION_OBJECTPATH,
};
// ////// Server

struct NotificationServer<
    H: NotificationHandler<T> + 'static + Sync + Send + Clone,
    T: 'static + Sync + Send + Clone,
> {
    count: u32,
    config: NotificationServerConfig,
    handler: H,
    stop_event: Arc<Event>,
    _phantom: PhantomData<T>,
}

impl<T, H> NotificationServer<H, T>
where
    T: 'static + Sync + Send + Clone,
    H: NotificationHandler<T> + 'static + Sync + Send + Clone,
{
    fn with_handler(handler: H) -> Self {
        let stop_event = Event::new();
        let stop_event = Arc::new(stop_event);

        NotificationServer {
            count: 0,
            config: NotificationServerConfig::default(),
            handler,
            stop_event,
            _phantom: Default::default(),
        }
    }
}

fn close_timeout(timeout: i32, default_timeout: u64) -> Option<Duration> {
    // time should not be shorter than this to avoid bypassing the original response
    let minimum_timeout = 10; // ms

    if timeout == 0 {
        None // sleep do not expire at all, user must close
    } else if timeout < 0 {
        Some(Duration::from_millis(default_timeout))
    } else if (timeout as u64) < minimum_timeout {
        log::warn!("timeout is below minimum timeout of {minimum_timeout}ms -> falling back to minimum timeout");
        Some(Duration::from_millis(minimum_timeout))
    } else {
        Some(Duration::from_millis(timeout as u64))
    }
}

#[test]
fn test_timeout_before_close() {
    assert_eq!(close_timeout(0, 3000), None);
    assert_eq!(close_timeout(1, 3000), Some(Duration::from_millis(10)));
    assert_eq!(close_timeout(-1, 3000), Some(Duration::from_millis(3000)));
    assert_eq!(close_timeout(256, 3000), Some(Duration::from_millis(256)));
}

async fn sleep_before_close(time_to_sleep: Option<Duration>) {
    if let Some(time_to_sleep) = time_to_sleep {
        async_std::task::sleep(time_to_sleep).await;
    } else {
        pending::<()>().await;
    }
}

#[derive(Debug)]
pub enum HandledNotification {
    Closed(CloseReason),
    Handled,
}

#[zbus::interface(name = "org.freedesktop.Notifications")]
impl<H, T> NotificationServer<H, T>
where
    H: NotificationHandler<T> + 'static + Sync + Send + Clone,
    T: 'static + Sync + Send + Clone,
{
    /// Can be `async` as well.
    #[allow(clippy::too_many_arguments)]
    fn get_server_information(&self) -> ServerInformation {
        log::trace!("received info request");
        ServerInformation {
            name: String::from("name"),
            vendor: String::from("hoodie"),
            version: String::from(env!("CARGO_PKG_VERSION")),
            spec_version: String::from("1.1"),
        }
    }

    #[allow(clippy::too_many_arguments)]
    async fn notify(
        &mut self,
        appname: String,
        replace_id: u32,
        icon: String,
        summary: String,
        body: String,
        raw_actions: Vec<String>,
        raw_hints: HashMap<String, zbus::zvariant::OwnedValue>,
        timeout: i32,
        #[zbus(signal_context)] ctx: SignalContext<'_>,
    ) -> zbus::fdo::Result<u32> {
        self.count += 1;
        let id = self.count;

        let actions = Action::from_vec(&raw_actions);
        let hints = collect_hints(raw_hints);

        let (action_tx, action_rx) = async_std::channel::bounded(5);
        let (close_tx, close_rx) = async_std::channel::bounded(1);

        log::trace!("Notification #{id} received timeout: {timeout}, {summary:?}");

        let persistent = hints.contains(&Hint::Resident(true)) || Timeout::Never == timeout;
        if persistent {
            log::trace!("Notification #{id} is persistent");
        } else {
            log::trace!("Notification #{id} will expire after {timeout}ms");
        }

        let received = ReceivedNotification {
            appname,
            id,
            replace_id,
            icon,
            summary,
            body,
            actions,
            hints,
            timeout,
            action_tx: action_tx.downgrade(),
            close_tx: close_tx.downgrade(),
        };

        let time_to_sleep = close_timeout(timeout, self.config.default_timeout);

        let handler = self.handler.clone();
        let mut handler_task =
            async_std::task::spawn(async move { handler.call(received).await }).fuse();

        // compile_error!(
        //     r#"Welcome Back, here is where you left off :D
        // 1. please remove the close_channel, that should be the return value of the handler instead.
        // 2. handle persistent notifications better: allow multiple actions, don't close on action
        // 3. consider using `std::ops::ControlFlow`"#
        // );
        let ctx_in_task = ctx.to_owned();

        let notification_is_open = async move {
            let _close_tx_lifetime = close_tx.clone();
            let _action_tx_lifetime = action_tx.clone();

            let close_with_reason = |ctx, reason: CloseReason| async move {
                if let Err(error) = Self::notification_closed(ctx, id, reason).await {
                    log::warn!("Notification #{id}, failed to send close signal ({error})");
                    Err(format!(
                        "Notification #{id}, failed to send close signal ({error})"
                    ))
                } else {
                    Ok(())
                }
            };

            log::trace!("Notification #{id}: handler loop start");
            loop {
                select! {
                    action = action_rx.recv().fuse() => {
                        log::trace!("Notification #{id}: action received (persistent: {persistent})");

                        match action {
                            Ok(action) =>  {
                                let invoked = Self::action_invoked(&ctx_in_task, id, &action).await;
                                log::trace!("Notification #{id}: action invoked {action}");
                                if let Err(error) = invoked {
                                    log::warn!("Notification #{id}, failed to send action signal ({error})");
                                    return Err(format!("Notification #{id}, failed to send action signal ({error})"))
                                }
                                if !persistent {
                                    log::trace!("Notification #{id} is not persistent: closing after action");
                                    return close_with_reason(&ctx_in_task, CloseReason::CloseAction).await;
                                }
                            }
                            Err(error) =>  {
                                log::warn!("Notification #{id}, failed to receive action ({error})");
                                return Err(format!("Notification #{id}, failed to receive action ({error})"));
                            }
                        };

                        log::trace!("Notification #{id} action handled, loop should continue");
                    }
                    reason = close_rx.recv().fuse() => {
                        match reason {
                            Ok(reason) => {
                                log::trace!("Notification #{id}: closed by user ({reason:?})");
                                return close_with_reason(&ctx_in_task ,reason).await;
                            }
                            Err(error) => {
                                log::warn!("Notification #{id}, failed to receive close {error}");
                                return Err(format!("Notification #{id}, failed to receive close {error}"));
                            }
                        }
                    }
                    result = handler_task => {
                        // TODO: IF persistent don't close after handler has returned
                        match result {
                            Ok(Some(reason))=> {
                                log::trace!("Notification #{id}: handler finished");
                                return close_with_reason(&ctx_in_task, reason).await;
                            }
                            Ok(None)=> {
                                log::trace!("Notification #{id}: handler finished");
                                return close_with_reason(&ctx_in_task, CloseReason::Dismissed).await; // TODO: not sure if Dismissed is correct
                            }
                            Err(error)  => {
                                log::warn!("Notification #{id}: handler errored {error}");
                                return Err(error)
                            }
                        }
                    }
                    _ = sleep_before_close(time_to_sleep).fuse() => {
                        log::trace!("Notification #{id}: expired after {time_to_sleep:?}" );
                        return close_with_reason(&ctx_in_task, CloseReason::Expired).await;
                    }
                }
            }
        };

        async_std::task::spawn(notification_is_open);

        Ok(id)
    }

    async fn stop(&self) -> bool {
        // TODO: maybe use ObjectServer::remove instead of sleeping
        log::info!("received stop");

        let cause_stop = Arc::downgrade(&self.stop_event);
        async_std::task::spawn(async move {
            async_std::task::sleep(Duration::from_millis(500)).await;
            if let Some(stop_event) = cause_stop.upgrade() {
                stop_event.notify(1);
            }
        });
        true
    }

    #[zbus(signal)]
    async fn action_invoked(ctx: &SignalContext<'_>, id: u32, action: &str) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn notification_closed(
        ctx: &SignalContext<'_>,
        id: u32,
        reason: CloseReason,
    ) -> zbus::Result<()>;
}

pub struct ServerHandle {
    #[allow(dead_code)]
    connection: Option<Connection>,
    stopped: Arc<Event>,
}
impl ServerHandle {
    pub fn stop(&mut self) {
        self.connection = None;
    }
    pub async fn stopped(&self) {
        self.stopped.listen().await;
    }
}

/// Starts the server
pub async fn start<H, T>(handler: H) -> crate::error::Result<ServerHandle>
where
    H: NotificationHandler<T> + 'static + Sync + Send + Clone,
    T: 'static + Sync + Send + Clone,
{
    start_at(NOTIFICATION_OBJECTPATH, handler).await
}

pub async fn start_at<H, T>(sub_bus: &str, handler: H) -> crate::error::Result<ServerHandle>
where
    H: NotificationHandler<T> + 'static + Sync + Send + Clone,
    T: 'static + Sync + Send + Clone,
{
    let server_state = NotificationServer::with_handler(handler);
    let bus = NotificationBus::custom(sub_bus)?;

    log::info!(
        "instantiated server ({NOTIFICATION_DEFAULT_BUS}) at {:?}",
        bus
    );
    // let stopped = std::sync::Arc::downgrade(&server_state.stop_rx);
    let stopped = server_state.stop_event.clone();

    let interface_name = <NotificationServer<H, T> as zbus::Interface>::name();

    log::info!("launching session at {sub_bus:?}\n {NOTIFICATION_DEFAULT_BUS:?} \n {interface_name:?}\n {bus:?}");
    let connection = zbus::ConnectionBuilder::session()?
        .name(bus.clone().into_name())?
        .serve_at(NOTIFICATION_OBJECTPATH, server_state)?
        .build()
        .await?;
    log::info!("launched");

    let handle = ServerHandle {
        connection: Some(connection),
        stopped,
    };

    Ok(handle)
}

/// Starts the server
// FIXME: add proper server error type
pub fn start_blocking<H, T>(handler: H) -> crate::error::Result<ServerHandle>
where
    H: NotificationHandler<T> + 'static + Sync + Send + Clone,
    T: 'static + Sync + Send + Clone,
{
    log::info!("start blocking");
    zbus::block_on(start(handler))
}

pub fn stop(sub_bus: &str) -> crate::error::Result<()> {
    let bus = NotificationBus::custom(sub_bus)?;
    let connection = zbus::blocking::Connection::session()?;
    connection.call_method(
        Some(bus.into_name()),
        NOTIFICATION_OBJECTPATH,
        Some(NOTIFICATION_DEFAULT_BUS),
        "Stop",
        &(),
    )?;

    Ok(())
}
