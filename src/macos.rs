#![allow(missing_docs)]
//! macOS notification back-ends.
//!
//! Two implementations are available, selected at compile time.
//! The `UNUserNotificationCenter` path is the default; enable `macos_legacy` to use
//! the old `NSUserNotificationCenter` path instead.
//!
//! | Feature | `macos_legacy` (`NSUserNotificationCenter`) | default (`UNUserNotificationCenter`) |
//! |---|---|---|
//! | Crate | `mac-notification-sys` | `mac-usernotifications` |
//! | macOS requirement | Any supported macOS | macOS 10.14+ |
//! | Bundle ID required | No | Yes (ad-hoc signature is enough) |
//! | `show()` resolves | Immediately | Once delivered |
//! | `response().await` | No | Yes — suspends until interaction |
//! | `response_blocking()` | No | Yes |
//! | `update()` | No | Yes (re-sends by UUID) |
//! | `update_async()` | No | Yes |
//! | `close()` | No | No (not yet) |
//! | `id()` on handle | No | No (not yet) |
//! | Reply actions | No | Yes |
//! | Action buttons | No | Yes (multiple) |
//! | Timeout support | No | Yes |
//! | Authorization request | No | Yes (`request_auth`) |

/// Items that belong exclusively to the legacy `NSUserNotificationCenter` path.
///
/// Enable the `macos_legacy` feature to activate this module.
#[cfg(feature = "macos_legacy")]
mod nsusernotification;

#[cfg(feature = "macos_legacy")]
pub use mac_notification_sys::{get_bundle_identifier_or_default, set_application};

#[cfg(feature = "macos_legacy")]
pub(crate) use nsusernotification::{schedule_notification, show_notification};

#[cfg(feature = "macos_legacy")]
pub use nsusernotification::{ApplicationError, MacOsError, NotificationError, NotificationHandle};

#[cfg(not(feature = "macos_legacy"))]
mod usernotifications;

#[cfg(not(feature = "macos_legacy"))]
pub(crate) use usernotifications::{
    schedule_notification, show_notification, show_notification_async,
};

#[cfg(not(feature = "macos_legacy"))]
pub use usernotifications::{MacOsError, NotificationHandle};

#[cfg(not(feature = "macos_legacy"))]
pub use mac_usernotifications::{check_bundle, request_auth, request_auth_blocking};
