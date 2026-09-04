//! Demonstrates `PortalNotificationHandle::on_close()` — and its main limitation.
//!
//! This mirrors the classic `on_close.rs` example, but for the portal handle.
//!
//! # Why this is different from the classic `on_close`
//!
//! The classic (non-portal) D-Bus interface, `org.freedesktop.Notifications`, defines a
//! `NotificationClosed` signal that fires for *every* way a notification goes away
//! (expired, dismissed by the user, or closed programmatically) and reports why via a
//! [`CloseReason`].
//!
//! The desktop portal frontend interface, `org.freedesktop.portal.Notification`, does
//! **not** have an equivalent signal — only `ActionInvoked` exists, and sandboxed apps have
//! no access to the backend implementation interface where `NotificationClosed` actually
//! lives. As a result, `PortalNotificationHandle::on_close()` is implemented in terms of
//! `wait_for_action()` and can only ever observe `NotificationResponse::Closed` in theory;
//! in practice that variant is never produced by the portal backend, so the closure passed
//! to `on_close()` only fires when an action button (or the notification body) is clicked —
//! **never** when the user plainly dismisses the notification.
//!
//! # The notification does not close itself
//!
//! Per the portal spec, notifications are **not** automatically removed when an action is
//! invoked — they outlast the application by design, until explicitly withdrawn (or
//! dismissed by the user). `on_close()` takes `&self`, so the handle remains usable once it
//! returns; this example calls `handle.close()` afterwards to withdraw the notification once
//! it has been acknowledged.
//!
//! Run with:
//!
//! ```text
//! cargo run --example portal-on-close --features async
//! ```
//!
//! Try both to see the difference:
//!
//! 1. Click "Acknowledge" — the closure fires immediately, the notification is withdrawn,
//!    and the process exits.
//! 2. Dismiss the notification instead (swipe it away / let it sit) — the closure never
//!    fires. This is expected: press Ctrl+C to stop waiting.
//!
//! # Expected behavior (verified on KDE Plasma 6.7.4 only, see tasks.md)
//!
//! Click "Acknowledge": closure fires, notification closes. Dismiss instead: hangs forever
//! (Ctrl+C to stop).

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this example is Linux / BSD only (requires xdg-desktop-portal)");
}

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::portal::{Button, Notification, Priority};

    println!("Sending notification…");
    println!("Click \"Acknowledge\" to resolve on_close(). Dismissing it will NOT resolve it!");

    let handle = Notification::new("Are you still there?")
        .body("Click Acknowledge to let me know, or just dismiss me and see what happens.")
        .priority(Priority::Normal)
        .icon_named("dialog-information")
        .button(Button::new("ack", "Acknowledge"))
        .show()
        .await?;

    handle.on_close(|reason| {
        // Reachable in principle, but never actually produced by the portal backend —
        // included here only for completeness with the classic `on_close` API.
        println!("closed, reason: {reason:?}");
    });

    println!("on_close() resolved — this only happens on an action click via the portal.");

    println!("Closing notification.");
    handle.close().await;

    Ok(())
}
