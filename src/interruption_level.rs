//! Interruption level for macOS UserNotifications.
//!
//! This controls whether the notification breaks through Focus modes.
//! Only available on macOS 12+.

#[cfg(target_os = "macos")]
pub use mac_usernotifications::InterruptionLevel;

#[cfg(not(target_os = "macos"))]
/// Dummy type for non-macOS platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptionLevel {
    /// Adds to the notification list without lighting the screen or playing a sound.
    Passive,
    /// Presents immediately, lights the screen, and can play a sound. The default.
    Active,
    /// Presents immediately and bypasses Focus settings.
    TimeSensitive,
    /// Presents immediately, bypasses mute and Do Not Disturb. Requires a special entitlement.
    Critical,
}
