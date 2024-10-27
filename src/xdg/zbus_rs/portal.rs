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
    priority: Priority, // low, normal, high, urgent
}

impl From<&Notification> for PortalNotification {
    fn from(notification: &Notification) -> Self {
        let urgency = notification
            .hints
            .iter()
            .find(|h| matches!(h, Hint::Urgency(_)));

        eprintln!("urgency: {:?}", urgency);

        let priority = if let Some(Hint::Urgency(urgency)) = urgency {
            Priority::from(*urgency)
        } else {
            Priority::Normal
        };

        eprintln!("priority: {:?}", priority);
        Self {
            title: notification.summary.clone(),
            body: notification.body.clone(),
            // markup_body: None,
            priority,
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
