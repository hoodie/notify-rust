use crate::{action::UserResponse, error::*, notification::Notification, CloseReason, Timeout};
pub use mac_usernotifications::Error as MacOsError;
use mac_usernotifications::Sound;
use std::{ops::Deref, time::Duration};

/// A handle to a sent notification (`UNUserNotificationCenter` path).
///
/// `show()` returns this handle as soon as macOS accepts the notification
/// request — before the user has interacted.  Call
/// [`response().await`](NotificationHandle::response) to wait for the
/// user's response, or drop the handle to stop observing it (the
/// notification stays visible; the response channel is cleaned up).
#[derive(Debug)]
pub struct NotificationHandle {
    notification: Notification,
    inner: mac_usernotifications::NotificationHandle,
}

impl NotificationHandle {
    pub(crate) fn new(
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
    /// Can be passed to [`mac_usernotifications::close_delivered`] or
    /// [`mac_usernotifications::cancel_pending`].
    pub fn notification_id(&self) -> &str {
        self.inner.notification_id()
    }

    /// Returns the handle's id.
    pub fn id(&self) -> crate::NotificationId {
        crate::NotificationId::Mac(self.inner.notification_id().to_owned())
    }

    /// Wait for the user's response.
    ///
    /// Returns as soon as the user interacts with the notification or the
    /// timeout elapses.  For fire-and-forget notifications (no actions)
    /// this resolves immediately with a dismissed response.
    pub async fn response(self) -> UserResponse {
        match self.inner.response().await {
            Ok(resp) => resp.into(),
            Err(_) => UserResponse::Closed(CloseReason::Expired),
        }
    }

    /// Blocking version of [`response`](Self::response).
    pub fn response_blocking(self) -> UserResponse {
        match self.inner.response_blocking() {
            Ok(resp) => resp.into(),
            Err(_) => UserResponse::Closed(CloseReason::Expired),
        }
    }

    /// Re-send the notification in-place, preserving its id.
    ///
    /// Mutate the handle via `DerefMut` first to change title, body, etc.,
    /// then call `update()` to push the changes to Notification Center.
    pub fn update(&mut self) -> Result<()> {
        let nid = self.inner.notification_id().to_owned();
        self.notification.id = Some(crate::NotificationId::Mac(nid));
        show_notification_blocking(&self.notification)?;
        Ok(())
    }

    /// Async version of [`update`](Self::update).
    pub async fn update_async(&mut self) -> Result<()> {
        let nid = self.inner.notification_id().to_owned();
        self.notification.id = Some(crate::NotificationId::Mac(nid));
        show_notification_async(&self.notification).await?;
        Ok(())
    }

    /// Close the delivered notification.
    ///
    /// Removes the notification from Notification Center.
    pub async fn close(&self) {
        mac_usernotifications::close_delivered(self.inner.notification_id()).await;
    }

    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(CloseReason),
    {
        let response = self.inner.response_blocking().unwrap();
        if let Some(close_reason) = response.close_reason {
            closure(close_reason.into());
        }
    }
}

impl From<mac_usernotifications::CloseReason> for CloseReason {
    fn from(close_reason: mac_usernotifications::CloseReason) -> Self {
        match close_reason {
            mac_usernotifications::CloseReason::Dismissed => CloseReason::Dismissed,
            mac_usernotifications::CloseReason::Expired => CloseReason::Expired,
            // mac_usernotifications::CloseReason::Action => CloseReason::Action,
            // mac_usernotifications::CloseReason::Other(_) => CloseReason::Other,
        }
    }
}

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

/// Maps a `mac_usernotifications::NotificationResponse` to the cross-platform `UserResponse`:
///
/// | `NotificationResponse`        | `UserResponse`                        |
/// | ----------------------------- | ------------------------------------- |
/// | `is_dismiss_action() == true` | `Closed(CloseReason::Dismissed)`      |
/// | `reply_text` is `Some(text)`  | `Reply(text)`                         |
/// | any other action identifier   | `Action(action_identifier)`           |
///
/// The raw action identifier is forwarded as-is (no special casing). A body-tap
/// on macOS comes through as `Action("default")`, mirroring XDG behaviour.
impl From<mac_usernotifications::NotificationResponse> for UserResponse {
    fn from(resp: mac_usernotifications::NotificationResponse) -> Self {
        if resp.is_dismiss_action() {
            UserResponse::Closed(CloseReason::Dismissed)
        } else if let Some(ref text) = resp.reply_text {
            UserResponse::Reply(text.clone())
        } else {
            UserResponse::Action(resp.action_identifier.clone())
        }
    }
}

impl From<&Notification> for mac_usernotifications::Notification {
    fn from(n: &Notification) -> Self {
        let mut un = mac_usernotifications::Notification::new()
            .title(&n.summary)
            .message(&n.body)
            .maybe_subtitle(n.subtitle.as_deref())
            .maybe_sound(n.sound_name.clone().map(Sound::Custom));

        if let Some(ref sound_name) = n.sound_name {
            un = un.sound(sound_name.as_str());
        }
        for action in &n.actions {
            let mac_action = match &action.kind {
                crate::action::ActionKind::Button => {
                    let mut mac = mac_usernotifications::Action::button(&action.id, &action.label);
                    if action.requires_authentication {
                        mac = mac.requires_authentication();
                    }
                    mac
                }
                crate::action::ActionKind::Reply { button_title, placeholder } => {
                    let mut mac = mac_usernotifications::Action::reply(
                        &action.id,
                        &action.label,
                        button_title,
                        placeholder,
                    );
                    if action.requires_authentication {
                        mac = mac.requires_authentication();
                    }
                    mac
                }
            };
            un = un.action(mac_action);
        }
        if let Timeout::Milliseconds(ms) = n.timeout {
            un = un.timeout(Duration::from_millis(ms as u64));
        }
        if let Some(ref path) = n.path_to_image {
            un = un.image_path(path);
        }
        if let Some(ref nid) = n.id {
            let id_str = match nid {
                crate::NotificationId::Mac(ref string_id) => string_id.clone(),
                crate::NotificationId::Xdg(num) => num.to_string(),
            };
            un = un.id(&id_str);
        }
        if let Some(level) = n.interruption_level {
            un = un.interruption_level(level);
        }
        if let Some(ref tid) = n.thread_id {
            un = un.thread_id(tid);
        }
        un
    }
}

pub(crate) fn schedule_notification(
    notification: &Notification,
    delivery_date: f64,
) -> Result<NotificationHandle> {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();
    let delay = (delivery_date - now).max(0.1);
    let un = mac_usernotifications::Notification::from(notification)
        .schedule_in(Duration::from_secs_f64(delay));
    let inner = mac_usernotifications::send_and_wait_for_delivery_blocking(un)?;
    Ok(NotificationHandle::new(notification.clone(), inner))
}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    show_notification_blocking(notification)
}

pub(crate) fn show_notification_blocking(
    notification: &Notification,
) -> Result<NotificationHandle> {
    let un = mac_usernotifications::Notification::from(notification);
    let inner = mac_usernotifications::send_and_wait_for_delivery_blocking(un)?;
    Ok(NotificationHandle::new(notification.clone(), inner))
}

pub(crate) async fn show_notification_async(
    notification: &Notification,
) -> Result<NotificationHandle> {
    let un = mac_usernotifications::Notification::from(notification);
    let inner = mac_usernotifications::send_and_wait_for_delivery(un).await?;
    Ok(NotificationHandle::new(notification.clone(), inner))
}
