#![allow(dead_code)]
use notify_rust::Notification;
use std::time::Duration;

mod common;

#[cfg(target_os = "windows")]
fn main() {
    println!("This is not a windows feature")
}

#[cfg(any(
    all(unix, not(target_os = "macos")),
    all(target_os = "macos", not(feature = "macos_legacy"))
))]
fn update_via_handle() {
    let mut notification_handle = Notification::new()
        .summary("First Notification")
        .body("This notification will be changed!")
        .icon("dialog-warning")
        .show()
        .unwrap();

    std::thread::sleep(Duration::from_millis(1_500));

    notification_handle
        .appname("foo") // changing appname to keep plasma from merging the new and the old one
        .icon("dialog-ok")
        .body("<b>This</b> has been changed through the notification_handle");

    notification_handle.update().unwrap();
}

#[cfg(any(
    all(unix, not(target_os = "macos")),
    all(target_os = "macos", not(feature = "macos_legacy"))
))]
fn update_via_stored_id() {
    let handle = Notification::new()
        .summary("First Notification")
        .body("This notification will be changed!")
        .icon("dialog-warning")
        .show()
        .unwrap();

    let stored_id = handle.id();
    std::thread::sleep(Duration::from_millis(1_500));

    Notification::new()
        .appname("foo") // changing appname keeps plasma from merging old and new
        .icon("dialog-ok")
        .body("<b>This</b> has been changed by sending a new notification with the same id")
        .id(stored_id)
        .show()
        .unwrap();
}

#[cfg(any(
    all(unix, not(target_os = "macos")),
    all(target_os = "macos", not(feature = "macos_legacy"))
))]
fn recycling_one_id() {
    for i in 1..5 {
        let recycled_id: u32 = 6666; // you should probably not do this at all
        std::thread::sleep(Duration::from_millis(500));
        Notification::new()
            .icon("dialog-ok")
            .body(&format!("notification {i}"))
            .id(recycled_id)
            .show()
            .unwrap();
    }
}

#[cfg(all(target_os = "macos", feature = "macos_legacy"))]
fn main() {
    println!("this example requires the default macOS backend (UNUserNotificationCenter)");
}

#[cfg(any(
    all(unix, not(target_os = "macos")),
    all(target_os = "macos", not(feature = "macos_legacy"))
))]
fn main() {
    if !common::setup() {
        return;
    }

    // please use the handle to update a notification
    update_via_handle();

    // If you really have to, store the id yourself
    update_via_stored_id();

    // or recycle a hardcoded id (not recommended)
    recycling_one_id();
}
