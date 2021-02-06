// #![allow(dead_code)]
use notify_rust::{Notification, Timeout, Urgency};
use std::time::Duration;

#[cfg(target_os = "windows")]
fn main() {
    println!("This is not a windows feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn update_via_handle() {
    let configurations = [
        ("low", Urgency::Low, Timeout::Milliseconds(5_000)),
        ("normal", Urgency::Normal, Timeout::Milliseconds(6_000)),
        ("critical", Urgency::Critical, Timeout::Milliseconds(10_000)),
        ("normal", Urgency::Normal, Timeout::Milliseconds(6_000)),
    ];

    let mut notification_handle = Notification::new()
        .summary("First Notification")
        .body("This notification will be changed!")
        .icon("dialog-warning")
        .show()
        .unwrap();

    for (summary, urgency, timeout) in &configurations {
        std::thread::sleep(Duration::from_millis(1_500));

        notification_handle
            .summary(summary)
            .timeout(*timeout)
            .urgency(*urgency)

            .icon("dialog-ok")
            .body("<b>This</b> has been changed through the notification_handle");

        notification_handle.update();
    }
}

fn recycling_one_id() {
    for i in 1..5 {
        let id = 6666; // you should probably not do this at all
        std::thread::sleep(Duration::from_millis(500));
        Notification::new()
            .icon("dialog-ok")
            .body(&format!("notification{}", i))
            .id(id)
            .show()
            .unwrap();
    }
}

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    // please use the handle to update a notification
    update_via_handle();

    //// or come up with your own don't do this:
    // recycling_one_id()
}
