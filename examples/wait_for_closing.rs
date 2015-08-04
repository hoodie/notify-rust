extern crate notify_rust;

use notify_rust::{Notification,NotificationHint};

fn main()
{
    Notification::new()
        .summary("Don't Mind me")
        .hint(NotificationHint::Transient(true))
        .body("I'll be gone soon enough.\nSorry for the inconvenience.")
        .show()
        .wait_for_action({|action|
            match action {
                "__closed" => {println!("the notification was closed")}, // here "__closed" is a hardcoded keyword
                _ => ()
            }
        });
}
