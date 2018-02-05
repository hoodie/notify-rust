extern crate notify_rust;
use notify_rust::Notification;

fn main() {
    Notification::new()
        .summary("Notify Rust Windows")
        .body("yay, we have limited windows support\nWith multiple lines\nSound\nBut no images")
        .show()
        .unwrap();
}
