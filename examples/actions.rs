extern crate notify_rust;

use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;

fn main()
{
    Notification::new()
        .summary("click me")

        .action("default", "default")    // IDENTIFIER, LABEL
        .action("clicked", "click here") // IDENTIFIER, LABEL

        .hint(Hint::Resident(true))
        .show()
        .unwrap()
        .wait_for_action({|action|
            match action {
                "default"  => println!("so boring"),
                "clicked"  => println!("that was correct"),
                // here "__closed" is a hardcoded keyword
                "__closed" => println!("the notification was closed"),
                _ => ()
            }
        });
}
