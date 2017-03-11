#![allow(unused_must_use)]
extern crate notify_rust;
use notify_rust::*;
use notify_rust::NotificationHint as Hint;
fn main()
{
    Notification::new()
        .summary("Persistent notification")
        .body("This should not go away unless you want it to.")
        .icon("firefox")
        .hint(Hint::Resident(true)) // does not work on kde
        .timeout(Timeout::Never) // works on kde and gnome
        .show();

}
