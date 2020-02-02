#![allow(unused_imports)]
// import `NotificationHandle` to test re-export
use notify_rust::{Hint, Notification, NotificationHandle};

fn main() {
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let handle: NotificationHandle = Notification::new()
            .summary("Don't Mind me")
            .hint(Hint::Transient(true))
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
