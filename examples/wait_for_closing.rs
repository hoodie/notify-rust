#[cfg(target_os = "macos")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(any(all(unix, not(target_os = "macos")), target_os = "windows"))]
fn main() {
    use notify_rust::CloseReason;

    let mut notification = notify_rust::Notification::new();
    notification
        .summary("Don't Mind me")
        .body("I'll be gone soon enough.\nSorry for the inconvenience.");

    #[cfg(all(unix, not(target_os = "macos")))]
    notification.hint(notify_rust::Hint::Transient(true));

    notification
        .show()
        .unwrap()
        .on_close(|reason: CloseReason| println!("the notification was closed reason: {reason:?}"));
}
