use crate::{error::*, notification::Notification};

pub use mac_notification_sys::error::{ApplicationError, Error as MacOsError, NotificationError};

use std::ops::{Deref, DerefMut};

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct NotificationHandle {
    notification: Notification,
    response: NotificationRespone
}

impl NotificationHandle {
    #[allow(missing_docs)]
    pub fn new(notification: Notification) -> NotificationHandle {
        NotificationHandle { notification }
    }

    /// Waits for the user to act on a notification and then calls
    /// `invocation_closure` with the name of the corresponding action.
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        // wait_for_action_signal(&self.connection, self.id, invocation_closure).await;
        // self.
    }
}

impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        &self.notification
    }
}

/// Allow to easily modify notification properties
impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.notification
    }
}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    let response = mac_notification_sys::Notification::default()
        .title(notification.summary.as_str())
        .message(&notification.body)
        .maybe_subtitle(notification.subtitle.as_deref())
        .maybe_sound(notification.sound_name.as_deref())
        .send()?;

    Ok(NotificationHandle::new(notification.clone(), response))
}

pub(crate) fn schedule_notification(
    notification: &Notification,
    delivery_date: f64,
) -> Result<NotificationHandle> {
    mac_notification_sys::Notification::default()
        .title(notification.summary.as_str())
        .message(&notification.body)
        .maybe_subtitle(notification.subtitle.as_deref())
        .maybe_sound(notification.sound_name.as_deref())
        .delivery_date(delivery_date)
        .send()?;

    Ok(NotificationHandle::new(notification.clone()))
}
