/// Legacy `NSUserNotificationCenter` backend.
#[cfg(not(feature = "preview-macos-un"))]
mod nsusernotification;

#[cfg(not(feature = "preview-macos-un"))]
pub(crate) use nsusernotification::{schedule_notification, show_notification};

#[cfg(not(feature = "preview-macos-un"))]
pub use nsusernotification::{ApplicationError, MacOsError, NotificationError, NotificationHandle};

#[cfg(not(feature = "preview-macos-un"))]
pub use mac_notification_sys::{get_bundle_identifier_or_default, set_application};

// ── UNUserNotificationCenter preview path ───────────────────────────────────────

/// `UNUserNotificationCenter` backend (opt-in via `preview-macos-un`).
#[cfg(feature = "preview-macos-un")]
mod usernotifications;

#[cfg(feature = "preview-macos-un")]
pub(crate) use usernotifications::{
    schedule_notification, show_notification, show_notification_async,
};

#[cfg(feature = "preview-macos-un")]
pub use usernotifications::{MacOsError, NotificationHandle};

#[cfg(feature = "preview-macos-un")]
pub use mac_usernotifications::blocking::{
    get_notification_settings as get_notification_settings_blocking,
    request_auth as request_auth_blocking,
};
#[cfg(feature = "preview-macos-un")]
pub use mac_usernotifications::{check_bundle, get_notification_settings, request_auth};
