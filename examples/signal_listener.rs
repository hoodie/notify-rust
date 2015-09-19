extern crate notify_rust;

use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;

fn main()
{
    Notification::new()
        .summary("click me")
        .action("default", "default")
        .action("clicked", "click here")
        .hint(Hint::Resident(true))
        .show()
        .unwrap()
        .wait_for_action({|action|
            match action {
                "default" => {println!("so boring")},
                "clicked" => {println!("that was correct")},
                "__closed" => {println!("the notification was closed")}, // here "__closed" is a hardcoded keyword
                _ => ()
            }
        });
}
