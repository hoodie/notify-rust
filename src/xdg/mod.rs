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
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(inner) => inner.wait_for_action(invocation_closure),

            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => inner.wait_for_action(invocation_closure),
        }
    }

    /// Manually close the notification
    ///
    /// # Example
    /// see
    /// ```no_run
    /// let handle: notify_rust::NotificationHandle = Notification::new()
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
    pub fn on_close<F>(self, closure: F)
    where
        F: FnOnce(),
    {
        match self.inner {
            #[cfg(feature = "dbus")]
            NotificationHandleInner::Dbus(inner) => inner.wait_for_action(|action| {
                if action == "__closed" {
                    closure();
                }
            }),
            #[cfg(feature = "zbus")]
            NotificationHandleInner::Zbus(inner) => inner.wait_for_action(|action| {
                if action == "__closed" {
                    closure();
                }
            }),
        }
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

//#[cfg(all(not(any(feature = "dbus", feature="zbus")), unix, not(target_os = "macos")))]
//compile_error!("you have to build with eiter zbus or dbus turned on");

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    if std::env::var("ZBUS").is_ok() {
        eprintln!("using zbus");
        #[cfg(not(feature = "zbus"))]
        unimplemented!("build with feature=z please!");
        #[cfg(feature = "zbus")]
        zbus_rs::connect_and_send_notification(notification).map(Into::into)
    } else {
        #[cfg(not(any(feature = "dbus", feature = "zbus")))]
        return Err("can't show notification, no dbus connection possible in this build".into());

        #[cfg(feature = "dbus")]
        dbus_rs::connect_and_send_notification(notification).map(Into::into)
    }
}

// here be public functions

/// Get list of all capabilities of the running notification server.
pub fn get_capabilities() -> Result<Vec<String>> {
    #[cfg(not(any(feature = "dbus", feature = "zbus")))]
    return Err("can't get capabilities, no dbus connection possible in this build".into());

    #[cfg(feature = "zbus")]
    todo!("add zbus support for get capabilities");

    #[cfg(feature = "dbus")]
    dbus_rs::get_capabilities()
}

/// Returns a struct containing `ServerInformation`.
///
/// This struct contains `name`, `vendor`, `version` and `spec_version` of the notification server
/// running.
/// TODO dbus stuff module!!!
pub fn get_server_information() -> Result<ServerInformation> {
    if std::env::var("ZBUS").is_ok() {
        eprintln!("using zbus");
        #[cfg(not(feature = "zbus"))]
        unimplemented!("build with feature=z please!");
        #[cfg(feature = "zbus")]
        zbus_rs::get_server_information()
    } else {
        #[cfg(not(any(feature = "dbus", feature = "zbus")))]
        return Err("can't show notification, no dbus connection possible in this build".into());

        #[cfg(feature = "dbus")]
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
pub fn handle_action<F>(id: u32, func: F)
where
    F: FnOnce(&str),
{
    #[cfg(feature = "dbus")]
    dbus_rs::handle_action(id, func)
}
