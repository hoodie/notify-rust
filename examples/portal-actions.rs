//! Demonstrates action buttons and `wait_for_action()` via the XDG desktop portal.
//!
//! This mirrors the classic `actions.rs` / `response.rs` examples, but uses
//! `portal::Notification` and `PortalNotificationHandle::wait_for_action()` instead of
//! the classic `Notification` / `NotificationHandle`.
//!
//! # What `wait_for_action` can and cannot see
//!
//! `org.freedesktop.portal.Notification` only defines an `ActionInvoked` signal — there is
//! no `NotificationClosed` signal on the frontend interface (that only exists on the
//! backend implementation interface, which sandboxed apps cannot access). This means:
//!
//! - Clicking a button (including the notification body, which maps to the `"default"`
//!   action) resolves `wait_for_action()` with a [`NotificationResponse`].
//! - Dismissing the notification without clicking anything (closing it, letting it expire,
//!   etc.) does **not** resolve `wait_for_action()` — the future will simply hang forever.
//!   See `examples/portal-on-close.rs` for a dedicated demonstration of that limitation.
//!
//! # Closing the notification afterwards
//!
//! Per the portal spec, notifications are **not** automatically removed when an action is
//! invoked — they are expected to outlast the application until explicitly withdrawn or
//! dismissed by the user. `wait_for_action()` takes `&self`, so the handle is still usable
//! once it resolves; this example calls `handle.close()` afterwards to withdraw the
//! notification once the response has been handled.
//!
//! Run with:
//!
//! ```text
//! cargo run --example portal-actions --features async
//! ```
//!
//! Click "Yes" or "No" to see the response printed and the notification close itself. If you
//! dismiss the notification instead, this process will hang — that is expected, not a bug
//! (see above).
//!
//! # Expected behavior (verified on KDE Plasma 6.7.4 only, see tasks.md)
//!
//! Clicking a button (or the body) prints the response and the notification closes.

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this example is Linux / BSD only (requires xdg-desktop-portal)");
}

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::portal::{Button, Notification, Priority};
    use notify_rust::NotificationResponse;

    println!("Sending notification with action buttons…");

    let handle = Notification::new("Portal survey")
        .body("Did you enjoy the portal notification API?")
        .priority(Priority::Normal)
        .icon_named("dialog-question")
        .button(Button::new("yes", "Yes"))
        .button(Button::new("no", "No"))
        // Clicking the notification body itself (rather than a button) invokes the
        // "default" action, which surfaces here as `NotificationResponse::Default`.
        .default_action("default")
        .show()
        .await?;

    println!("Waiting for you to click a button (or the notification body)…");

    handle
        .wait_for_action(|response: &NotificationResponse| match response {
            NotificationResponse::Default => println!("notification body clicked"),
            NotificationResponse::Action(key) if key == "yes" => println!("you said: yes"),
            NotificationResponse::Action(key) if key == "no" => println!("you said: no"),
            NotificationResponse::Action(key) => println!("unknown action: {key:?}"),
            // Never actually reached via the portal - see module docs above.
            NotificationResponse::Closed(reason) => println!("closed: {reason:?}"),
            NotificationResponse::Reply(text) => println!("replied: {text:?}"),
        })
        .await;

    println!("Closing notification.");
    handle.close().await;

    Ok(())
}
