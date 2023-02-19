#[cfg(target_os = "macos")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]

fn main() {
    let handle = notify_rust::Notification::new()
        .summary("Don't Mind me")
        .hint(notify_rust::Hint::Transient(true))
        .body("I'll be gone soon enough.\nSorry for the inconvenience.")
        .show()
        .unwrap();
    handle.wait_for_action(|action| {
        if "__closed" == action {
            println!("the notification was closed")
        }
    });
}
