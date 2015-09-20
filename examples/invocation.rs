#![allow(unused_must_use)]
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

    // use it this way
    for urgency in [Low, Medium, High].iter(){
        Notification::new()
            .summary(&format!("Urgency {:?}", urgency))
            .body("This notification uses hints")
            .icon("firefox")
            .urgency(*urgency)
            .show();
    }
}
