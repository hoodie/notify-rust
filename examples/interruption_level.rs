//! Demonstrates the `interruption_level` feature on macOS (UserNotifications).
//!
//! This example shows how to use different interruption levels to control
//! whether notifications break through Focus modes on macOS 12+.

mod common;

#[cfg(all(target_os = "macos", not(feature = "macos_legacy")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::{InterruptionLevel, Notification};

    if !common::setup() {
        return Ok(());
    }

    // Passive: adds to notification list without lighting screen or playing sound
    Notification::new()
        .summary("Passive Notification")
        .body("This notification will not break through Focus modes or light the screen.")
        .interruption_level(InterruptionLevel::Passive)
        .show()?;

    // Active: presents immediately, can light screen and play sound (default)
    Notification::new()
        .summary("Active Notification")
        .body("This notification will present immediately with default behavior.")
        .interruption_level(InterruptionLevel::Active)
        .show()?;

    // TimeSensitive: bypasses Focus settings
    Notification::new()
        .summary("Time Sensitive Notification")
        .body("This notification bypasses Focus modes (e.g., for time-critical alerts).")
        .interruption_level(InterruptionLevel::TimeSensitive)
        .show()?;

    // Critical: bypasses mute and Do Not Disturb (requires special entitlement)
    Notification::new()
        .summary("Critical Notification")
        .body("This notification requires a special entitlement and bypasses all restrictions.")
        .interruption_level(InterruptionLevel::Critical)
        .show()?;

    Ok(())
}

#[cfg(all(target_os = "macos", feature = "macos_legacy"))]
fn main() {
    println!("this example requires the default macOS backend (UNUserNotificationCenter)");
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("this is a macOS-only example");
}
