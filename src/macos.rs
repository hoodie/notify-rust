pub use crate::{
    error::*,
    notification::Notification,
};

use std::ops::{Deref, DerefMut};

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct NotificationHandle {
    notification: Notification,
}

impl NotificationHandle {
    #[allow(missing_docs)]
    pub fn new(notification: Notification) -> NotificationHandle {
        NotificationHandle { notification }
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
    mac_notification_sys::send_notification(
        &notification.summary,                                // title
        &notification.subtitle.as_ref().map(AsRef::as_ref),   // subtitle
        &notification.body,                                   // message
        &notification.sound_name.as_ref().map(AsRef::as_ref), // sound
    )?;

    Ok(NotificationHandle::new(notification.clone()))
}

pub(crate) fn schedule_notification(notification: &Notification, delivery_date: f64) -> Result<NotificationHandle> {
    mac_notification_sys::schedule_notification(
        &notification.summary,                                // title
        &notification.subtitle.as_ref().map(AsRef::as_ref),   // subtitle
        &notification.body,                                   // message
        &notification.sound_name.as_ref().map(AsRef::as_ref), // sound
        delivery_date,
    )?;

    Ok(NotificationHandle::new(notification.clone()))
}
