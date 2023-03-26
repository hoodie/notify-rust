#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]

fn main() {
    use notify_rust::CloseReason;

    std::env::set_var("RUST_LOG", "notify_rust=trace");
    env_logger::init();
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        notify_rust::Notification::new()
            .summary("Don't Mind me")
            .hint(notify_rust::Hint::Transient(true))
            .body("I'll be gone soon enough.\nSorry for the inconvenience.")
            .show()
            .unwrap()
            .on_close(|reason: CloseReason| {
                println!("the notification was closed reason: {reason:?}")
            });
    }
}
