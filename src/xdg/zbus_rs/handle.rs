use crate::{
    error::*,
    notification::Notification,
    xdg::{
        self, ActionResponseHandler, NOTIFICATION_PORTAL_BUS_NAME, NOTIFICATION_PORTAL_INTERFACE,
        NOTIFICATION_PORTAL_OBJECTPATH,
    },
    ActionResponse, CloseReason,
};

use super::{
    send_notification_via_connection, wait_for_action_signal, wait_for_action_signal_portal,
};

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct ZbusNotificationHandle {
    pub(crate) id: u32,
    // pub(crate) id: NotificationId,
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

    pub async fn wait_for_action(self, invocation_closure: impl ActionResponseHandler) {
        wait_for_action_signal(&self.connection, self.id, invocation_closure).await;
    }

    pub async fn close_fallible(self) -> Result<()> {
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

    pub async fn close(self) {
        self.close_fallible().await.unwrap();
    }

    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(CloseReason),
    {
        zbus::block_on(self.wait_for_action(|action: &ActionResponse| {
            if let ActionResponse::Closed(reason) = action {
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

    pub fn update(&mut self) {
        self.update_fallible().unwrap();
    }
}

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct PortalNotificationHandle {
    pub(crate) id: String,
    pub(crate) connection: zbus::Connection,
    // pub(crate) notification: PortalNotification,
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

    pub async fn wait_for_action(self, invocation_closure: impl ActionResponseHandler) {
        wait_for_action_signal_portal(&self.connection, &self.id, invocation_closure).await;
        // todo!()
    }

    pub async fn close_fallible(self) -> Result<()> {
        self.connection
            .call_method(
                // Some(self.notification.bus.clone().into_name()),
                NOTIFICATION_PORTAL_BUS_NAME.into(),
                NOTIFICATION_PORTAL_OBJECTPATH,
                NOTIFICATION_PORTAL_INTERFACE.into(),
                "CloseNotification",
                &(self.id),
            )
            .await?;
        Ok(())
    }

    pub async fn close(self) {
        self.close_fallible().await.unwrap();
    }

    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(CloseReason),
    {
        zbus::block_on(self.wait_for_action(|action: &ActionResponse| {
            if let ActionResponse::Closed(reason) = action {
                closure(*reason);
            }
        }));
    }

    pub fn update_fallible(&mut self) -> Result<()> {
        todo!()
    }

    pub fn update(&mut self) {
        self.update_fallible().unwrap();
    }
}
