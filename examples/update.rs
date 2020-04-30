#![allow(dead_code)]
use notify_rust::Notification;
use std::time::Duration;


#[cfg(target_os = "windows")]
fn main() { println!("This is not a windows feature") }

#[cfg(all(unix, not(target_os = "macos")))]
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

    notification_handle.update();
}

#[allow(dead_code)]
#[cfg(all(unix, not(target_os = "macos")))]
fn update_via_manually_stored_id() {
    let handle = Notification::new()
        .summary("First Notification")
        .body("This notification will be changed!")
        .icon("dialog-warning")
        .show()
        .unwrap();

    let id = handle.id();
    std::thread::sleep(Duration::from_millis(1_500));

    Notification::new()
        .appname("foo") // changing appname to keep plasma from merging the new and the old one
        .icon("dialog-ok")
        .body("<b>This</b> has been changed by sending a new notification with the same id")
        .id(id)
        .show()
        .unwrap();
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

    //// If your really have to, store the if yourself
    // update_via_manually_stored_id();

    //// or come up with your own don't do this:
    recycling_one_id()
}
