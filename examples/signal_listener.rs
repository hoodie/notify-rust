extern crate notify_rust;

use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;

fn main()
{
    Notification::new().summary("don't click me").action("clicked wrong", "click here").show();
    let id = Notification::new().summary("click me").action("clicked", "click here").hint(Hint::Resident(true)).show();
    notify_rust::wait_for_action_signal(id);
}
