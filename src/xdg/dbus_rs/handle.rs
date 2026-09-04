use dbus::ffidisp::Connection;

use crate::{
    error::*,
    notification::Notification,
    xdg::{ActionResponseHandler, CloseReason, NotificationResponse},
};

use super::{build_message, send_notification_via_connection, wait_for_action_signal};

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct DbusNotificationHandle {
    pub(crate) id: u32,
    pub(crate) connection: Connection,
    pub(crate) notification: Notification,
}

impl DbusNotificationHandle {
    pub(crate) fn new(
        id: u32,
        connection: Connection,
        notification: Notification,
    ) -> DbusNotificationHandle {
        DbusNotificationHandle {
            id,
            connection,
            notification,
        }
    }

    pub fn wait_for_action(self, invocation_closure: impl ActionResponseHandler) -> Result<()> {
        wait_for_action_signal(&self.connection, self.id, invocation_closure)
    }

    pub fn close(self) {
        let mut message = build_message("CloseNotification", Default::default());
        message.append_items(&[self.id.into()]);
        let _ = self.connection.send(message); // If closing fails there's nothing we could do anyway
    }

    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(CloseReason),
    {
        let _ = self.wait_for_action(|action: &NotificationResponse| {
            if let NotificationResponse::Closed(reason) = action {
                closure(*reason);
            }
        });
    }

    pub fn update(&mut self) -> Result<()> {
        self.id = send_notification_via_connection(&self.notification, self.id, &self.connection)?;
        Ok(())
    }
}
