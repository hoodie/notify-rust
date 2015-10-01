#![allow(unused_must_use)]
extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationUrgency::*;
fn main()
{
    // use it this way
    for urgency in [Low, Normal, Critical].iter(){
        Notification::new()
            .summary(&format!("Urgency {:?}", urgency))
            .body("This notification uses hints")
            .icon("firefox")
            .urgency(*urgency)
            .show();
    }

    Notification::new()
        .body("Urgency from String")
        .icon("dialog-warning")
        .urgency("high".into()) // only if you realy have to :D
        .show();

}


