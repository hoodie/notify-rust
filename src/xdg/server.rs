#![allow(missing_docs)]
use std::collections::HashMap;

use crate::{CloseReason, Hint};

#[cfg(feature = "d")]
pub use crate::xdg::server_dbus::stop;

#[cfg(feature = "z")]
pub use crate::xdg::server_zbus::stop;

// ////// Actions and Hints

// TODO: move
#[derive(Debug, PartialEq)]
pub struct Action {
    pub tag: String,
    pub description: String,
}

impl Action {
    pub fn new(description: impl ToString, tag: impl ToString) -> Action {
        Self {
            tag: tag.to_string(),
            description: description.to_string(),
        }
    }

    pub fn from_single(key: impl ToString) -> Action {
        Self {
            tag: key.to_string(),
            description: key.to_string(),
        }
    }

    pub fn from_vec(raw: &[String]) -> Vec<Action> {
        raw.iter()
            .zip(raw.iter().skip(1).map(Some).chain(Some(None)))
            .enumerate()
            //.filter_map(|(i, pair)| (i % 2 == 0).then_some(pair))
            .filter(|(i, _pair)| (i % 2 == 0))
            .filter_map(|(_, (a, b))| b.map(|b| Action::new(a, b)))
            .collect()
    }
}

#[test]
fn test_action_from_vec() {
    let raw = [
        "click for one", // one
        "one",           //
        "click for two",
        "two",
        "click for three",
        "three",
    ]
    .map(|s| s.to_string());
    assert_eq!(
        Action::from_vec(&raw),
        vec![
            Action::new("click for one", "one"),
            Action::new("click for two", "two"),
            Action::new("click for three", "three"),
        ]
    );
}

impl From<&str> for Action {
    fn from(value: &str) -> Self {
        eprintln!("action from str");
        Action {
            tag: value.into(),
            description: value.into(),
        }
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

// // TODO: can there be a `None` CloseReason at all?
// pub type CloseOrDefer  = ControlFlow<Option<CloseReason>, ()>;

// pub type HandlerResult = Result<CloseOrDefer, String>;

pub type HandlerResult = Result<Option<CloseReason>, String>;

#[async_trait::async_trait]
pub trait NotificationHandler<T>
where
    T: Send,
{
    async fn call(&self, notification: ReceivedNotification) -> HandlerResult;
}

#[async_trait::async_trait]
impl<F, Fut> NotificationHandler<()> for F
where
    F: Send + Sync + 'static + Fn(ReceivedNotification) -> Fut,
    Fut: std::future::Future<Output = ()> + Send + 'static,
{
    async fn call(&self, notification: ReceivedNotification) -> HandlerResult {
        self(notification).await;
        // Ok(CloseOrDefer::Break(None))
        Ok(None)
    }
}

#[async_trait::async_trait]
impl<F, Fut> NotificationHandler<HandlerResult> for F
where
    F: Send + Sync + 'static + Fn(ReceivedNotification) -> Fut,
    Fut: std::future::Future<Output = HandlerResult> + Send + 'static,
{
    async fn call(&self, notification: ReceivedNotification) -> HandlerResult {
        self(notification).await
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
