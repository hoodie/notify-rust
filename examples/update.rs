extern crate notify_rust;
use notify_rust::Notification;
fn main()
{
    let mut notification = Notification::new()
        .summary("Firefox Crashed")
        .body("Just <b>kidding</b>, this is just the notify_show example.")
        .icon("firefox")
        .show();

    std::thread::sleep_ms(1_500);

    notification
        .appname("foo") // changing appname to keep plasma from merging the new and the old one
        .body("wait, something has changed");

    notification.update();
}

