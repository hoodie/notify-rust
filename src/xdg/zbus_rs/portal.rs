use crate::{
    error::*,
    notification::Notification,
    priority::Priority,
    xdg::{
        NOTIFICATION_PORTAL_BUS_NAME, NOTIFICATION_PORTAL_INTERFACE, NOTIFICATION_PORTAL_OBJECTPATH,
    },
    Hint,
};
use zbus::zvariant::{SerializeDict, Type};

#[derive(SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
struct PortalNotification {
    title: String,
    body: String,
    // #[zvariant(rename = "markup-body")]
    // markup_body: Option<String>,
    priority: String, // low, normal, high, urgent
}

impl From<&Notification> for PortalNotification {
    fn from(notification: &Notification) -> Self {
        Self {
            title: notification.summary.clone(),
            body: notification.body.clone(),
            // markup_body: None,
            priority: "urgent".to_string(),
        }
    }
}

async fn add_notification(
    notification: &Notification,
    id: &str,
    connection: &zbus::Connection,
) -> Result<()> {
    connection
        .call_method(
            NOTIFICATION_PORTAL_BUS_NAME.into(),
            NOTIFICATION_PORTAL_OBJECTPATH,
            NOTIFICATION_PORTAL_INTERFACE.into(),
            "AddNotification",
            &(id, PortalNotification::from(notification)),
        )
        .await?;
    Ok(())
}

pub(crate) async fn connect_and_send_notification(
    notification: &Notification,
    id: &str,
    // ) -> Result<ZbusNotificationHandle> {
) -> Result<()> {
    let connection = zbus::Connection::session().await?;
    add_notification(notification, id, &connection).await?;

    Ok(())
    // Ok(ZbusNotificationHandle::new(
    // id,
    // connection,
    // notification.clone(),
    // ))
}
