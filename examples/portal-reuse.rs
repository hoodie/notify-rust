//! Demonstrates the portal's ID-based replace behaviour by reusing an explicit ID.
//!
//! This mirrors the classic `reuse.rs` example. Per the portal spec, calling
//! `AddNotification` again with the **same `id`** replaces the previously shown
//! notification in-place, rather than showing a second popup.
//!
//! # How this differs from `portal-update.rs`
//!
//! `examples/portal-update.rs` demonstrates `PortalNotificationHandle::update_with()`,
//! which explicitly reuses the *same D-Bus connection* for every update — the recommended
//! approach, and the only one that reliably replaces the notification on spec-compliant
//! backends (see that example's docs for why the connection matters).
//!
//! This example instead calls `portal::Notification::new(...).id(...).show()` twice,
//! **each on its own fresh connection** (since `show()` opens a new session-bus connection
//! per call). This is the simplest possible way to reuse an ID, but it is more fragile:
//! `xdg-desktop-portal` tracks active notifications by `(app_id, id)` and evicts them when
//! the *originating* connection disconnects. Since unsandboxed processes usually share the
//! same empty `app_id` (`""`), whether this still counts as a "replace" depends on whether
//! the first connection has already dropped by the time the second `AddNotification`
//! arrives — a race that `update_with()` avoids entirely.
//!
//! Run with:
//!
//! ```text
//! cargo run --example portal-reuse --features async
//! ```
//!
//! # Expected behavior (verified on KDE Plasma 6.7.4 only, see tasks.md)
//!
//! One notification, replaced in place — not two separate popups.

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this example is Linux / BSD only (requires xdg-desktop-portal)");
}

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use async_std::task::sleep;
    use notify_rust::portal::{Notification, Priority};
    use std::time::Duration;

    const STABLE_ID: &str = "notify-rust-portal-reuse-example";

    println!("Showing first notification with id = {STABLE_ID:?}…");
    Notification::new("News update")
        .body("Something bad happened")
        .priority(Priority::Normal)
        .icon_named("dialog-warning")
        .id(STABLE_ID)
        .show()
        .await?;

    sleep(Duration::from_millis(1500)).await;

    println!("Showing second notification with the same id…");
    Notification::new("News update")
        .body("Just kidding, nothing happened")
        .priority(Priority::Normal)
        .icon_named("dialog-information")
        .id(STABLE_ID)
        .show()
        .await?;

    Ok(())
}
