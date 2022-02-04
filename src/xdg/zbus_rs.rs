use crate::{error::*, notification::Notification, xdg};
use zbus::blocking::Connection;

use super::{ActionResponse, ActionResponseHandler, CloseReason};

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct ZbusNotificationHandle {
    pub(crate) id: u32,
    pub(crate) connection: Connection,
    pub(crate) notification: Notification,
}

impl ZbusNotificationHandle {
    pub(crate) fn new(id: u32, connection: Connection, notification: Notification) -> ZbusNotificationHandle {
        ZbusNotificationHandle {
            id,
            connection,
            notification,
        }
    }

    pub fn wait_for_action(self, invocation_closure: impl ActionResponseHandler) {
        wait_for_action_signal(&self.connection, self.id, invocation_closure);
    }

    pub fn close(self) {
        self.connection
            .call_method(
                Some(crate::xdg::NOTIFICATION_NAMESPACE),
                crate::xdg::NOTIFICATION_OBJECTPATH,
                Some(crate::xdg::NOTIFICATION_NAMESPACE),
                "CloseNotification",
                &(self.id),
            )
            .unwrap();
    }

    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(CloseReason),
    {
        self.wait_for_action(|action: &ActionResponse| {
            if let ActionResponse::Closed(reason) = action {
                closure(*reason);
            }
        });
    }

    pub fn update(&mut self) {
        self.id = send_notificaion_via_connection(&self.notification, self.id, &self.connection).unwrap();
    }
}

pub fn send_notificaion_via_connection(notification: &Notification, id: u32, connection: &Connection) -> Result<u32> {
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
                notification.timeout.into_i32(),
            ),
        )?
        .body()
        .unwrap();
    Ok(reply)
}

pub fn connect_and_send_notification(notification: &Notification) -> Result<ZbusNotificationHandle> {
    let connection = zbus::blocking::Connection::session()?;
    let inner_id = notification.id.unwrap_or(0);
    let id = send_notificaion_via_connection(notification, inner_id, &connection)?;
    Ok(ZbusNotificationHandle::new(id, connection, notification.clone()))
}

pub fn get_capabilities() -> Result<Vec<String>> {
    let connection = zbus::blocking::Connection::session()?;
    let info: Vec<String> = connection
        .call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "GetCapabilities",
            &(),
        )?
        .body()
        .unwrap();

    Ok(info)
}

pub fn get_server_information() -> Result<xdg::ServerInformation> {
    let connection = zbus::blocking::Connection::session()?;
    let info: xdg::ServerInformation = connection
        .call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "GetServerInformation",
            &(),
        )?
        .body()
        .unwrap();

    Ok(info)
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
pub fn handle_action(id: u32, func: impl ActionResponseHandler) {
    let connection = Connection::session().unwrap();
    wait_for_action_signal(&connection, id, func);
}

fn wait_for_action_signal(connection: &Connection, id: u32, handler: impl ActionResponseHandler) {
    let proxy = zbus::blocking::fdo::DBusProxy::new(connection).unwrap();
    proxy
        .add_match("interface='org.freedesktop.Notifications',member='ActionInvoked'")
        .unwrap();
    proxy
        .add_match("interface='org.freedesktop.Notifications',member='NotificationClosed'")
        .unwrap();

    for msg in zbus::blocking::MessageIterator::from(connection).flatten() {
        if let Ok(header) = msg.header() {
            if let Ok(zbus::MessageType::Signal) = header.message_type() {
                match header.member() {
                    Ok(Some(name)) if name == "ActionInvoked" => match msg.body::<(u32, String)>() {
                        Ok((nid, action)) if nid == id => {
                            handler.call(&ActionResponse::Custom(&action));
                            break;
                        }
                        _ => {}
                    },
                    Ok(Some(name)) if name == "NotificationClosed" => match msg.body::<(u32, u32)>() {
                        Ok((nid, reason)) if nid == id => {
                            handler.call(&ActionResponse::Closed(reason.into()));
                            break;
                        }
                        _ => {}
                    },
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }
}
