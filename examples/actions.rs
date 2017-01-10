extern crate notify_rust;

use notify_rust::{Action, Notification};
use notify_rust::NotificationHint as Hint;

fn main()
{
    let mut notification = Notification::new()
        .summary("click me")

        .action("default", "default")    // IDENTIFIER, LABEL
        .action("clicked", "click here") // IDENTIFIER, LABEL

        .hint(Hint::Resident(true))
        .show()
        .unwrap();

    match notification.wait_for_action() {
        Action::NotificationClosed => println!("the notification was closed"),
        Action::ActionInvoked(action) => {
            match &*action {
                "default" => println!("so boring"),
                "clicked" => println!("that was correct"),
                _ => (),
            }
        }
    }
}
