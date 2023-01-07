use notify_rust::Notification;

fn print() {
    println!("notification was closed, don't know why");
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    std::env::set_var("RUST_LOG", "notify_rust=trace");
    env_logger::init();

    Notification::new()
        .summary("Time is running out")
        .body("This will go away.")
        .icon("clock")
        .timeout(4000)
        .show()
        .map(|handler| handler.on_close(print))
        .unwrap();
}
