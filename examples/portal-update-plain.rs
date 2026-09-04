//! Demonstrates the bare `PortalNotificationHandle::update()` — re-sending unchanged content.
//!
//! This is the portal counterpart to the classic `update.rs` example's `update_via_handle()`
//! function, and complements `examples/portal-update.rs` (which uses `update_with()` to send
//! *new* content).
//!
//! # `update()` vs. `update_with()`
//!
//! `update()` takes no arguments — it just re-sends `AddNotification` with the *original*
//! content the handle was created with, reusing the handle's own connection:
//!
//! ```text
//! pub fn update(&mut self) {
//!     self.update_fallible().unwrap();
//! }
//! ```
//!
//! There is currently no public way to mutate the content stored in a `PortalNotificationHandle`
//! before calling `update()` — if you need different content, use
//! [`update_with()`](notify_rust::portal::NotificationHandle::update_with) instead (see
//! `examples/portal-update.rs`). Bare `update()` is mainly useful for "bumping" a notification —
//! re-asserting it (and its original content) is still current — without having to reconstruct
//! a `portal::Notification` each time.
//!
//! Run with:
//!
//! ```text
//! cargo run --example portal-update-plain --features async
//! ```
//!
//! # Expected behavior (verified on KDE Plasma 6.7.4 only, see tasks.md)
//!
//! One notification, refreshed in place three times — not four separate popups.

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

    println!("Showing initial notification…");
    let mut handle = Notification::new("Still running…")
        .body("This notification gets re-sent, unchanged, every second.")
        .priority(Priority::Normal)
        .icon_named("appointment-soon")
        .show()
        .await?;

    for tick in 1..=3 {
        sleep(Duration::from_secs(1)).await;
        println!("Calling update() — resend #{tick} (content unchanged)…");
        handle.update();
    }

    sleep(Duration::from_secs(1)).await;

    println!("Closing notification.");
    handle.close().await;

    Ok(())
}
