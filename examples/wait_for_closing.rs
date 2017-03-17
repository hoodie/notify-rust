extern crate notify_rust;

use notify_rust::{Notification,NotificationHint};

fn main()
{
    Notification::new()
        .summary("Don't Mind me")
        .hint(NotificationHint::Transient(true))
        .body("I'll be gone soon enough.\nSorry for the inconvenience.")
        .show().unwrap()
        .wait_for_action({|action| if let "__closed" = action {
            println!("the notification was closed")
        }
        });
}
