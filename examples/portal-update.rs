//! Demonstrates in-place notification updates via the XDG desktop portal.
//!
//! This example uses `portal::Notification` together with
//! `PortalNotificationHandle::update_with()` to replace a live notification
//! with new content without creating a new popup each time.
//!
//! # Why `update_with` instead of re-calling `show()`
//!
//! `xdg-desktop-portal` tracks active notifications by `(app_id, notification_id)`.
//! When a D-Bus connection closes, the portal fires `on_peer_disconnect` and removes
//! every notification that was sent over that connection.  For unsandboxed processes
//! (e.g. run from a terminal) the `app_id` is `""`, so every notification from every
//! connection shares the same app namespace.
//!
//! If you call `.show()` again with the same ID on a *new* connection the sequence is:
//!
//! 1. First connection sends `AddNotification(id="x", ...)` → portal records `("", "x")`.
//! 2. First connection drops → `on_peer_disconnect` erases `("", "x")`.
//! 3. Second connection sends `AddNotification(id="x", ...)` → portal sees a *new*
//!    entry for `("", "x")` and shows a new popup rather than replacing the first.
//!
//! `update_with()` avoids this by reusing the original connection for every subsequent
//! `AddNotification` call, so the portal always sees the same sender and performs a
//! true in-place replacement.
//!
//! Run with:
//!
//! ```text
//! cargo run --example portal-update --features async
//! ```
//!
//! # Expected behavior (verified on KDE Plasma 6.7.4 only, see tasks.md)
//!
//! One notification, body ticks from "0%" to "100%" — not a new popup per step.

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this example is Linux / BSD only (requires xdg-desktop-portal)");
}

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use async_std::task::sleep;
    use notify_rust::portal::{Button, Notification, Priority};
    use std::time::Duration;

    // Step 1 — Send the initial notification and hold on to the handle.
    //
    // The handle keeps the D-Bus connection alive.  All subsequent updates
    // must go through this handle so the portal sees the same sender.
    println!("Showing initial notification…");
    let mut handle = Notification::new("Uploading…")
        .body("0% complete")
        .priority(Priority::Normal)
        .icon_named("document-send")
        .button(Button::new("cancel", "Cancel"))
        .show()
        .await?;

    // Step 2 — Update in-place by calling handle.update_with() on the *same* handle.
    //
    // Each call reuses self.connection, so the portal treats the call as a replacement
    // of the existing notification rather than a new one.
    for pct in [25u8, 50, 75, 100] {
        sleep(Duration::from_millis(800)).await;

        let body = if pct < 100 {
            format!("{}% complete", pct)
        } else {
            "Done! ✓".to_owned()
        };

        println!("Updating to {}%…", pct);

        // Build a fresh portal::Notification describing the new state.
        // The ID is irrelevant here — update_with() always uses the handle's ID.
        let update = Notification::new("Uploading…")
            .body(body)
            .priority(Priority::Normal)
            .icon_named("document-send")
            .button(Button::new("cancel", "Cancel"));

        handle.update_with(update).await;
    }

    // Step 3 — Let the final state linger for a moment, then dismiss.
    sleep(Duration::from_secs(2)).await;

    println!("Closing notification.");
    handle.close().await;

    Ok(())
}
