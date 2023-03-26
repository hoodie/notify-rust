#![allow(missing_docs)]
use std::collections::HashMap;

use crate::{CloseReason, Hint};

#[cfg(feature = "d")]
pub use crate::xdg::server_dbus::stop;

#[cfg(feature = "z")]
pub use crate::xdg::server_zbus::stop;

// ////// Actions and Hints

// TODO: move 
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
    pub fn from_vec(raw: &[String]) -> Vec<Action> {
        raw.iter()
            .zip(raw.iter().map(Some).chain(Some(None)))
            .filter_map(|(a, b)| b.map(|b| (a, b)))
            .map(Action::from_pair)
            .collect()
    }
}

#[cfg(feature = "zbus")]
pub fn collect_hints(raw_hints: HashMap<String, zbus::zvariant::OwnedValue>) -> Vec<Hint> {
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

// ////// Config

#[derive(Clone, Debug)]
pub struct NotificationServerConfig {
    pub default_timeout: u64,
}

impl Default for NotificationServerConfig {
    fn default() -> Self {
        Self {
            default_timeout: 3000,
        }
    }
}

// ////// Received Notifications

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

// ////// Handler

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

// /////// Util
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
    }: &ReceivedNotification,
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
