#![allow(unused_must_use)]
use notify_rust::Notification;
use notify_rust::Urgency::*;

#[cfg(target_os = "macos")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // use it this way
    for urgency in &[Low, Normal, Critical] {
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
