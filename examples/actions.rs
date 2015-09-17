extern crate notify_rust;

use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;

fn main()
{
    Notification::new()
        .summary("click me")
        .action("inert", "inert")
        .action("nothing", "does nothing")
        .hint(Hint::Resident(true))
        .show();
}
