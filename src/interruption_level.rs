//! Interruption level for macOS notifications.
//!
//! This controls whether the notification breaks through Focus modes.
//! Only meaningful on macOS 12+.

/// Interruption level for a notification (macOS 12+).
///
/// Controls whether a notification breaks through Focus modes or silent
/// settings. Only has an effect on macOS — the value is ignored on other
/// platforms.
///
/// Available via the `preview-macos-un` feature on macOS.
#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
pub use mac_usernotifications::InterruptionLevel;

/// Interruption level for a notification (macOS 12+).
///
/// On non-macOS platforms (or macOS without `preview-macos-un`) this type is a
/// no-op placeholder with the same variants for source compatibility.
#[cfg(not(all(target_os = "macos", feature = "preview-macos-un")))]
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
