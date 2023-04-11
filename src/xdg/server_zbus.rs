#![allow(missing_docs)]
use super::server::*;
/// Notification Server
///
/// ## Todo:
/// * [x] handle actions from "UI"
/// * [x] handle close from "UI"
/// * [ ] handle update from "UI"
///
use futures_util::{future::pending, select, try_join, FutureExt};
use std::{collections::HashMap, error::Error, time::Duration};
use zbus::{dbus_interface, SignalContext};

use crate::{
    xdg::NotificationBus, CloseReason, ServerInformation, NOTIFICATION_DEFAULT_BUS,
    NOTIFICATION_OBJECTPATH,
};
// ////// Server

struct NotificationServer<H: NotificationHandler + 'static + Sync + Send + Clone> {
    count: u32,
    config: NotificationServerConfig,
    handler: H,
    stop_event: std::sync::Arc<event_listener::Event>,
}

impl<H: NotificationHandler + 'static + Sync + Send + Clone> NotificationServer<H> {
    fn with_handler(handler: H) -> Self {
        let stop_event = event_listener::Event::new();
        let stop_event = std::sync::Arc::new(stop_event);

        NotificationServer {
            count: 0,
            config: NotificationServerConfig::default(),
            handler,
            stop_event,
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

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl<H> NotificationServer<H>
where
    H: NotificationHandler + 'static + Sync + Send + Clone,
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

        let (action_tx, action_rx) = async_std::channel::bounded(1);
        let (close_tx, close_rx) = async_std::channel::bounded(1);

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
        let handler_task = async_std::task::spawn(async move {
            handler.call(received).await;
        });

        let ctx_in_task = ctx.to_owned();
        async_std::task::spawn(async move {
            let _close_tx_lifetime = close_tx.clone();
            let _action_tx_lifetime = action_tx.clone();

            // waiting for actions and close notifications
            let (reason, action) = select! {
                action = action_rx.recv().fuse() => {
                    log::trace!("close from action");
                    (CloseReason::CloseAction, action.ok())
                }
                reason = close_rx.recv().fuse() => {
                    log::trace!("close from user");
                    (reason.unwrap_or(CloseReason::Expired), None)
                }
                _ = sleep_before_close(time_to_sleep).fuse() => {
                    log::trace!("close from expire after {time_to_sleep:?}", );
                    (CloseReason::Expired, None)
                }
            };

            // respond to notification
            if let Some(action) = action {
                if let Err(error) = try_join!(
                    Self::notification_closed(&ctx_in_task, id, reason),
                    Self::action_invoked(&ctx_in_task, id, &action)
                ) {
                    log::warn!("failed to send invoked action signal {error}");
                }
            } else if let Err(error) = Self::notification_closed(&ctx_in_task, id, reason).await {
                log::warn!("failed to send closed signal {error}");
            }
            if handler_task.cancel().await.is_none() {
                log::warn!("canceling notification handler that was already done");
            }
        });

        Ok(id)
    }

    async fn stop(&self) -> bool {
        // TODO: maybe use ObjectServer::remove instead of sleeping
        log::info!("received stop");

        let cause_stop = std::sync::Arc::downgrade(&self.stop_event);
        async_std::task::spawn(async move {
            async_std::task::sleep(Duration::from_millis(500)).await;
            if let Some(stop_event) = cause_stop.upgrade() {
                stop_event.notify(1);
            }
        });
        true
    }

    #[dbus_interface(signal)]
    async fn action_invoked(ctx: &SignalContext<'_>, id: u32, action: &str) -> zbus::Result<()>;

    #[dbus_interface(signal)]
    async fn notification_closed(
        ctx: &SignalContext<'_>,
        id: u32,
        reason: CloseReason,
    ) -> zbus::Result<()>;
}

/// Starts the server
pub async fn start<H: NotificationHandler + 'static + Sync + Send + Clone>(
    handler: H,
) -> Result<(), Box<dyn Error + Send>> {
    start_at(NOTIFICATION_OBJECTPATH, handler).await
}

pub async fn start_at<H: NotificationHandler + 'static + Sync + Send + Clone>(
    sub_bus: &str,
    handler: H,
// FIXME: add proper server error type
) -> Result<(), Box<dyn Error + Send>> {
    let server_state = NotificationServer::with_handler(handler);
    let bus = NotificationBus::custom(sub_bus).ok_or("invalid subpath")?;

    log::info!(
        "instantiated server ({NOTIFICATION_DEFAULT_BUS}) at {:?}",
        bus
    );
    // let stopped = std::sync::Arc::downgrade(&server_state.stop_rx);
    let stopped = server_state.stop_event.listen();

    let interface_name = <NotificationServer<H> as zbus::Interface>::name();
    log::info!("launching session at {sub_bus:?}\n {NOTIFICATION_DEFAULT_BUS:?} \n {interface_name:?}\n {bus:?}");
    let _connection = zbus::ConnectionBuilder::session()?
        .name(bus.clone().into_name())?
        .serve_at(NOTIFICATION_OBJECTPATH, server_state)?
        .build()
        .await?;
    log::info!("launched");

    stopped.await;
    log::info!("shutting down");

    Ok(())
}

/// Starts the server
pub fn start_blocking<H: NotificationHandler + 'static + Sync + Send + Clone>(
    handler: H,
) -> Result<(), Box<dyn Error + Send>> {
    log::info!("start blocking");
    zbus::block_on(start(handler))
}

pub fn stop(sub_bus: &str) -> crate::error::Result<()> {
    let bus = NotificationBus::custom(sub_bus).ok_or("invalid subpath")?;
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
