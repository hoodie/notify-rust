extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;

fn main()
{
    // use it this way
    Notification::new()
        .summary("Firefox Crashed")
        .body("Just <b>kidding</b>, this is just the notify_show example.")
        .icon("firefox")
        .hint(Hint::Urgency(0))
        .hint(Hint::X(0))
        .show();


}

