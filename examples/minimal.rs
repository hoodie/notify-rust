#![allow(unused_must_use)]
extern crate notify_rust;
use notify_rust::Notification;
fn main() {
    Notification::new()
        .summary("minimal notification")
        .show();
}

