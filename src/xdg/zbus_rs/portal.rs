use std::{collections::HashMap, os::fd::OwnedFd};

use crate::{
    error::*,
    notification::Notification,
    priority::Priority,
    xdg::{
        NOTIFICATION_PORTAL_BUS_NAME, NOTIFICATION_PORTAL_INTERFACE, NOTIFICATION_PORTAL_OBJECTPATH,
    },
    Hint,
};
use zbus::zvariant::{Dict, Fd, SerializeDict, Signature, Type, Value};

pub use super::handle::PortalNotificationHandle as NotificationHandle;

use sound::Sound;
mod sound {
    use super::*;
    #[derive(serde::Serialize, PartialEq, Debug, Type)]
    pub struct Sound(Fd<'static>);
}

use icon::Icon;
mod icon {
    use super::*;
    #[derive(serde::Serialize, PartialEq, Debug)]
    pub enum Icon {
        Themed(Vec<String>),
        File(Fd<'static>),
    }

    impl Into<Dict<'static, 'static>> for Icon {
        fn into(self) -> Dict<'static, 'static> {
            let icon = match self {
                Icon::Themed(names) => ("themed", Value::from(names)),
                Icon::File(fd) => ("file-descriptor", Value::from(fd)),
            };

            let val = Dict::from(HashMap::from([icon]));
            val
        }
    }

    impl From<OwnedFd> for Icon {
        fn from(fd: OwnedFd) -> Self {
            Icon::File(Fd::from(fd))
        }
    }

    impl Icon {
        pub fn themed<I: Into<String>>(names: Vec<I>) -> Self {
            Icon::Themed(names.into_iter().map(Into::into).collect())
        }
    }

    impl Type for Icon {
        fn signature() -> Signature<'static> {
            Signature::from_static_str_unchecked("v")
        }
    }
}

#[derive(Debug, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct PortalNotification {
    title: String,
    body: String,
    #[zvariant(rename = "markup-body")]
    markup_body: Option<String>,
    priority: Priority, // low, normal, high, urgent
    icon: Option<Icon>,
    sound: Option<Sound>,
    // default_action: Option<String>,
    // default_action_target: Option<String>,
    // actions: Option<Vec<String>>,
    // hints: Option<HashMap<String, Value>>,
    // category: Option<String>,
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
            markup_body: None,
            priority,
            icon: None,
            sound: None,
            // default_action: todo!(),
            // default_action_target: todo!(),
            // actions: todo!(),
        }
    }
}

async fn add_notification(
    notification: &Notification,
    id: &str,
    connection: &zbus::Connection,
) -> Result<NotificationHandle> {
    connection
        .call_method(
            NOTIFICATION_PORTAL_BUS_NAME.into(),
            NOTIFICATION_PORTAL_OBJECTPATH,
            NOTIFICATION_PORTAL_INTERFACE.into(),
            "AddNotification",
            &(id, PortalNotification::from(notification)),
        )
        .await?;

    Ok(NotificationHandle::new(
        id,
        connection.clone(),
        notification.clone(),
    ))
}

async fn remove_notification(id: &str, connection: &zbus::Connection) -> Result<()> {
    connection
        .call_method(
            NOTIFICATION_PORTAL_BUS_NAME.into(),
            NOTIFICATION_PORTAL_OBJECTPATH,
            NOTIFICATION_PORTAL_INTERFACE.into(),
            "RemoveNotification",
            &id,
        )
        .await?;

    Ok(())
}

pub(crate) async fn connect_and_send_notification(
    notification: &Notification,
    id: &str,
) -> Result<NotificationHandle> {
    let connection = zbus::Connection::session().await?;
    add_notification(notification, id, &connection).await
}
