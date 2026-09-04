use crate::{
    error::*,
    notification::Notification,
    xdg::{self, ActionResponseHandler, CloseReason, NotificationResponse},
};

use super::{
    portal::{remove_notification, send_portal_notification_on_connection, update_notification},
    send_notification_via_connection, wait_for_action_signal, wait_for_action_signal_portal,
};

/// A handle to a shown notification (classic D-Bus path).
///
/// Keeps a D-Bus connection alive so that action signals keep working for the
/// lifetime of this handle.
#[derive(Debug)]
pub struct ZbusNotificationHandle {
    pub(crate) id: u32,
    pub(crate) connection: zbus::Connection,
    pub(crate) notification: Notification,
}

impl ZbusNotificationHandle {
    pub(crate) fn new(
        id: u32,
        connection: zbus::Connection,
        notification: Notification,
    ) -> ZbusNotificationHandle {
        ZbusNotificationHandle {
            id,
            connection,
            notification,
        }
    }

    pub async fn wait_for_action(&self, invocation_closure: impl ActionResponseHandler) {
        wait_for_action_signal(&self.connection, self.id, invocation_closure).await;
    }

    pub async fn close_fallible(&self) -> Result<()> {
        self.connection
            .call_method(
                Some(self.notification.bus.clone().into_name()),
                xdg::NOTIFICATION_OBJECTPATH,
                Some(xdg::NOTIFICATION_INTERFACE),
                "CloseNotification",
                &(self.id),
            )
            .await?;
        Ok(())
    }

    pub async fn close(&self) {
        let _ = self.close_fallible().await;
    }

    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(CloseReason),
    {
        zbus::block_on(self.wait_for_action(|action: &NotificationResponse| {
            if let NotificationResponse::Closed(reason) = action {
                closure(*reason);
            }
        }));
    }

    pub fn update_fallible(&mut self) -> Result<()> {
        self.id = zbus::block_on(send_notification_via_connection(
            &self.notification,
            self.id,
            &self.connection,
        ))?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        self.update_fallible()
    }
}

/// A handle to a notification sent via the XDG desktop portal
/// (`org.freedesktop.portal.Notification`).
///
/// Keeps a D-Bus connection alive so that `ActionInvoked` signals keep working
/// for the lifetime of this handle.
#[derive(Debug)]
pub struct PortalNotificationHandle {
    /// The auto-generated notification ID.
    pub(crate) id: String,
    pub(crate) connection: zbus::Connection,
    pub(crate) notification: Notification,
}

impl PortalNotificationHandle {
    pub(crate) fn new(
        id: impl Into<String>,
        connection: zbus::Connection,
        notification: Notification,
    ) -> PortalNotificationHandle {
        PortalNotificationHandle {
            id: id.into(),
            connection,
            notification,
        }
    }

    /// Wait for the user to invoke an action on this notification.
    ///
    /// Blocks asynchronously until the portal emits an `ActionInvoked` signal for this
    /// notification's ID, then calls `invocation_closure` with the action key string.
    ///
    /// Takes `&self` (unlike an earlier version of this method, which consumed `self`) so
    /// the handle can still be used afterwards — typically to call
    /// [`close`](Self::close) once the action has been handled. Per the portal spec,
    /// notifications are **not** automatically removed when an action is invoked; they are
    /// expected to outlast the application and remain visible until explicitly withdrawn via
    /// `RemoveNotification` or dismissed by the user.
    ///
    /// Note: `org.freedesktop.portal.Notification` does **not** define a
    /// `NotificationClosed` signal, so a plain dismissal by the user will never resolve
    /// this future.  Only explicit action button clicks will trigger the closure.
    pub async fn wait_for_action(&self, invocation_closure: impl ActionResponseHandler) {
        wait_for_action_signal_portal(&self.connection, &self.id, invocation_closure).await;
    }

    /// Send `RemoveNotification` to the portal to dismiss this notification.
    ///
    /// Returns an error if the D-Bus call fails.
    pub async fn close_fallible(self) -> Result<()> {
        remove_notification(&self.id, &self.connection).await
    }

    /// Dismiss this notification via the portal.
    ///
    /// Panics if the underlying `RemoveNotification` D-Bus call fails.
    /// Use [`close_fallible`](Self::close_fallible) if you need to handle errors.
    pub async fn close(self) {
        self.close_fallible().await.unwrap();
    }

    /// Execute `closure` when the notification is dismissed by the user.
    ///
    /// Takes `&self` so the handle remains usable afterwards — e.g. to call
    /// [`close`](Self::close) once the closure has run. The portal does **not**
    /// automatically remove a notification just because an action was invoked (see
    /// [`wait_for_action`](Self::wait_for_action) for details); if you want the
    /// notification to disappear after handling an action, close it explicitly.
    ///
    /// Note: `org.freedesktop.portal.Notification` does **not** expose a
    /// `NotificationClosed` signal.  This method blocks waiting for any
    /// `ActionInvoked` signal (which includes the user clicking a button); a
    /// plain dismissal will never resolve it.  Prefer using
    /// `wait_for_action` with an async runtime instead.
    pub fn on_close<F>(&self, closure: F)
    where
        F: FnOnce(CloseReason),
    {
        zbus::block_on(self.wait_for_action(|response: &NotificationResponse| {
            if let NotificationResponse::Closed(reason) = response {
                closure(*reason);
            }
        }));
    }

    /// Re-send the notification with the same ID, replacing the previously
    /// shown notification in-place (as specified by the portal spec).
    ///
    /// This variant updates using the content that was originally sent. To supply
    /// new content (different body, title, etc.) use
    /// [`update_with_fallible`](Self::update_with_fallible) instead.
    pub fn update_fallible(&mut self) -> Result<()> {
        zbus::block_on(update_notification(
            &self.notification,
            &self.id,
            &self.connection,
        ))?;
        Ok(())
    }

    /// Replace this notification in-place using the originally sent content.
    ///
    /// Re-sends `AddNotification` with the same ID, causing the portal to replace the
    /// currently visible notification with the updated content.
    ///
    /// Panics if the D-Bus call fails.  Use [`update_fallible`](Self::update_fallible)
    /// if you need to handle errors.
    ///
    /// To supply new content use [`update_with`](Self::update_with) instead.
    pub fn update(&mut self) {
        self.update_fallible().unwrap();
    }

    /// Replace this notification in-place with new content, reusing the existing
    /// D-Bus connection.
    ///
    /// This is the **correct** way to update a portal notification with changed
    /// content (different body text, title, priority, etc.).  It sends
    /// `AddNotification` with the **same ID** and the **same connection** that was
    /// used to send the original notification.
    ///
    /// # Why the same connection matters
    ///
    /// `xdg-desktop-portal` tracks active notifications by `(app_id, notification_id)`
    /// and removes all notifications for an `app_id` when the originating D-Bus sender
    /// disconnects (`on_peer_disconnect`).  For unsandboxed apps the `app_id` is derived
    /// from the process's systemd unit name, and falls back to `""` when no unit is
    /// found (e.g. processes launched from a terminal).
    ///
    /// If you open a **new** connection for each update the portal sees a new sender,
    /// and when the old connection closes it evicts the first notification from its
    /// active table.  The second `AddNotification` therefore creates a brand-new popup
    /// rather than replacing the existing one.
    ///
    /// By reusing `self.connection` this method guarantees the portal sees the same
    /// sender for the entire lifetime of the notification.
    ///
    /// # Errors
    ///
    /// Returns an error if the D-Bus call fails.
    pub async fn update_with_fallible(
        &mut self,
        notification: crate::portal::Notification,
    ) -> Result<()> {
        send_portal_notification_on_connection(&notification, &self.id, &self.connection).await
    }

    /// Replace this notification in-place with new content.
    ///
    /// Panics if the underlying D-Bus call fails.
    /// Use [`update_with_fallible`](Self::update_with_fallible) to handle errors.
    ///
    /// See [`update_with_fallible`](Self::update_with_fallible) for an explanation of
    /// why this reuses the existing connection rather than opening a new one.
    pub async fn update_with(&mut self, notification: crate::portal::Notification) {
        self.update_with_fallible(notification).await.unwrap();
    }
}
