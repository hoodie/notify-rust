//! Demonstrates waiting for a notification to be closed/dismissed.
//!
//! Uses `on_close()` which works on both Linux (XDG/D-Bus) and macOS
//! (UNUserNotificationCenter).  On macOS at least one action button must be
//! present for the notification centre to deliver a dismiss event; without
//! actions the dismiss is swallowed silently.
//!
//! Requires a valid app bundle on macOS:
//!   cargo bundle --example on_close && open target/debug/bundle/osx/*.app

use notify_rust::{Action, Notification};

mod common;

#[cfg(target_os = "windows")]
fn main() {
    log::info!("this is a xdg/macos only feature");
}

#[cfg(all(target_os = "macos", feature = "macos_legacy"))]
fn main() {
    log::info!("this example requires the default macOS backend (UNUserNotificationCenter)");
}

#[cfg(any(
    all(unix, not(target_os = "macos")),
    all(target_os = "macos", not(feature = "macos_legacy"))
))]
fn main() {
    if !common::setup() {
        return;
    }

    Notification::new()
        .summary("Time is running out")
        .body("This will go away.")
        .icon("clock")
        .action(Action::button("dismiss", "Dismiss"))
        .show()
        .expect("failed to show notification")
        .on_close(|reason| log::info!("notification was closed: {reason:?}"));
}
