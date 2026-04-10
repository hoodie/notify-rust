#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(any(all(unix, not(target_os = "macos")), target_os = "windows"))]
fn main() {
    use notify_rust::{CloseReason, Notification};

    thread::spawn(|| {
        Notification::new()
            .summary("Time is running out")
            .body("This will go away.")
            .icon("clock")
            .show()
            .map(|handler| {
                handler.on_close(|reason: CloseReason| {
                    println!("notification was closed ({:?})", reason);
                })
            })
            .unwrap();
    });
    wait_for_keypress();
}
