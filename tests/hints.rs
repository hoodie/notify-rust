#![allow(unused_must_use)]
extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;
use notify_rust::NotificationUrgency::*;

#[test]
fn urgency()
{
    // use it this way
    for urgency in [
        Hint::Urgency(Low),
        Hint::Urgency(Medium),
        Hint::Urgency(High)
    ].iter(){
        Notification::new()
            .summary(&format!("Urgency {:?}", urgency))
            .hint(urgency.clone())
            .show();
    }
}

#[test]
fn category()
{
    Notification::new()
        .appname("thunderbird")
        .summary("Category:email")
        .icon("thunderbird")
        .hint(Hint::Category("email".to_string()))
        .show();
}

#[test]
fn persistent() {

    Notification::new()
        .summary("Incoming Call: Your Mom!")
        .body("Resident:True")
        .icon("call-start")
        .hint(Hint::Resident(true))
        .show();

    Notification::new()
        .summary("Incoming Call: Your Mom!")
        .body("Resident:False, but Timeout=0")
        .icon("call-start")
        .hint(Hint::Resident(false))
        .timeout(0)
        .show();

}

