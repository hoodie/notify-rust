extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;
fn main()
{
    Notification::new()
        .summary("Firefox Crashed")
        .body("Just <b>kidding</b>, this is just the notify_show example.")
        .icon("firefox")
        .hint(Hint::Resident(true)) // does not work on kde
        .timeout(0) // works on kde and gnome
        .show();

}

