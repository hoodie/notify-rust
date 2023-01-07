#![allow(missing_docs)]
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
    CloseReason, Hint, ServerInformation, NOTIFICATION_NAMESPACE, NOTIFICATION_OBJECTPATH,
};

#[derive(Debug)]
pub struct Action {
    pub tag: String,
    pub description: String,
}

impl Action {
    fn from_pair(pair: (&String, &String)) -> Action {
        Self {
            tag: pair.0.to_owned(),
            description: pair.1.to_owned(),
        }
    }
    fn from_vec(raw: &[String]) -> Vec<Action> {
        raw.iter()
            .zip(raw.iter().map(Some).chain(Some(None)))
            .filter_map(|(a, b)| b.map(|b| (a, b)))
            .map(Action::from_pair)
            .collect()
    }
}

fn collect_hints(raw_hints: HashMap<String, zbus::zvariant::OwnedValue>) -> Vec<Hint> {
    raw_hints
        .into_iter()
        .filter_map(|(k, v)| match Hint::from_zbus(&k, v.into()) {
            Ok(hint) => Some(hint),
            Err(error) => {
                log::warn!("invalid notification hint {error}");
                None
            }
        })
        .collect()
}

#[derive(Debug)]
pub struct ReceivedNotification {
    pub appname: String,
    pub id: u32,
    pub replace_id: u32,
    pub icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<Action>,
    pub hints: Vec<Hint>,
    pub timeout: i32,
    pub close_tx: async_std::channel::WeakSender<CloseReason>,
    pub action_tx: async_std::channel::WeakSender<String>,
}

pub trait NotificationHandler {
    fn call(&self, notification: ReceivedNotification);
}

impl<F> NotificationHandler for F
where
    F: Fn(ReceivedNotification),
{
    fn call(&self, notification: ReceivedNotification) {
        self(notification);
    }
}

#[derive(Clone, Debug)]
struct NotificationServerConfig {
    default_timeout: u64,
}

impl Default for NotificationServerConfig {
    fn default() -> Self {
        Self {
            default_timeout: 3000,
        }
    }
}

struct NotificationServer<H: NotificationHandler + 'static + Sync + Send + Clone> {
    count: u32,
    config: NotificationServerConfig,
    handler: H,
    stop_rx: async_std::channel::Receiver<()>,
    stop_tx: async_std::channel::Sender<()>,
}

impl<H: NotificationHandler + 'static + Sync + Send + Clone> NotificationServer<H> {
    fn with_handler(handler: H) -> Self {
        let (stop_tx, stop_rx) = async_std::channel::bounded(1);

        NotificationServer {
            count: 0,
            config: NotificationServerConfig::default(),
            handler,
            stop_rx,
            stop_tx,
        }
    }
}

pub fn print_notification(
    ReceivedNotification {
        appname,
        id,
        icon,
        summary,
        body,
        actions,
        hints,
        timeout,
        ..
    }: ReceivedNotification,
) {
    let display = |name, value: &dyn std::fmt::Debug| eprintln!(" [{name:^9}] {value:?}");

    display("name", &appname);
    display("id", &id);
    display("summary", &summary);
    display("body", &body);
    display("actions", &actions);
    display("icon", &icon);
    display("hints", &hints);
    display("timeout", &timeout);
}

// #[dbus_interface]
#[cfg_attr(
    feature = "debug_namespace",
    dbus_interface(name = "de.hoodie.Notifications")
)]
#[cfg_attr(
    not(feature = "debug_namespace"),
    dbus_interface(name = "org.freedesktop.Notifications")
)]
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
        let minimum_timeout = 10; // ms

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

        // phase 1: decide the timeout
        let timeout_before_close = if timeout == 0 {
            None // sleep do not expire at all, user must close
        } else if timeout < 0 {
            Some(Duration::from_millis(self.config.default_timeout))
        } else if (timeout as u64) < minimum_timeout {
            log::warn!("timeout is below minimum timeout of {minimum_timeout}ms -> falling back to minimum timeout");
            Some(Duration::from_millis(minimum_timeout))
        } else {
            Some(Duration::from_millis(timeout as u64))
        };

        // spawning so we can return immediately and respond with the new id
        // while waiting for actions and observing the timeout
        let ctx_in_task = ctx.to_owned();
        async_std::task::spawn(async move {
            // holding on to the reference to keep the channel alive while task is running
            let _close_tx = close_tx.clone();
            let _action_tx = action_tx.clone();

            let sleep = async {
                if let Some(timeout_before_close) = timeout_before_close {
                    async_std::task::sleep(timeout_before_close).await;
                } else {
                    pending::<()>().await;
                }
            };

            // phase 2: wait for user input (action or close) or expiration
            let (reason, action) = select! {
                action = action_rx.recv().fuse() => {
                    log::trace!("close from action");
                    (CloseReason::CloseAction, action.ok())
                }
                reason = close_rx.recv().fuse() => {
                    log::trace!("close from user");
                    (reason.unwrap_or(CloseReason::Expired), None)
                }
                _ = sleep.fuse() => {
                    log::trace!("close from expire after {timeout_before_close:?}", );
                    (CloseReason::Expired, None)
                }
            };

            // phase 3: respond to notification
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
        });

        // phase 0: pass notification and channels to server implementor
        let handler = self.handler.clone();
        async_std::task::spawn(async move {
            handler.call(received);
        });

        Ok(id)
    }

    async fn stop(&self) -> bool {
        // TODO: maybe use ObjectServer::remove instead of sleeping
        log::info!("received stop");

        let cause_stop = self.stop_tx.downgrade();
        async_std::task::spawn(async move {
            async_std::task::sleep(Duration::from_millis(500)).await;
            if let Some(stop_tx) = cause_stop.upgrade() {
                stop_tx.send(()).await.unwrap();
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
async fn start_with<H: NotificationHandler + 'static + Sync + Send + Clone>(
    handler: H,
) -> Result<(), Box<dyn Error>> {
    let server_state = NotificationServer::with_handler(handler);
    log::info!("instantiated server");
    let stopped = server_state.stop_rx.clone();

    zbus::ConnectionBuilder::session()?
        .name(NOTIFICATION_NAMESPACE)?
        .serve_at(NOTIFICATION_OBJECTPATH, server_state)?
        .build()
        .await?;
    log::info!(
        "launch session\n {:?}\n {:?}",
        NOTIFICATION_NAMESPACE,
        NOTIFICATION_OBJECTPATH
    );

    stopped.recv().await?;
    log::info!("shutting down");

    Ok(())
}

/// Starts the server
pub fn blocking_start_with<H: NotificationHandler + 'static + Sync + Send + Clone>(
    handler: H,
) -> Result<(), Box<dyn Error>> {
    log::info!("start blocking");
    zbus::block_on(start_with(handler))
}
