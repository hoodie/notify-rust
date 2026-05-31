use crate::{
    error::*,
    notification::Notification,
    notification_id::NotificationId,
    response::{CloseHandler, NotificationResponse},
    CloseReason, Timeout, Urgency,
};
use std::{ops::Deref, time::Duration};

pub use mac_usernotifications::Error as MacOsError;

/// A handle to a sent notification (`UNUserNotificationCenter` path).
///
/// [`Notification::show`] returns this handle as soon as macOS accepts the
/// notification request — before the user has interacted. Call
/// [`response().await`](NotificationHandle::response) to wait for the user's
/// response, or drop the handle to stop observing it (the notification stays
/// visible; the response channel is cleaned up).
#[derive(Debug)]
pub struct NotificationHandle {
    notification: Notification,
    inner: mac_usernotifications::NotificationHandle,
}

impl NotificationHandle {
    /// Send `notification` and return a handle for observing the response.
    ///
    /// Provided for source-compatibility with the legacy macOS path.
    /// Prefer [`Notification::show`](crate::Notification::show) for most use-cases.
    ///
    /// # Panics
    ///
    /// Panics if `UNUserNotificationCenter` refuses to deliver the notification
    /// (e.g. no bundle identifier, authorisation not granted).
    pub fn new(notification: Notification) -> Self {
        show_notification_blocking(&notification)
            .expect("UNUserNotificationCenter: failed to deliver notification")
    }

    pub(crate) fn from_parts(
        notification: Notification,
        inner: mac_usernotifications::NotificationHandle,
    ) -> Self {
        Self {
            notification,
            inner,
        }
    }

    /// The notification's request identifier.
    ///
    /// This is the UUID assigned by macOS to the `UNNotificationRequest`.
    pub fn notification_id(&self) -> &str {
        self.inner.notification_id()
    }

    /// Returns the handle's id as a [`NotificationId`].
    pub fn id(&self) -> NotificationId {
        NotificationId::Mac(self.inner.notification_id().to_owned())
    }

    /// Wait for the user's response.
    ///
    /// Returns as soon as the user interacts with the notification or the
    /// timeout elapses.
    pub async fn response(self) -> NotificationResponse {
        match self.inner.response().await {
            Ok(resp) => resp.into(),
            Err(_) => NotificationResponse::Closed(CloseReason::Expired),
        }
    }

    /// Blocking version of [`response`](Self::response).
    pub fn response_blocking(self) -> NotificationResponse {
        match self.inner.response_blocking() {
            Ok(resp) => resp.into(),
            Err(_) => NotificationResponse::Closed(CloseReason::Expired),
        }
    }

    /// Re-send the notification in-place, preserving its identifier.
    ///
    /// Mutate the handle via `DerefMut` first to change title, body, etc.,
    /// then call `update()` to push the changes to Notification Center.
    pub fn update(&mut self) -> Result<()> {
        let nid = self.inner.notification_id().to_owned();
        self.notification.id = Some(NotificationId::Mac(nid));
        show_notification_blocking(&self.notification)?;
        Ok(())
    }

    /// Async version of [`update`](Self::update).
    pub async fn update_async(&mut self) -> Result<()> {
        let nid = self.inner.notification_id().to_owned();
        self.notification.id = Some(NotificationId::Mac(nid));
        show_notification_async(&self.notification).await?;
        Ok(())
    }

    /// Remove the delivered notification from Notification Center.
    pub async fn close(&self) {
        mac_usernotifications::close_delivered(self.inner.notification_id()).await;
    }

    /// Wait for the user to interact and call `invocation_closure` with the action identifier.
    ///
    /// The special value `"__closed"` is passed when the notification is dismissed.
    #[deprecated(
        since = "4.18.0",
        note = "Use `response()` / `response_blocking()` instead"
    )]
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        let response = self.inner.response_blocking().unwrap();
        invocation_closure(&response.action_identifier);
    }

    /// Executes a closure after the notification has closed.
    #[deprecated(
        since = "4.18.0",
        note = "Use `response_blocking()` and match on `NotificationResponse::Closed` instead"
    )]
    pub fn on_close<A>(self, handler: impl CloseHandler<A>) {
        let response = self.inner.response_blocking().unwrap();
        if let Some(close_reason) = response.close_reason {
            handler.call(close_reason.into());
        }
    }
}

impl From<mac_usernotifications::CloseReason> for CloseReason {
    fn from(close_reason: mac_usernotifications::CloseReason) -> Self {
        match close_reason {
            mac_usernotifications::CloseReason::Dismissed => CloseReason::Dismissed,
            mac_usernotifications::CloseReason::Expired => CloseReason::Expired,
        }
    }
}

impl From<mac_usernotifications::NotificationResponse> for NotificationResponse {
    fn from(resp: mac_usernotifications::NotificationResponse) -> Self {
        if resp.is_dismiss_action() {
            NotificationResponse::Closed(CloseReason::Dismissed)
        } else if let Some(ref text) = resp.reply_text {
            NotificationResponse::Reply(text.clone())
        } else {
            NotificationResponse::Action(resp.action_identifier.clone())
        }
    }
}

impl From<Urgency> for mac_usernotifications::InterruptionLevel {
    fn from(val: Urgency) -> Self {
        match val {
            Urgency::Low => mac_usernotifications::InterruptionLevel::Passive,
            Urgency::Normal => mac_usernotifications::InterruptionLevel::Active,
            Urgency::Critical => mac_usernotifications::InterruptionLevel::TimeSensitive,
        }
    }
}

// SAFETY: `mac_usernotifications::NotificationHandle` uses channels internally
// which are `Send + Sync` but not automatically `UnwindSafe`. The handle does
// not expose any raw pointers and panics cannot corrupt its state.
impl std::panic::UnwindSafe for NotificationHandle {}
impl std::panic::RefUnwindSafe for NotificationHandle {}

impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        &self.notification
    }
}

impl std::ops::DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.notification
    }
}

impl From<&Notification> for mac_usernotifications::Notification {
    fn from(n: &Notification) -> Self {
        let mut un = mac_usernotifications::Notification::new()
            .title(&n.summary)
            .message(&n.body)
            .maybe_subtitle(n.subtitle.as_deref())
            .maybe_sound(n.sound_name.as_deref());

        // 4.18: actions is Vec<String> with flat [id, label, id, label, …] pairs
        for chunk in n.actions.chunks(2) {
            if let (Some(id), Some(label)) = (chunk.first(), chunk.get(1)) {
                un = un.action(mac_usernotifications::Action::button(id, label));
            }
        }

        if let Timeout::Milliseconds(ms) = n.timeout {
            un = un.timeout(Duration::from_millis(ms as u64));
        }

        if let Some(ref path) = n.path_to_image {
            un = un.image_path(path);
        }

        if let Some(ref nid) = n.id {
            let id_str = match nid {
                NotificationId::Mac(ref string_id) => string_id.clone(),
                NotificationId::Xdg(num) => num.to_string(),
            };
            un = un.id(&id_str);
        }

        if let Some(level) = n.interruption_level {
            un = un.interruption_level(level);
        }

        un
    }
}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    show_notification_blocking(notification)
}

pub(crate) fn show_notification_blocking(
    notification: &Notification,
) -> Result<NotificationHandle> {
    let un = mac_usernotifications::Notification::from(notification);
    let inner = un.send_blocking()?;
    Ok(NotificationHandle::from_parts(notification.clone(), inner))
}

pub(crate) async fn show_notification_async(
    notification: &Notification,
) -> Result<NotificationHandle> {
    let un = mac_usernotifications::Notification::from(notification);
    let inner = un.send().await?;
    Ok(NotificationHandle::from_parts(notification.clone(), inner))
}

pub(crate) fn schedule_notification(
    notification: &Notification,
    delivery_date: f64,
) -> Result<NotificationHandle> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    let delay = (delivery_date - now).max(0.1);
    let un = mac_usernotifications::Notification::from(notification)
        .schedule_in(Duration::from_secs_f64(delay));
    let inner = un.send_blocking()?;
    Ok(NotificationHandle::from_parts(notification.clone(), inner))
}
