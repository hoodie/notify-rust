//! This module contains XDG and DBus specific code.
//!
//! it should not be available under any platform other than `(unix, not(target_os = "macos"))`

#[cfg(feature = "dbus")]
use dbus::ffidisp::Connection as DbusConnection;

use crate::{error::*, notification::Notification};

use std::ops::{Deref, DerefMut};

#[cfg(feature = "dbus")]
mod dbus_rs;
#[cfg(feature = "zbus")]
mod zbus_rs;

#[cfg(not(feature = "debug_namespace"))]
pub static NOTIFICATION_NAMESPACE: &str = "org.freedesktop.Notifications";
#[cfg(not(feature = "debug_namespace"))]
pub static NOTIFICATION_OBJECTPATH: &str = "/org/freedesktop/Notifications";

#[cfg(feature = "debug_namespace")]
pub static NOTIFICATION_NAMESPACE: &str = "de.hoodie.Notifications";
#[cfg(feature = "debug_namespace")]
pub static NOTIFICATION_OBJECTPATH: &str = "/de/hoodie/Notifications";

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
    pub(crate) fn for_dbus(id: u32, connection: DbusConnection, notification: Notification) -> NotificationHandle {
        NotificationHandle {
            inner: dbus_rs::DbusNotificationHandle::new(id, connection, notification).into(),
        }
    }

    #[cfg(feature = "zbus")]
    pub(crate) fn for_zbus(id: u32, connection: zbus::Connection, notification: Notification) -> NotificationHandle {
        NotificationHandle {
            inner: zbus_rs::ZbusNotificationHandle::new(id, connection, notification).into(),
        }
    }

    /// Waits for the user to act on a notification and then calls
    /// `invocation_closure` with the name of the corresponding action.
    // pub fn wait_for_action(self, invocation_closure: impl ActionResponseHandler) {
    pub fn wait_for_action<F>(mut self, invocation_closure: F) -> Self
    where
        F: FnOnce(&str),
    {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref mut inner) => inner.wait_for_action(|action: &ActionResponse| {
                if let ActionResponse::Custom(action) = action {
                    invocation_closure(action);
                }
            }),

            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref mut inner) => inner.wait_for_action(|action: &ActionResponse| {
                if let ActionResponse::Custom(action) = action {
                    invocation_closure(action);
                }
            }),
        };
        self
    }

    /// Manually close the notification
    ///
    /// # Example
    /// see
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
            NotificationHandleInner::Zbus(inner) => inner.close(),
        }
    }

    /// Executes a closure after the notification has closed.
    /// ## Example
    /// ```no_run
    /// # use notify_rust::Notification;
    /// Notification::new().summary("Time is running out")
    ///                    .body("This will go away.")
    ///                    .icon("clock")
    ///                    .show()
    ///                    .unwrap()
    ///                    .on_close(|| println!("closed"));
    /// ```
    pub fn on_close<A>(mut self, handler: impl CloseHandler<A>) -> Self {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(ref mut inner) => inner.wait_for_action(|action: &ActionResponse| {
                if let ActionResponse::Closed(reason) = action {
                    handler.handle(*reason);
                }
            }),
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(ref mut inner) => inner.wait_for_action(|action: &ActionResponse| {
                if let ActionResponse::Closed(reason) = action {
                    handler.handle(*reason);
                }
            }),
        };
        self
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
        NotificationHandle { inner: handle.into() }
    }
}

#[cfg(feature = "zbus")]
impl From<zbus_rs::ZbusNotificationHandle> for NotificationHandle {
    fn from(handle: zbus_rs::ZbusNotificationHandle) -> NotificationHandle {
        NotificationHandle { inner: handle.into() }
    }
}

// here be public functions

#[cfg(all(not(any(feature = "dbus", feature = "zbus")), unix, not(target_os = "macos")))]
compile_error!("you have to build with eiter zbus or dbus turned on");

/// Which Dbus implementation are we using?
#[derive(Copy, Clone, Debug)]
pub enum DbusStack {
    /// using [dbus-rs](https://docs.rs/dbus-rs)
    Dbus,
    /// using [zbus](https://docs.rs/zbus)
    Zbus,
}

#[cfg(all(feature = "dbus", feature = "zbus"))]
const ZBUS_SWITCH_VAR: &str = "ZBUS_NOTIFICATION";

#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    zbus_rs::connect_and_send_notification(notification).map(Into::into)
}

#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    dbus_rs::connect_and_send_notification(notification).map(Into::into)
}

#[cfg(all(feature = "dbus", feature = "zbus"))]
pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    if std::env::var(ZBUS_SWITCH_VAR).is_ok() {
        zbus_rs::connect_and_send_notification(notification).map(Into::into)
    } else {
        dbus_rs::connect_and_send_notification(notification).map(Into::into)
    }
}

/// Get the currently dsed [`DbusStack`]
///
/// (zbus only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub fn dbus_stack() -> Option<DbusStack> {
    Some(DbusStack::Zbus)
}

/// Get the currently dsed [`DbusStack`]
///
/// (dbus-rs only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub fn dbus_stack() -> Option<DbusStack> {
    Some(DbusStack::Dbus)
}

/// Get the currently dsed [`DbusStack`]
///
/// both dbus-rs and zbus, switch via `$ZBUS_NOTIFICATION`
#[cfg(all(feature = "dbus", feature = "zbus"))]
pub fn dbus_stack() -> Option<DbusStack> {
    Some(if std::env::var(ZBUS_SWITCH_VAR).is_ok() {
        DbusStack::Zbus
    } else {
        DbusStack::Dbus
    })
}

/// Get the currently dsed [`DbusStack`]
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
    zbus_rs::get_capabilities()
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
    if std::env::var(ZBUS_SWITCH_VAR).is_ok() {
        zbus_rs::get_capabilities()
    } else {
        dbus_rs::get_capabilities()
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
    zbus_rs::get_server_information()
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
    if std::env::var(ZBUS_SWITCH_VAR).is_ok() {
        zbus_rs::get_server_information()
    } else {
        dbus_rs::get_server_information()
    }
}

/// Return value of `get_server_information()`.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "zbus", derive(zvariant_derive::Type))]
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
#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
#[doc(hidden)]
pub fn stop_server() {
    #[cfg(feature = "dbus")]
    dbus_rs::stop_server()
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
/// (zbus only)
#[cfg(all(feature = "zbus", not(feature = "dbus")))]
pub fn handle_action(id: u32, handler: impl ActionResponseHandler) {
    zbus_rs::handle_action(id, handler)
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
/// (dbus-rs only)
#[cfg(all(feature = "dbus", not(feature = "zbus")))]
pub fn handle_action(id: u32, handler: impl ActionResponseHandler) {
    dbus_rs::handle_action(id, handler)
}

/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
/// both dbus-rs and zbus, switch via `$ZBUS_NOTIFICATION`
#[cfg(all(feature = "dbus", feature = "zbus"))]
pub fn handle_action<F>(id: u32, func: F)
where
    F: FnOnce(&str),
{
    if std::env::var(ZBUS_SWITCH_VAR).is_ok() {
        zbus_rs::handle_action(id, func)
    } else {
        dbus_rs::handle_action(id, func)
    }
}

/// Reased passed to NotificationClosed Signal
///
/// ## Specification
/// As listed under [Table 8. NotificationClosed Parameters]https://specifications.freedesktop.org/notification-spec/latest/ar01s09.html#idm46350804042704)
#[derive(Copy, Clone, Debug)]
pub enum CloseReason {
    /// The notification expired
    Expired,
    /// The notification was dismissed by the user
    Dismissed,
    /// The notification was closed by a call to `CloseNotification`
    CloseAction,
    /// Undefined/Reserved reason
    Other(u32),
}

impl From<u32> for CloseReason {
    fn from(raw_reason: u32) -> Self {
        match raw_reason {
            1 => CloseReason::Expired,
            2 => CloseReason::Dismissed,
            3 => CloseReason::CloseAction,
            other => CloseReason::Other(other),
        }
    }
}

// pub type ActionResponseHandler = dyn (FnOnce(&ActionResponse));

// pub(crate) trait ActionResponseHandler: Send + Sync + 'static {
pub trait ActionResponseHandler {
    fn call(self, response: &ActionResponse);
}

// impl<F: Send + Sync + 'static> ActionResponseHandler for F
impl<F> ActionResponseHandler for F
where
    F: FnOnce(&ActionResponse),
{
    fn call(self, res: &ActionResponse) {
        (self)(res)
    }
}

pub enum ActionResponse<'a> {
    Custom(&'a str),
    Closed(CloseReason),
}

impl<'a> From<&'a str> for ActionResponse<'a> {
    fn from(raw: &'a str) -> Self {
        Self::Custom(raw)
    }
}

pub trait CloseHandler<T> {
    fn handle(&self, reason: CloseReason);
}

impl<F> CloseHandler<CloseReason> for F
where
    F: Fn(CloseReason),
{
    fn handle(&self, reason: CloseReason) {
        self(reason)
    }
}
impl<F> CloseHandler<()> for F
where
    F: Fn(),
{
    fn handle(&self, _: CloseReason) {
        self()
    }
}
