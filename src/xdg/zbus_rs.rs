use crate::{error::*, notification::Notification, xdg};
use zbus::Connection;

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

    pub fn wait_for_action<F>(mut self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        // todo!("no action handling yet")
        wait_for_action_signal(&mut self.connection, self.id, invocation_closure);
    }

    pub fn close(self) {
        self.connection.call_method(
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            crate::xdg::NOTIFICATION_OBJECTPATH,
            Some(crate::xdg::NOTIFICATION_NAMESPACE),
            "CloseNotification",
            &(self.id),
        ).unwrap();
    }

    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(),
    {
        self.wait_for_action(|action| {
            if action == "__closed" {
                closure();
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
                crate::hints::hints_to_map(&notification.hints),
                notification.timeout.into_i32(),
            ),
        )?
        .body()
        .unwrap();
    Ok(dbg!(reply))
}

pub fn connect_and_send_notification(notification: &Notification) -> Result<ZbusNotificationHandle> {
    let connection = zbus::Connection::new_session()?;
    let inner_id = notification.id.unwrap_or(0);
    let id = send_notificaion_via_connection(notification, inner_id, &connection)?;
    Ok(ZbusNotificationHandle::new(id, connection, notification.clone()))
}

pub fn get_capabilities() -> Result<Vec<String>> {
    let connection = zbus::Connection::new_session()?;
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
    let connection = zbus::Connection::new_session()?;
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
pub fn handle_action<F>(id: u32, func: F)
where
    F: FnOnce(&str),
{
    let mut connection = Connection::new_session().unwrap();
    wait_for_action_signal(&mut connection, id, func);
}


fn wait_for_action_signal<F>(connection: &mut Connection, id: u32, func: F)
where
    F: FnOnce(&str),
{
    let proxy = zbus::fdo::DBusProxy::new(connection).unwrap();
    proxy
        .add_match("interface='org.freedesktop.Notifications',member='ActionInvoked'")
        .unwrap();
    proxy
        .add_match("interface='org.freedesktop.Notifications',member='NotificationClosed'")
        .unwrap();

    while let Ok(msg) = connection.receive_message() {
        if let Ok(header) = msg.header() {
            if let Ok(zbus::MessageType::Signal) = header.message_type() {
                match header.member() {
                    Ok(Some("ActionInvoked")) => match msg.body::<(u32, String)>() {
                        Ok((nid, action)) if nid == id => {
                            func(&action);
                            break;
                        }
                        _ => {}
                    },
                    Ok(Some("NotificationClosed")) => match msg.body::<(u32, u32)>() {
                        Ok((nid, _)) if nid == id => break,
                        _ => {}
                    },

                    Ok(_) => {}
                    Err(_) => {}
                }
            }
        }
    }
}
