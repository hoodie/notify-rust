#![allow(unused_imports)]

use notify_rust::Notification;

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    Notification::new()
        .summary("click me")
        .body("Waiting for your response.")
        .action("default", "default")
        .action("open", "Open")
        .show()
        .unwrap()
        .wait_for_action(|action| println!("action: {action}"));
}

#[cfg(target_os = "macos")]
fn main() {
    Notification::new()
        .summary("click me")
        .body("Waiting for your response.")
        .action("default", "default")
        .show()
        .unwrap()
        .wait_for_action(|action| println!("action: {action}"));
}

#[cfg(target_os = "windows")]
fn main() {
    let _handle = Notification::new()
        .summary("Notify Rust Windows")
        .body("Windows toast activation is not exposed by the backend yet.")
        .show()
        .unwrap();

    println!("Windows listener callbacks are not exposed by tauri-winrt-notification 0.7");
}
