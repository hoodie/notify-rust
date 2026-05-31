use crate::{error::*, notification::Notification};
use std::ops::{Deref, DerefMut};

pub use mac_notification_sys::error::{ApplicationError, Error as MacOsError, NotificationError};

/// A handle to a shown notification (`NSUserNotificationCenter` path).
///
/// This stack is deprecated on macOS 14+. Enable the `preview-macos-un`
/// feature to use the modern `UNUserNotificationCenter` path instead.
#[derive(Debug)]
pub struct NotificationHandle {
    notification: Notification,
}

impl NotificationHandle {
    /// Construct a handle wrapping the given notification.
    pub fn new(notification: Notification) -> Self {
        Self { notification }
    }

    /// Wait for the user to interact with the notification.
    ///
    /// The closure receives the action identifier as a `&str`.
    /// The special value `"__closed"` is passed when the notification is
    /// dismissed without activating any action.
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        let mut n = build_mac_notification(&self.notification);
        n.wait_for_click(true);
        match n
            .send()
            .unwrap_or(mac_notification_sys::NotificationResponse::None)
        {
            mac_notification_sys::NotificationResponse::ActionButton(ref label) => {
                invocation_closure(label);
            }
            mac_notification_sys::NotificationResponse::Click => invocation_closure("default"),
            mac_notification_sys::NotificationResponse::Reply(ref text) => {
                invocation_closure(text);
            }
            mac_notification_sys::NotificationResponse::CloseButton(_)
            | mac_notification_sys::NotificationResponse::None => invocation_closure("__closed"),
        }
    }
}

impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        &self.notification
    }
}

impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.notification
    }
}

fn build_mac_notification(notification: &Notification) -> mac_notification_sys::Notification<'_> {
    let mut n = mac_notification_sys::Notification::default();
    n.title(notification.summary.as_str())
        .message(&notification.body)
        .maybe_subtitle(notification.subtitle.as_deref())
        .maybe_sound(notification.sound_name.as_deref());

    if let Some(ref image_path) = notification.path_to_image {
        n.content_image(image_path);
    }
    n
}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    let n = build_mac_notification(notification);
    n.send()?;
    Ok(NotificationHandle::new(notification.clone()))
}

pub(crate) fn schedule_notification(
    notification: &Notification,
    delivery_date: f64,
) -> Result<NotificationHandle> {
    let mut n = build_mac_notification(notification);
    n.delivery_date(delivery_date);
    n.send()?;
    Ok(NotificationHandle::new(notification.clone()))
}
