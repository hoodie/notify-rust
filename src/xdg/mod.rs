//! This module contains `XDG` and `DBus` specific code.
//!
//! it should not be available under any platform other than `(unix, not(target_os = "macos"))`

#[cfg(feature = "dbus")]
use dbus::ffidisp::Connection as DbusConnection;

#[cfg(feature = "zbus")]
use zbus::{block_on, zvariant};

use crate::{error::*, notification::Notification};

use std::ops::{Deref, DerefMut};

#[cfg(feature = "dbus")]
mod dbus_rs;
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
use dbus_rs::bus;

#[cfg(feature = "zbus")]
mod zbus_rs;
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
use zbus_rs::bus;

#[cfg(all(feature = "dbus", feature = "zbus"))]
mod bus;

// #[cfg(all(feature = "server", feature = "dbus", unix, not(target_os = "macos")))]
// pub mod server_dbus;

#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
pub mod server;

#[cfg(all(feature = "server", feature = "dbus", unix, not(target_os = "macos")))]
pub mod server_dbus;

#[cfg(all(feature = "server", feature = "zbus", unix, not(target_os = "macos")))]
pub mod server_zbus;

#[doc(hidden)]
pub static NOTIFICATION_DEFAULT_BUS: &str = "org.freedesktop.Notifications";

#[doc(hidden)]
pub static NOTIFICATION_INTERFACE: &str = "org.freedesktop.Notifications";

#[doc(hidden)]
pub static NOTIFICATION_OBJECTPATH: &str = "/org/freedesktop/Notifications";

pub(crate) use bus::NotificationBus;

#[derive(Debug)]
enum NotificationHandleInner {
    #[cfg(feature = "dbus")]
    Dbus(dbus_rs::DbusNotificationHandle),

    #[cfg(feature = "zbus")]
    Zbus(zbus_rs::ZbusNotificationHandle),
}

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct NotificationHandle {
    inner: NotificationHandleInner,
}

#[allow(dead_code)]
impl NotificationHandle {
    #[cfg(feature = "dbus")]
    pub(crate) fn for_dbus(
        id: u32,
        connection: DbusConnection,
        notification: Notification,
    ) -> NotificationHandle {
        NotificationHandle {
            inner: dbus_rs::DbusNotificationHandle::new(id, connection, notification).into(),
        }
    }

    #[cfg(feature = "zbus")]
    pub(crate) fn for_zbus(
        id: u32,
        connection: zbus::Connection,
        notification: Notification,
    ) -> NotificationHandle {
        NotificationHandle {
            inner: zbus_rs::ZbusNotificationHandle::new(id, connection, notification).into(),
        }
    }

    /// Waits for the user to act on a notification and then calls
    /// `invocation_closure` with the name of the corresponding action.
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(inner) => {
                inner.wait_for_action(|action: &ActionResponse| match action {
                    ActionResponse::Custom(action) => invocation_closure(action),
                    ActionResponse::Closed(_reason) => invocation_closure("__closed"), // FIXME: remove backward compatibility with 5.0
                });
            }

            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => {
                block_on(
                    inner.wait_for_action(|action: &ActionResponse| match action {
                        ActionResponse::Custom(action) => invocation_closure(action),
                        ActionResponse::Closed(_reason) => invocation_closure("__closed"), // FIXME: remove backward compatibility with 5.0
                    }),
                );
            }
        };
    }

    /// Manually close the notification
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use notify_rust::*;
    /// let handle: NotificationHandle = Notification::new()
    ///     .summary("oh no")
    ///     .hint(notify_rust::Hint::Transient(true))
    ///     .body("I'll be here till you close me!")
    ///     .hint(Hint::Resident(true)) // does not work on kde
    ///     .timeout(Timeout::Never) // works on kde and gnome
    ///     .show()
    ///     .unwrap();
    /// // ... and then later
    /// handle.close();
    /// ```
    pub fn close(self) {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(inner) => inner.close(),
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => block_on(inner.close()),
        }
    }

    /// Executes a closure after the notification has closed.
    ///
    /// ## Example 1: *I don't care about why it closed* (the good ole API)
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// Notification::new().summary("Time is running out")
    ///                    .body("This will go away.")
    ///                    .icon("clock")
    ///                    .show()
    ///                    .unwrap()
    ///                    .on_close(|| println!("closed"));
    /// ```
    ///
    /// ## Example 2: *I **do** care about why it closed* (added in v4.5.0)
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// Notification::new().summary("Time is running out")
    ///                    .body("This will go away.")
    ///                    .icon("clock")
    ///                    .show()
    ///                    .unwrap()
    ///                    .on_close(|reason| println!("closed: {:?}", reason));
    /// ```
    pub fn on_close<A>(self, handler: impl CloseHandler<A>) {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(inner) => {
                inner.wait_for_action(|action: &ActionResponse| {
                    if let ActionResponse::Closed(reason) = action {
                        handler.call(*reason);
                    }
                });
            }
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => {
                block_on(inner.wait_for_action(|action: &ActionResponse| {
                    if let ActionResponse::Closed(reason) = action {
                        handler.call(*reason);
                    }
                }));
            }
        };
    }

    /// Replace the original notification with an updated version
    /// ## Example
    /// ```no_run
    /// # use notify_rust::Notification;
    /// let mut notification = Notification::new().summary("Latest News")
    ///                                           .body("Bayern Dortmund 3:2")
    ///                                           .show()
    ///                                           .unwrap();
    ///
    /// std::thread::sleep_ms(1_500);
    ///
    /// notification.summary("Latest News (Correction)")
    ///             .body("Bayern Dortmund 3:3");
    ///
    /// notification.update();
    /// ```
    /// Watch out for different implementations of the
    /// notification server! On plasma5 for instance, you should also change the appname, so the old
    /// message is really replaced and not just amended. Xfce behaves well, all others have not
    /// been tested by the developer.
    pub fn update(&mut self) {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref mut inner) => inner.update(),
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref mut inner) => inner.update(),
        }
    }

    /// Returns the Handle's id.
    pub fn id(&self) -> u32 {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref inner) => inner.id,
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref inner) => inner.id,
        }
    }
}

/// Required for `DerefMut`
impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref inner) => &inner.notification,
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref inner) => &inner.notification,
        }
    }
}

/// Allow you to easily modify notification properties
impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref mut inner) => &mut inner.notification,
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref mut inner) => &mut inner.notification,
        }
    }
}

#[cfg(feature = "dbus")]
impl From<dbus_rs::DbusNotificationHandle> for NotificationHandleInner {
    fn from(handle: dbus_rs::DbusNotificationHandle) -> NotificationHandleInner {
        NotificationHandleInner::Dbus(handle)
    }
}

#[cfg(feature = "zbus")]
impl From<zbus_rs::ZbusNotificationHandle> for NotificationHandleInner {
    fn from(handle: zbus_rs::ZbusNotificationHandle) -> NotificationHandleInner {
        NotificationHandleInner::Zbus(handle)
    }
}

#[cfg(feature = "dbus")]
impl From<dbus_rs::DbusNotificationHandle> for NotificationHandle {
    fn from(handle: dbus_rs::DbusNotificationHandle) -> NotificationHandle {
        NotificationHandle {
            inner: handle.into(),
        }
    }
}

#[cfg(feature = "zbus")]
impl From<zbus_rs::ZbusNotificationHandle> for NotificationHandle {
    fn from(handle: zbus_rs::ZbusNotificationHandle) -> NotificationHandle {
        NotificationHandle {
            inner: handle.into(),
        }
    }
}

// here be public functions

// TODO: breaking change, wait for 5.0
// #[cfg(all(feature = "dbus", feature = "zbus"))]
//compile_error!("the z and d features are mutually exclusive");

#[cfg(all(
    not(any(feature = "dbus", feature = "zbus")),
    unix,
    not(target_os = "macos")
))]
compile_error!("you have to build with either zbus or dbus turned on");

/// Which Dbus implementation are we using?
#[derive(Copy, Clone, Debug)]
pub enum DbusStack {
    /// using [dbus-rs](https://docs.rs/dbus-rs)
    Dbus,
    /// using [zbus](https://docs.rs/zbus)
    Zbus,
}

#[cfg(all(feature = "dbus", feature = "zbus"))]
const DBUS_SWITCH_VAR: &str = "DBUSRS";

#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    log::trace!("showing notification...");
    let handle = block_on(zbus_rs::connect_and_send_notification(notification)).map(Into::into);
    log::trace!("showing notification done");
    handle
}

#[cfg(all(feature = "async", feature = "zbus"))]
pub(crate) async fn show_notification_async(
    notification: &Notification,
) -> Result<NotificationHandle> {
    zbus_rs::connect_and_send_notification(notification)
        .await
        .map(Into::into)
}

#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    dbus_rs::connect_and_send_notification(notification).map(Into::into)
}

#[cfg(all(feature = "dbus", feature = "zbus"))]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        dbus_rs::connect_and_send_notification(notification).map(Into::into)
    } else {
        block_on(zbus_rs::connect_and_send_notification(notification)).map(Into::into)
    }
}

/// Get the currently used [`DbusStack`]
///
/// (zbus only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub fn dbus_stack() -> Option<DbusStack> {
    Some(DbusStack::Zbus)
}

/// Get the currently used [`DbusStack`]
///
/// (dbus-rs only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub fn dbus_stack() -> Option<DbusStack> {
    Some(DbusStack::Dbus)
}

/// Get the currently used [`DbusStack`]
///
/// both dbus-rs and zbus, switch via `$ZBUS_NOTIFICATION`
#[cfg(all(feature = "dbus", feature = "zbus"))]
pub fn dbus_stack() -> Option<DbusStack> {
    Some(if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        DbusStack::Dbus
    } else {
        DbusStack::Zbus
    })
}

/// Get the currently used [`DbusStack`]
///
/// neither zbus nor dbus-rs are configured
#[cfg(all(not(feature = "dbus"), not(feature = "zbus")))]
pub fn dbus_stack() -> Option<DbusStack> {
    None
}

/// Get list of all capabilities of the running notification server.
///
/// (zbus only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub fn get_capabilities() -> Result<Vec<String>> {
    block_on(zbus_rs::get_capabilities())
}

/// versions of functions that target a specific bus;
#[cfg(all(feature = "zbus", feature = "config_bus", not(feature = "dbus")))]
pub mod at_bus {
    use super::*;
    /// Returns a struct containing `ServerInformation`.
    ///
    /// This struct contains `name`, `vendor`, `version` and `spec_version` of the notification server
    /// running.
    ///
    /// (zbus only)
    #[cfg(all(feature = "zbus", not(feature = "dbus")))]
    pub fn get_server_information(sub_bus: &str) -> Result<ServerInformation> {
        let bus = NotificationBus::custom(sub_bus).ok_or("invalid subpath")?;
        block_on(zbus_rs::get_server_information_at_bus(bus))
    }

    /// Get list of all capabilities of the running notification server.
    ///
    /// (zbus only)
    #[cfg(all(feature = "zbus", not(feature = "dbus")))]
    pub fn get_capabilities(sub_bus: &str) -> Result<Vec<String>> {
        let bus = NotificationBus::custom(sub_bus).ok_or("invalid subpath")?;
        block_on(zbus_rs::get_capabilities_at_bus(bus))
    }

    #[cfg(all(feature = "async", feature = "zbus"))]
    pub(crate) async fn show_notification_async(
        notification: &Notification,
        bus: NotificationBus,
    ) -> Result<NotificationHandle> {
        zbus_rs::connect_and_send_notification_at_bus(notification, bus)
            .await
            .map(Into::into)
    }
}

/// Get list of all capabilities of the running notification server.
///
/// (dbus-rs only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub fn get_capabilities() -> Result<Vec<String>> {
    dbus_rs::get_capabilities()
}

/// Get list of all capabilities of the running notification server.
///
/// both dbus-rs and zbus, switch via `$ZBUS_NOTIFICATION`
#[cfg(all(feature = "dbus", feature = "zbus"))]
pub fn get_capabilities() -> Result<Vec<String>> {
    if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        dbus_rs::get_capabilities()
    } else {
        block_on(zbus_rs::get_capabilities())
    }
}

/// Returns a struct containing `ServerInformation`.
///
/// This struct contains `name`, `vendor`, `version` and `spec_version` of the notification server
/// running.
///
/// (zbus only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub fn get_server_information() -> Result<ServerInformation> {
    block_on(zbus_rs::get_server_information())
}

/// Returns a struct containing `ServerInformation`.
///
/// This struct contains `name`, `vendor`, `version` and `spec_version` of the notification server
/// running.
///
/// (dbus-rs only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub fn get_server_information() -> Result<ServerInformation> {
    dbus_rs::get_server_information()
}

/// Returns a struct containing `ServerInformation`.
///
/// This struct contains `name`, `vendor`, `version` and `spec_version` of the notification server
/// running.
///
/// both dbus-rs and zbus, switch via `$ZBUS_NOTIFICATION`
#[cfg(all(feature = "dbus", feature = "zbus"))]
pub fn get_server_information() -> Result<ServerInformation> {
    if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        dbus_rs::get_server_information()
    } else {
        block_on(zbus_rs::get_server_information())
    }
}

/// Return value of `get_server_information()`.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "zbus", derive(zvariant::Type))]
pub struct ServerInformation {
    /// The product name of the server.
    pub name: String,
    /// The vendor name.
    pub vendor: String,
    /// The server's version string.
    pub version: String,
    /// The specification version the server is compliant with.
    pub spec_version: String,
}

/// Strictly internal.
/// The NotificationServer implemented here exposes a "Stop" function.
/// stops the notification server
#[cfg(all(feature = "server", feature = "dbus", unix, not(target_os = "macos")))]
#[doc(hidden)]
pub fn stop_server() {
    dbus_rs::stop_server()
}

/// Strictly internal.
/// The NotificationServer implemented here exposes a "Stop" function.
/// stops the notification server
#[cfg(all(feature = "server", feature = "zbus", unix, not(target_os = "macos")))]
#[doc(hidden)]
pub fn stop_server() -> Result<()> {
    server_zbus::stop(Default::default())
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out [`NotificationHandle::wait_for_action`]
/// (xdg only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
// #[deprecated(note="please use [`NotificationHandle::wait_for_action`]")]
pub fn handle_action<F>(id: u32, func: F)
where
    F: FnOnce(&ActionResponse),
{
    block_on(zbus_rs::handle_action(id, func));
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out [`NotificationHandle::wait_for_action`]
/// (xdg only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
// #[deprecated(note="please use `NotificationHandle::wait_for_action`")]
pub fn handle_action<F>(id: u32, func: F)
where
    F: FnOnce(&ActionResponse),
{
    dbus_rs::handle_action(id, func);
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out [`NotificationHandle::wait_for_action`]
/// both dbus-rs and zbus, switch via `$ZBUS_NOTIFICATION`
#[cfg(all(feature = "dbus", feature = "zbus"))]
// #[deprecated(note="please use `NotificationHandle::wait_for_action`")]
pub fn handle_action<F>(id: u32, func: F)
where
    F: FnOnce(&ActionResponse),
{
    if std::env::var(DBUS_SWITCH_VAR).is_ok() {
        dbus_rs::handle_action(id, func);
    } else {
        block_on(zbus_rs::handle_action(id, func));
    }
}

/// Reason passed to `NotificationClosed` Signal
///
/// ## Specification
/// As listed under [Table 8. `NotificationClosed` Parameters](https://specifications.freedesktop.org/notification-spec/latest/ar01s09.html#idm46350804042704)
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "zbus", derive(serde::Serialize, serde::Deserialize))]
pub enum CloseReason {
    /// Undefined/Reserved reason
    Other(u32),
    /// The notification expired
    Expired,
    /// The notification was dismissed by the user
    Dismissed,
    /// The notification was closed by a call to `CloseNotification`
    CloseAction,
}

#[cfg(feature = "zbus")]
impl zvariant::Type for CloseReason {
    fn signature() -> zvariant::Signature<'static> {
        // "uu" -> "uuu" 🤷‍♂️
        zvariant::Signature::try_from("u").unwrap()
    }
}

impl From<CloseReason> for u32 {
    fn from(val: CloseReason) -> Self {
        match dbg!(val) {
            CloseReason::Expired => 1,
            CloseReason::Dismissed => 2,
            CloseReason::CloseAction => 3,
            CloseReason::Other(other) => other,
        }
    }
}

impl From<u32> for CloseReason {
    fn from(raw_reason: u32) -> Self {
        match raw_reason {
            1 => CloseReason::Expired,
            2 => CloseReason::Dismissed,
            3 => CloseReason::CloseAction,
            // _ => unreachable!()
            other => CloseReason::Other(other),
        }
    }
}

/// Helper Trait implemented by `Fn()`
pub trait ActionResponseHandler {
    fn call(self, response: &ActionResponse);
}

// impl<F: Send + Sync + 'static> ActionResponseHandler for F
impl<F> ActionResponseHandler for F
where
    F: FnOnce(&ActionResponse),
{
    fn call(self, res: &ActionResponse) {
        (self)(res);
    }
}

/// Response to an action
#[derive(Debug)]
pub enum ActionResponse<'a> {
    /// Custom Action configured by the Notification.
    Custom(&'a str),

    /// The Notification was closed.
    Closed(CloseReason),
}

impl<'a> From<&'a str> for ActionResponse<'a> {
    fn from(raw: &'a str) -> Self {
        Self::Custom(raw)
    }
}

/// Your handy callback for the `Close` signal of your Notification.
///
/// This is implemented by `Fn()` and `Fn(CloseReason)`, so there is probably no good reason for you to manually implement this trait.
/// Should you find one anyway, please notify me and I'll gladly remove this obviously redundant comment.
pub trait CloseHandler<T> {
    /// This is called with the [`CloseReason`].
    fn call(&self, reason: CloseReason);
}

impl<F> CloseHandler<CloseReason> for F
where
    F: Fn(CloseReason),
{
    fn call(&self, reason: CloseReason) {
        self(reason);
    }
}

impl<F> CloseHandler<()> for F
where
    F: Fn(),
{
    fn call(&self, _: CloseReason) {
        self();
    }
}
