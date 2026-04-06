//! No-op stubs for XDG-only API surfaces.
//!
//! When the `noops` feature is enabled, this module provides do-nothing implementations
//! of methods and types that only exist on XDG (Linux/BSD) platforms. This lets
//! cross-platform code call `.hint()`, `.urgency()`, `.on_close()`, etc. without
//! sprinkling `#[cfg]` guards everywhere.
//!
//! # Example
//!
//! Without `noops`, this code does not compile on macOS or Windows:
//!
//! ```ignore
//! Notification::new()
//!     .summary("hello")
//!     .hint(Hint::Transient(true))   // ← XDG only
//!     .show()
//!     .unwrap()
//!     .on_close(|reason| println!("{reason:?}"));  // ← XDG only
//! ```
//!
//! With the `noops` feature enabled it compiles everywhere — the extra calls simply
//! do nothing on non-XDG platforms.

use crate::{notification::Notification, Hint, Urgency};

// ─── Types ───────────────────────────────────────────────────────────────────

/// Why a notification was closed.
///
/// On XDG this carries real information from the notification server.
/// On other platforms this is always [`CloseReason::Other`] because there is
/// no notification server to query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseReason {
    /// The notification expired.
    Expired,
    /// The user dismissed it.
    Dismissed,
    /// The application closed it via `close()`.
    CloseAction,
    /// Unknown reason.
    Other(u32),
}

/// Callback trait for [`NotificationHandle::on_close`].
///
/// Implemented automatically for `Fn()` and `Fn(CloseReason)`.
pub trait CloseHandler<T> {
    /// Called when the notification closes.
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

// ─── Notification builder stubs ───────────────────────────────────────────────

impl Notification {
    /// No-op on this platform.
    ///
    /// On XDG this adds a [`Hint`] to the notification. Enabled by the `noops` feature
    /// so cross-platform code compiles without `#[cfg]` guards.
    #[cfg(any(target_os = "macos", target_os = "windows"))]
    pub fn hint(&mut self, _hint: Hint) -> &mut Notification {
        self
    }

    /// No-op on this platform.
    ///
    /// On XDG and Windows this sets the notification urgency. On macOS urgency is not
    /// supported, so this is a no-op enabled by the `noops` feature.
    #[cfg(target_os = "macos")]
    pub fn urgency(&mut self, _urgency: Urgency) -> &mut Notification {
        self
    }
}

// ─── NotificationHandle stubs (macOS) ────────────────────────────────────────
//
// On macOS `show()` returns `Result<macos::NotificationHandle>`, so we can add
// methods to that type. On Windows `show()` returns `Result<()>` — there is no
// handle to extend.

#[cfg(target_os = "macos")]
mod macos_handle {
    use super::CloseHandler;
    use crate::macos::NotificationHandle;

    impl NotificationHandle {
        /// No-op on this platform.
        ///
        /// On XDG this blocks until the user acts on the notification, then calls
        /// `invocation_closure` with the action identifier.
        pub fn wait_for_action<F>(self, _invocation_closure: F)
        where
            F: FnOnce(&str),
        {
        }

        /// No-op on this platform.
        ///
        /// On XDG this sends a `CloseNotification` D-Bus message.
        pub fn close(self) {}

        /// No-op on this platform.
        ///
        /// On XDG this blocks until the notification closes, then calls `handler`.
        pub fn on_close<A>(&self, _handler: impl CloseHandler<A>) {}

        /// No-op on this platform.
        ///
        /// On XDG this re-sends the notification with updated fields.
        pub fn update(&mut self) {}

        /// Returns `0` on this platform.
        ///
        /// On XDG this returns the numeric notification ID assigned by the server.
        pub fn id(&self) -> u32 {
            0
        }
    }
}

// ─── Free-function stubs ──────────────────────────────────────────────────────

/// No-op stub for [`get_capabilities`](crate::get_capabilities).
///
/// Always returns an empty list. On XDG the real implementation queries the
/// running notification server.
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub fn get_capabilities() -> crate::error::Result<Vec<String>> {
    Ok(Vec::new())
}

/// No-op stub for [`get_server_information`](crate::get_server_information).
///
/// Returns a [`ServerInformation`] with all fields set to `"unknown"`. On XDG
/// the real implementation queries the running notification server.
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub fn get_server_information() -> crate::error::Result<ServerInformation> {
    Ok(ServerInformation {
        name: "unknown".to_owned(),
        vendor: "unknown".to_owned(),
        version: "unknown".to_owned(),
        spec_version: "unknown".to_owned(),
    })
}

/// Stub return value for [`get_server_information`].
#[cfg(any(target_os = "macos", target_os = "windows"))]
#[derive(Debug)]
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
