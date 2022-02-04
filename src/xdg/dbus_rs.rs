use dbus::{
    arg::messageitem::{MessageItem, MessageItemArray},
    ffidisp::{BusType, Connection, ConnectionItem},
    Message,
};

use super::{ActionResponse, ActionResponseHandler, CloseReason};

use crate::{
    error::*,
    hints::message::HintMessage,
    notification::Notification,
    xdg::{ServerInformation, NOTIFICATION_NAMESPACE, NOTIFICATION_OBJECTPATH},
};

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
    pub(crate) fn new(id: u32, connection: Connection, notification: Notification) -> DbusNotificationHandle {
        DbusNotificationHandle {
            id,
            connection,
            notification,
        }
    }

    pub fn wait_for_action(self, invocation_closure: impl ActionResponseHandler) {
        wait_for_action_signal(&self.connection, self.id, invocation_closure);
    }

    pub fn close(self) {
        let mut message = build_message("CloseNotification");
        message.append_items(&[self.id.into()]);
        let _ = self.connection.send(message); // If closing fails there's nothing we could do anyway
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
    let mut message = build_message("Notify");
    let timeout: i32 = notification.timeout.into();
    message.append_items(&[
        notification.appname.to_owned().into(), // appname
        id.into(),                              // notification to update
        notification.icon.to_owned().into(),    // icon
        notification.summary.to_owned().into(), // summary (title)
        notification.body.to_owned().into(),    // body
        pack_actions(notification),             // actions
        pack_hints(notification)?,              // hints
        timeout.into(),                         // timeout
    ]);

    let reply = connection.send_with_reply_and_block(message, 2000)?;

    match reply.get_items().get(0) {
        Some(&MessageItem::UInt32(ref id)) => Ok(*id),
        _ => Ok(0),
    }
}

pub fn connect_and_send_notification(notification: &Notification) -> Result<DbusNotificationHandle> {
    let connection = Connection::get_private(BusType::Session)?;
    let inner_id = notification.id.unwrap_or(0);
    let id = send_notificaion_via_connection(notification, inner_id, &connection)?;
    Ok(DbusNotificationHandle::new(id, connection, notification.clone()))
}

pub fn build_message(method_name: &str) -> Message {
    Message::new_method_call(
        NOTIFICATION_NAMESPACE,
        NOTIFICATION_OBJECTPATH,
        NOTIFICATION_NAMESPACE,
        method_name,
    )
    .unwrap_or_else(|_| panic!("Error building message call {:?}.", method_name))
}

pub fn pack_hints(notification: &Notification) -> Result<MessageItem> {
    if !notification.hints.is_empty() || !notification.hints_unique.is_empty() {
        let hints = notification
            .get_hints()
            .cloned()
            .map(HintMessage::wrap_hint)
            .collect::<Vec<(MessageItem, MessageItem)>>();

        if let Ok(array) = MessageItem::new_dict(hints) {
            return Ok(array);
        }
    }

    Ok(MessageItem::Array(
        MessageItemArray::new(vec![], "a{sv}".into()).unwrap(),
    ))
}

pub fn pack_actions(notification: &Notification) -> MessageItem {
    if !notification.actions.is_empty() {
        let mut actions = vec![];
        for action in &notification.actions {
            actions.push(action.to_owned().into());
        }
        if let Ok(array) = MessageItem::new_array(actions) {
            return array;
        }
    }

    MessageItem::Array(MessageItemArray::new(vec![], "as".into()).unwrap())
}

pub fn get_capabilities() -> Result<Vec<String>> {
    let mut capabilities = vec![];

    let message = build_message("GetCapabilities");
    let connection = Connection::get_private(BusType::Session)?;
    let reply = connection.send_with_reply_and_block(message, 2000)?;

    if let Some(&MessageItem::Array(ref items)) = reply.get_items().get(0) {
        for item in items.iter() {
            if let MessageItem::Str(ref cap) = *item {
                capabilities.push(cap.clone());
            }
        }
    }

    Ok(capabilities)
}

fn unwrap_message_string(item: Option<&MessageItem>) -> String {
    match item {
        Some(&MessageItem::Str(ref value)) => value.to_owned(),
        _ => "".to_owned(),
    }
}

pub fn get_server_information() -> Result<ServerInformation> {
    let message = build_message("GetServerInformation");
    let connection = Connection::get_private(BusType::Session)?;
    let reply = connection.send_with_reply_and_block(message, 2000)?;

    let items = reply.get_items();

    Ok(ServerInformation {
        name: unwrap_message_string(items.get(0)),
        vendor: unwrap_message_string(items.get(1)),
        version: unwrap_message_string(items.get(2)),
        spec_version: unwrap_message_string(items.get(3)),
    })
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
pub fn handle_action(id: u32, func: impl ActionResponseHandler) {
    let connection = Connection::get_private(BusType::Session).unwrap();
    wait_for_action_signal(&connection, id, func);
}

// Listens for the `ActionInvoked(UInt32, String)` signal.
fn wait_for_action_signal(connection: &Connection, id: u32, handler: impl ActionResponseHandler) {
    connection
        .add_match("interface='org.freedesktop.Notifications',member='ActionInvoked'")
        .unwrap();
    connection
        .add_match("interface='org.freedesktop.Notifications',member='NotificationClosed'")
        .unwrap();

    for item in connection.iter(1000) {
        if let ConnectionItem::Signal(message) = item {
            let items = message.get_items();

            let (path, interface, member) = (
                message
                    .path()
                    .map(|p| p.into_cstring().to_string_lossy().into_owned())
                    .unwrap_or_else(String::new),
                message
                    .interface()
                    .map(|p| p.into_cstring().to_string_lossy().into_owned())
                    .unwrap_or_else(String::new),
                message
                    .member()
                    .map(|p| p.into_cstring().to_string_lossy().into_owned())
                    .unwrap_or_else(String::new),
            );
            match (path.as_ref(), interface.as_ref(), member.as_ref()) {
                // match (protocol.unwrap(), iface.unwrap(), member.unwrap()) {
                // Action Invoked
                ("/org/freedesktop/Notifications", "org.freedesktop.Notifications", "ActionInvoked") => {
                    if let (&MessageItem::UInt32(nid), &MessageItem::Str(ref action)) = (&items[0], &items[1]) {
                        if nid == id {
                            handler.call(&ActionResponse::Custom(action));
                            break;
                        }
                    }
                }

                // Notification Closed
                ("/org/freedesktop/Notifications", "org.freedesktop.Notifications", "NotificationClosed") => {
                    if let (&MessageItem::UInt32(nid), &MessageItem::UInt32(reason)) = (&items[0], &items[1]) {
                        if nid == id {
                            handler.call(&ActionResponse::Closed(reason.into()));
                            break;
                        }
                    }
                }
                (..) => (),
            }
        }
    }
}

/// Strictly internal.
/// The NotificationServer implemented here exposes a "Stop" function.
/// stops the notification server
#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
#[doc(hidden)]
pub fn stop_server() {
    let message = build_message("Stop");
    let connection = Connection::get_private(BusType::Session).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(200));
    connection.send(message).unwrap();
}
