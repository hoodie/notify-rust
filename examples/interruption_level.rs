//! Demonstrates setting an interruption level on a macOS notification.
//!
//! Interruption levels control whether a notification breaks through
//! Focus modes. Requires macOS 12+.
//!
//! Run with:
//! ```text
//! cargo run --example interruption_level --features preview-macos-un
//! ```

#![allow(unused_imports)]
mod common;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cfg_if::cfg_if! {
        if #[cfg(all(target_os = "macos", feature = "preview-macos-un"))] {
            if !common::setup(file!()) {
                return Ok(());
            }
            notify_rust::check_bundle()?;

            notify_rust::Notification::new()
                .summary("Time Sensitive")
                .body("This notification bypasses Focus settings.")
                .urgency(notify_rust::InterruptionLevel::TimeSensitive)
                .show()?;

            println!("Notification sent.");
            // TODO: add run_bundled script
            // add oslog dev-dep
            Ok(())
        } else {
            println!("this example requires --features preview-macos-un on macOS");
            Ok(())
        }
    }
}
