#![allow(unused_imports)]
use notify_rust::Notification;

fn main() {
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let handle: notify_rust::NotificationHandle = Notification::new()
            .summary("Don't Mind me")
            .hint(notify_rust::Hint::Transient(true))
            .body("I'll be gone soon enough.\nSorry for the inconvenience.")
            .show()
            .unwrap();
        handle.wait_for_action(|action| {
            if "__closed" == action {
                println!("the notification was closed")
            }
        });
    }

    #[cfg(target_os = "macos")]
    Notification::new()
        .summary("PLATFORM ERROR")
        .subtitle("unsupported functionality")
        .body("cannot wait for closing on macOS.")
        .show()
        .unwrap();
}
