extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;
use notify_rust::NotificationUrgency::*;
fn main()
{
    Notification::new()
        .summary("Firefox Crashed")
        .body("Just <b>kidding</b>, this is just the notify_show example.")
        .icon("firefox")
        .show();

    urgency();
}

fn urgency()
{
    // use it this way
    for urgency in 0..3{
        Notification::new()
            .summary(&format!("Urgency {}", urgency))
            .body("This notification uses hints")
            .icon("firefox")
            .hint(Hint::Urgency(Low))
            .show();
    }
}
