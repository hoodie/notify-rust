extern crate notify_rust;
use notify_rust::Notification;

fn main() {

    Notification::new()
        .summary("Firefox Crashed")
        .body("Just <b>kidding</b>, this is just the notify-rust example.")
        .icon("firefox")
        .show()
        .unwrap();

}
