use crate::{error::*, notification::Notification};

pub use mac_notification_sys::error::{ApplicationError, Error as MacOsError, NotificationError};

use std::ops::{Deref, DerefMut};

use dbus::{blocking::Connection, BusType, Path};
use std::time::Duration;

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

/// Listen to notification
pub(crate) fn listen_notification() -> Result<()> {

    let conn = Connection::new(BusType::Session).unwrap();
    let notifications_path = Path::from("/org/freedesktop/Notifications");

    // Get a proxy for the Notifications interface
    let notifications_proxy = conn.with_proxy(notifications_path, "org.freedesktop.Notifications", Duration::from_secs(5));

    // Register a callback to be called whenever a notification is received
    notifications_proxy
        .method_call("org.freedesktop.Notifications", "Notify", (), Some(&[]))
        .unwrap()
        .match_path(notifications_path)
        .for_each(|msg| {
            println!("Received notification: {:?}", msg);

            // TODO: Do something with the notification

            Ok(())
        })
        .unwrap();

    // Run the event loop
    conn.enter_event_loop();

}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    mac_notification_sys::Notification::default()
        .title(notification.summary.as_str())
        .message(&notification.body)
        .maybe_subtitle(notification.subtitle.as_deref())
        .maybe_sound(notification.sound_name.as_deref())
        .send()?;

    Ok(NotificationHandle::new(notification.clone()))
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
