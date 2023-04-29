use std::sync::Arc;

use crate::{error::*, notification::Notification, xdg};
use async_lock::Mutex;
use zbus::{export::futures_util::TryStreamExt, MatchRule};

use super::{bus::NotificationBus, ActionResponse, ActionResponseHandler, CloseReason};

pub use self::handle::ZbusNotificationHandle;

pub mod bus {

    use zbus::names::BusName;

    use crate::{
        error::{ErrorKind, Result},
        xdg::NOTIFICATION_DEFAULT_BUS,
    };

    fn skip_first_slash(s: &str) -> &str {
        if let Some('/') = s.chars().next() {
            &s[1..]
        } else {
            s
        }
    }

    use std::path::PathBuf;

    type BusNameType = zbus::names::WellKnownName<'static>;

    #[derive(Clone, Debug)]
    pub struct NotificationBus(BusNameType);

    impl Default for NotificationBus {
        #[cfg(feature = "zbus")]
        fn default() -> Self {
            Self(zbus::names::WellKnownName::from_static_str(NOTIFICATION_DEFAULT_BUS).unwrap())
        }
    }

    impl NotificationBus {
        fn namespaced_custom(custom_path: &str) -> Result<String> {
            // abusing path for semantic join
            PathBuf::from("/de/hoodie/Notification")
                .join(custom_path)
                .to_str()
                .map(skip_first_slash)
                .map(|path| path.replace('/', "."))
                .ok_or_else(|| ErrorKind::InvalidBusName(custom_path.into()).into())
        }

        pub fn custom(custom_path: &str) -> Result<Self> {
            let inner = Self::namespaced_custom(custom_path)?;
            let name = zbus::names::WellKnownName::try_from(inner)?;
            Ok(Self(name))
        }

        pub fn to_name(&self) -> BusNameType {
            self.0.clone()
        }

        pub fn into_name(self) -> BusNameType {
            self.0
        }
    }
    impl From<NotificationBus> for BusName<'static> {
        fn from(value: NotificationBus) -> Self {
            value.into_name().into()
        }
    }
}
async fn send_notification_via_connection(
    notification: &Notification,
    id: u32,
    connection: &zbus::Connection,
) -> Result<u32> {
    send_notification_via_connection_at_bus(notification, id, connection, &Default::default()).await
}

async fn send_notification_via_connection_at_bus(
    notification: &Notification,
    id: u32,
    connection: &zbus::Connection,
    bus: &NotificationBus,
) -> Result<u32> {
    // if let Some(ref close_handler) = notification.close_handler {
    //     // close_handler.
    //     let connection = connection.clone();
    //     async_std::task::spawn(async move {
    //         wait_for_action_signal(&connection, id, |response: &ActionResponse<'_>| log::trace!("{:?}", response))
    //     });
    // }
    let reply: u32 = connection
        .call_method(
            Some(bus.to_name()),
            xdg::NOTIFICATION_OBJECTPATH,
            Some(xdg::NOTIFICATION_INTERFACE),
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
        .deserialize()?;
    log::trace!("reply received");
    Ok(reply)
}

pub async fn connect_and_send_notification(
    notification: &Notification,
) -> Result<ZbusNotificationHandle> {
    let bus = notification.bus.clone();
    log::trace!("connecting at {bus:?}");
    connect_and_send_notification_at_bus(notification, bus).await
}

pub(crate) async fn connect_and_send_notification_at_bus(
    notification: &Notification,
    bus: NotificationBus,
) -> Result<ZbusNotificationHandle> {
    let connection = zbus::Connection::session().await?;
    let inner_id = notification.id.unwrap_or(0);
    let id =
        send_notification_via_connection_at_bus(notification, inner_id, &connection, &bus).await?;

    let closed: Arc<Mutex<Option<CloseReason>>> = Default::default();
    let wait_for_close = {
        // locks `closed` until the closed signal is received
        let connection = connection.clone();
        let closed = closed.clone();
        let bus = bus.clone();
        async move {
            log::trace!("waiting for Notification #{id} to close");
            let mut closed = closed.lock().await;
            let reason = handle::await_close_signal(&connection, id, &bus).await;
            if let Some(reason) = reason {
                closed.replace(reason);
                log::trace!("Notification #{id} closed, writing reason: {reason:?}");
            } else {
                log::warn!("awaited close reason resulted in None");
            }
        }
    };
    async_std::task::spawn(wait_for_close);

    Ok(ZbusNotificationHandle {
        id,
        connection,
        notification: notification.clone(),
        closed,
    })
}

pub async fn get_capabilities_at_bus(bus: NotificationBus) -> Result<Vec<String>> {
    let connection = zbus::Connection::session().await?;
    let info: Vec<String> = connection
        .call_method(
            Some(bus.into_name()),
            xdg::NOTIFICATION_OBJECTPATH,
            Some(xdg::NOTIFICATION_INTERFACE),
            "GetCapabilities",
            &(),
        )
        .await?
        .body()
        .deserialize()?;
    Ok(info)
}

pub async fn get_capabilities() -> Result<Vec<String>> {
    get_capabilities_at_bus(Default::default()).await
}

/// Returns a struct containing `ServerInformation`.
///
/// This struct contains `name`, `vendor`, `version` and `spec_version` of the notification server
/// running.
///
/// (zbus only)
pub async fn get_server_information_at_bus(bus: NotificationBus) -> Result<xdg::ServerInformation> {
    let connection = zbus::Connection::session().await?;
    let info: xdg::ServerInformation = connection
        .call_method(
            Some(bus.into_name()),
            xdg::NOTIFICATION_OBJECTPATH,
            Some(xdg::NOTIFICATION_INTERFACE),
            "GetServerInformation",
            &(),
        )
        .await?
        .body()
        .deserialize()?;

    Ok(info)
}

pub async fn get_server_information() -> Result<xdg::ServerInformation> {
    get_server_information_at_bus(Default::default()).await
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
pub async fn handle_action(id: u32, func: impl ActionResponseHandler) {
    log::trace!("handle_action");
    let connection = zbus::Connection::session().await.unwrap();
    handle::wait_for_action_signal(&connection, id, &Default::default(), func).await;
}

mod handle {
    use super::*;

    /// A handle to a shown notification.
    ///
    /// This keeps a connection alive to ensure actions work on certain desktops.
    #[derive(Clone, Debug)]
    pub struct ZbusNotificationHandle {
        pub(crate) id: u32,
        pub(crate) connection: zbus::Connection,
        pub(crate) notification: Notification,
        pub(crate) closed: Arc<Mutex<Option<CloseReason>>>,
    }

    impl ZbusNotificationHandle {
        // TODO: soft deprecate: actually add Vec of Handlers
        pub async fn wait_for_action(&self, handler: impl ActionResponseHandler) {
            log::trace!("wait_for_action...");
            wait_for_action_signal(&self.connection, self.id, &self.notification.bus, handler)
                .await;
            log::trace!("wait_for_action. done");
        }

        pub async fn close_fallible(self) -> Result<()> {
            log::trace!("close id {}", self.id);
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

        pub async fn closed(&self) {
            self.closed.lock().await;
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

        #[deprecated(note = "this will be renamed into update in 5.0")]
        pub fn update_fallible(&mut self) -> Result<()> {
            self.id = zbus::block_on(send_notification_via_connection(
                &self.notification,
                self.id,
                &self.connection,
            ))?;
            Ok(())
        }

        pub fn update(&mut self) {
            #[allow(deprecated)]
            self.update_fallible().unwrap();
        }
    }

    // TODO: this also waits for close-signals, maybe we want to separate these tasks
    pub(super) async fn wait_for_action_signal(
        connection: &zbus::Connection,
        id: u32,
        bus: &NotificationBus,
        handler: impl ActionResponseHandler,
    ) {
        log::trace!("wait for action on #{id} on bus {bus:?}");
        let action_signal_rule = MatchRule::builder()
            .msg_type(zbus::MessageType::Signal)
            .sender(bus.clone())
            .unwrap()
            .interface(xdg::NOTIFICATION_INTERFACE)
            .unwrap()
            .member("ActionInvoked")
            .unwrap()
            .build();

        let proxy = zbus::fdo::DBusProxy::new(connection).await.unwrap();
        proxy.add_match_rule(action_signal_rule).await.unwrap();

        let close_signal_rule = MatchRule::builder()
            .msg_type(zbus::MessageType::Signal)
            .sender(bus.clone())
            .unwrap()
            .interface(xdg::NOTIFICATION_INTERFACE)
            .unwrap()
            .member("NotificationClosed")
            .unwrap()
            .build();
        proxy.add_match_rule(close_signal_rule).await.unwrap();

        while let Ok(Some(msg)) = zbus::MessageStream::from(connection).try_next().await {
            let header = msg.header();
            if let zbus::MessageType::Signal = header.message_type() {
                log::trace!("it's a signal message");

                match header.member() {
                    Some(name) if name == "ActionInvoked" => {
                        match msg.body().deserialize::<(u32, String)>() {
                            Ok((nid, action)) if nid == id => {
                                log::trace!("ActionInvoked {}", action);
                                handler.call(&ActionResponse::Custom(&action));
                                break;
                            }
                            other => {
                                log::warn!("ActionInvoked failed {:?}", other);
                            }
                        }
                    }
                    Some(name) if name == "NotificationClosed" => {
                        match msg.body().deserialize::<(u32, u32)>() {
                            Ok((nid, reason)) if nid == id => {
                                let reason: CloseReason = reason.into();
                                log::trace!("NotificationClosed {:?}", reason);
                                handler.call(&ActionResponse::Closed(reason));
                                break;
                            }
                            other => {
                                log::warn!("NotificationClosed failed {:?}", other);
                            }
                        }
                    }
                    _ => {
                        log::warn!("received unhandled signal");
                    }
                }
            }
        }
    }

    pub(super) async fn await_close_signal(
        connection: &zbus::Connection,
        id: u32,
        bus: &NotificationBus,
    ) -> Option<CloseReason> {
        log::trace!("wait for close signal on #{id} on bus {bus:?}");

        let close_signal_rule = MatchRule::builder()
            .msg_type(zbus::MessageType::Signal)
            .sender(bus.to_owned())
            .unwrap()
            .interface(xdg::NOTIFICATION_INTERFACE)
            .unwrap()
            .member("NotificationClosed")
            .unwrap()
            .build();

        let proxy = zbus::fdo::DBusProxy::new(connection).await.unwrap();
        proxy.add_match_rule(close_signal_rule).await.unwrap();

        while let Ok(Some(msg)) = zbus::MessageStream::from(connection).try_next().await {
            let header = msg.header();
            if let zbus::MessageType::Signal = header.message_type() {
                match header.member() {
                    Some(name) if name == "NotificationClosed" => {
                        return match msg.body().deserialize::<(u32, u32)>() {
                            Ok((nid, reason)) if nid == id => {
                                let reason: CloseReason = reason.into();
                                log::debug!("Notification Closed {:?}, await done", reason);
                                Some(reason)
                            }
                            other => {
                                log::warn!("NotificationClosed failed {:?}", other);
                                None
                            }
                        }
                    }
                    _ => {
                        log::trace!("received unhandled signal");
                    }
                };
            } else {
                log::warn!("signal not ok");
                return None;
            }
        }
        None
    }
}
