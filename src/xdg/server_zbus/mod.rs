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

use super::NotificationObjectPath;

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

impl ReceivedNotification {
    pub fn channels(
        &self,
    ) -> Option<(
        async_std::channel::Sender<String>,
        async_std::channel::Sender<CloseReason>,
    )> {
        Option::zip(self.action_tx.upgrade(), self.close_tx.upgrade())
    }
}

#[async_trait::async_trait]
pub trait NotificationHandler {
    async fn call(&self, notification: ReceivedNotification);
}

#[async_trait::async_trait]
impl<F, Fut> NotificationHandler for F
where
    F: Send + Sync + 'static + Fn(ReceivedNotification) -> Fut,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    async fn call(&self, notification: ReceivedNotification) {
        self(notification).await;
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
pub async fn start<H: NotificationHandler + 'static + Sync + Send + Clone>(
    handler: H,
) -> Result<(), Box<dyn Error>> {
    start_at(NOTIFICATION_OBJECTPATH, handler).await
}
pub async fn start_at<H: NotificationHandler + 'static + Sync + Send + Clone>(
    sub_path: &str,
    handler: H,
) -> Result<(), Box<dyn Error>> {
    let server_state = NotificationServer::with_handler(handler);
    let path = NotificationObjectPath::custom(sub_path).ok_or("invalid subpath")?;

    log::info!("instantiated server ({NOTIFICATION_NAMESPACE}) at {:?}", path);
    let stopped = server_state.stop_rx.clone();


    zbus::ConnectionBuilder::session()?
        .name(NOTIFICATION_NAMESPACE)?
        .serve_at(&path, server_state)?
        .build()
        .await?;
    log::info!("launch session\n {:?}\n {:?}", NOTIFICATION_NAMESPACE, path);

    stopped.recv().await?;
    log::info!("shutting down");

    Ok(())
}

/// Starts the server
// TODO: #[deprecated(note = "blocking can be enable from the outside")]
pub fn start_blocking<H: NotificationHandler + 'static + Sync + Send + Clone>(
    handler: H,
) -> Result<(), Box<dyn Error>> {
    log::info!("start blocking");
    zbus::block_on(start(handler))
}
