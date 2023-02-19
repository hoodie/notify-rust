use crate::{error::*, notification::Notification, xdg};
use zbus::{export::futures_util::TryStreamExt, MatchRule};

use super::{ActionResponse, ActionResponseHandler, CloseReason};

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
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

    pub async fn wait_for_action(self, invocation_closure: impl ActionResponseHandler) {
        wait_for_action_signal(&self.connection, self.id, invocation_closure).await;
    }

    pub async fn close_fallible(self) -> Result<()> {
        self.connection
            .call_method(
                Some(crate::xdg::NOTIFICATION_NAMESPACE),
                crate::xdg::NOTIFICATION_OBJECTPATH,
                Some(crate::xdg::NOTIFICATION_NAMESPACE),
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

pub async fn send_notification_via_connection(
    notification: &Notification,
    id: u32,
    connection: &zbus::Connection,
) -> Result<u32> {
    let reply: u32 = connection
        .call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "Notify",
            &(
                &notification.appname,
                id,
                &notification.icon,
                &notification.summary,
                &notification.body,
                &notification.actions,
                crate::hints::hints_to_map(notification),
                i32::from(notification.timeout),
            ),
        )
        .await?
        .body()
        .unwrap();
    Ok(reply)
}

pub async fn connect_and_send_notification(
    notification: &Notification,
) -> Result<ZbusNotificationHandle> {
    let connection = zbus::Connection::session().await?;
    let inner_id = notification.id.unwrap_or(0);
    let id = send_notification_via_connection(notification, inner_id, &connection).await?;
    Ok(ZbusNotificationHandle::new(
        id,
        connection,
        notification.clone(),
    ))
}

pub async fn get_capabilities() -> Result<Vec<String>> {
    log::trace!("get_capabilities()");
    let connection = zbus::Connection::session().await?;
    let info: Vec<String> = connection
        .call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "GetCapabilities",
            &(),
        )
        .await?
        .body()?;

    Ok(info)
}

pub async fn get_server_information() -> Result<xdg::ServerInformation> {
    log::trace!("get_server_information()");
    let connection = zbus::Connection::session().await?;
    let info: xdg::ServerInformation = connection
        .call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "GetServerInformation",
            &(),
        )
        .await?
        .body()?;

    Ok(info)
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
pub async fn handle_action(id: u32, func: impl ActionResponseHandler) {
    let connection = zbus::Connection::session().await.unwrap();
    wait_for_action_signal(&connection, id, func).await;
}

async fn wait_for_action_signal(
    connection: &zbus::Connection,
    id: u32,
    handler: impl ActionResponseHandler,
) {
    let action_signal_rule = MatchRule::builder()
        .msg_type(zbus::MessageType::Signal)
        .interface("org.freedesktop.Notifications")
        .unwrap()
        .member("ActionInvoked")
        .unwrap()
        .build();

    let proxy = zbus::fdo::DBusProxy::new(connection).await.unwrap();
    proxy.add_match_rule(action_signal_rule).await.unwrap();

    let close_signal_rule = MatchRule::builder()
        .msg_type(zbus::MessageType::Signal)
        .interface("org.freedesktop.Notifications")
        .unwrap()
        .member("NotificationClosed")
        .unwrap()
        .build();
    proxy.add_match_rule(close_signal_rule).await.unwrap();

    while let Ok(Some(msg)) = zbus::MessageStream::from(connection).try_next().await {
        if let Ok(header) = msg.header() {
            if let Ok(zbus::MessageType::Signal) = header.message_type() {
                match header.member() {
                    Ok(Some(name)) if name == "ActionInvoked" => {
                        match msg.body::<(u32, String)>() {
                            Ok((nid, action)) if nid == id => {
                                handler.call(&ActionResponse::Custom(&action));
                                break;
                            }
                            _ => {}
                        }
                    }
                    Ok(Some(name)) if name == "NotificationClosed" => {
                        match msg.body::<(u32, u32)>() {
                            Ok((nid, reason)) if nid == id => {
                                handler.call(&ActionResponse::Closed(reason.into()));
                                break;
                            }
                            _ => {}
                        }
                    }
                    Ok(_) | Err(_) => {}
                }
            }
        }
    }
}
