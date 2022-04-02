use crate::{error::*, notification::Notification, xdg};
use zbus::blocking::Connection;

use super::{ActionResponse, ActionResponseHandler, CloseReason, NOTIFICATION_NAMESPACE, NOTIFICATION_OBJECTPATH};

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
        log::trace!("wait_for_action");
        wait_for_action_signal(&self.connection, self.id, invocation_closure);
    }

    pub fn close(self) {
        log::trace!("close id {}", self.id);
        self.connection
            .call_method(
                Some(NOTIFICATION_NAMESPACE),
                NOTIFICATION_OBJECTPATH,
                Some(NOTIFICATION_NAMESPACE),
                "CloseNotification",
                &(self.id),
            )
            .unwrap();
    }

    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(CloseReason),
    {
        log::trace!("on_close");
        self.wait_for_action(|action: &ActionResponse| {
            if let ActionResponse::Closed(reason) = action {
                closure(*reason);
            }
        });
    }

    pub fn update(&mut self) {
        log::trace!("update id {}", self.id);
        self.id = send_notification_via_connection(&self.notification, self.id, &self.connection).unwrap();
    }
}

pub fn send_notification_via_connection(notification: &Notification, id: u32, connection: &Connection) -> Result<u32> {
    log::trace!("send_notification_via_connection");
    if let Some(ref close_handler) = notification.close_handler {
        // close_handler.
        let connection = connection.clone();
        async_std::task::spawn(async move {
            wait_for_action_signal(&connection, id, |response: &ActionResponse<'_>| log::trace!("{:?}", response))
        });
    }
    let reply: u32 = connection
        .call_method(
            Some(NOTIFICATION_NAMESPACE),
            NOTIFICATION_OBJECTPATH,
            Some(NOTIFICATION_NAMESPACE),
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
        .body()?;
    Ok(reply)
}

pub fn connect_and_send_notification(notification: &Notification) -> Result<ZbusNotificationHandle> {
    log::trace!("connect_and_send_notification");
    let connection = zbus::blocking::Connection::session()?;
    let inner_id = notification.id.unwrap_or(0);
    let id = send_notification_via_connection(notification, inner_id, &connection)?;
    Ok(ZbusNotificationHandle::new(id, connection, notification.clone()))
}

pub fn get_capabilities() -> Result<Vec<String>> {
    let connection = zbus::blocking::Connection::session()?;
    let info: Vec<String> = connection
        .call_method(
            Some(NOTIFICATION_NAMESPACE),
            NOTIFICATION_OBJECTPATH,
            Some(NOTIFICATION_NAMESPACE),
            "GetCapabilities",
            &(),
        )?
        .body()?;

    Ok(info)
}

pub fn get_server_information() -> Result<xdg::ServerInformation> {
    let connection = zbus::blocking::Connection::session()?;
    let info: xdg::ServerInformation = connection
        .call_method(
            Some(NOTIFICATION_NAMESPACE),
            NOTIFICATION_OBJECTPATH,
            Some(NOTIFICATION_NAMESPACE),
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
    log::trace!("handle_action");
    let connection = Connection::session().unwrap();
    wait_for_action_signal(&connection, id, func);
}

fn wait_for_action_signal(connection: &Connection, id: u32, handler: impl ActionResponseHandler) {
    let proxy = zbus::blocking::fdo::DBusProxy::new(connection).unwrap();
    log::trace!("waiting for signals");
    let action_invoked_signal = format!("interface='{}',member='ActionInvoked'", NOTIFICATION_NAMESPACE);
    let notification_closed_signal = format!("interface='{}',member='NotificationClosed'", NOTIFICATION_NAMESPACE);

    proxy.add_match(&action_invoked_signal).unwrap();
    proxy.add_match(&notification_closed_signal).unwrap();
    log::trace!("waiting for signals, proxies registered\n{}\n{}", action_invoked_signal, notification_closed_signal);

    for msg in zbus::blocking::MessageIterator::from(connection).flatten() {
        log::trace!("received message");
        if let Ok(header) = msg.header() {
            log::trace!("messages has a header");
            log::trace!("signal received {:?}", header);
            if let Ok(zbus::MessageType::Signal) = header.message_type() {
                log::trace!("it's a signal message");
                match header.member() {
                    Ok(Some(name)) if name == "ActionInvoked" => match msg.body::<(u32, String)>() {
                        Ok((nid, action)) if nid == id => {
                            log::trace!("ActionInvoked {}", action);
                            handler.call(&ActionResponse::Custom(&action));
                            break;
                        }
                        other => {
                            log::warn!("ActionInvoked failed {:?}", other);
                        }
                    },
                    Ok(Some(name)) if name == "NotificationClosed" => match msg.body::<(u32, u32)>() {
                        Ok((nid, reason)) if nid == id => {
                            let reason: CloseReason = reason.into();
                            log::trace!("Notification Closed {:?}", reason);
                            handler.call(&ActionResponse::Closed(reason));
                            break;
                        }
                        other => {
                            log::warn!("NotificationClosed failed {:?}", other);
                        }
                    },
                    Ok(_) => {
                        log::trace!("received unhandled signal");
                    }
                    Err(error) => {
                        log::trace!("failed to handle message {}", error);
                    }
                }
            }
        }
    }
}

pub fn stop_server() -> Result<()> {
    let connection = zbus::blocking::Connection::session()?;
    connection.call_method(
        Some(NOTIFICATION_NAMESPACE),
        NOTIFICATION_OBJECTPATH,
        Some(NOTIFICATION_NAMESPACE),
        "Stop",
        &(),
    )?;

    Ok(())
}
