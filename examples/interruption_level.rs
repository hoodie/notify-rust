//! Demonstrates setting an interruption level on a macOS notification.
//!
//! Interruption levels control whether a notification breaks through
//! Focus modes. Requires macOS 12+.
//!
//! Run with:
//! ```text
//! cargo run --example interruption_level --features preview-macos-un
//! ```

#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    notify_rust::check_bundle()?;
    notify_rust::request_auth_blocking()?;

    notify_rust::Notification::new()
        .summary("Time Sensitive")
        .body("This notification bypasses Focus settings.")
        .urgency(notify_rust::InterruptionLevel::TimeSensitive)
        .show()?;

    println!("Notification sent.");
    // TODO: add run_bundled script
    // add oslog dev-dep
    Ok(())
}
