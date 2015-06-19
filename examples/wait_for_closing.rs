extern crate notify_rust;

use notify_rust::Notification;

fn main()
{
    Notification::new()
        .summary("Don't Mind me")
        .body("I'll be gone soon enough.\nSorry for the inconvenience.")
        .show_and_wait_for_action({|action|
            match action {
                "__closed" => {println!("the notification was closed")}, // here "__closed" is a hardcoded keyword
                _ => ()
            }
        });
}
