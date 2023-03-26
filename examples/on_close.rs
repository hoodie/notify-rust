use notify_rust::{CloseReason, Notification};

fn print(reason: CloseReason) {
    println!("notification was closed {reason:?}");
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    use std::time::Duration;

    std::env::set_var("RUST_LOG", "notify_rust=trace");
    env_logger::init();

    let timeout = std::env::args()
        .nth(1)
        .and_then(|x| x.parse::<u64>().ok())
        .unwrap_or(5);

    Notification::new()
        .summary("Time is running out")
        .body("This will go away.")
        .icon("clock")
        // .action("action", "action")
        .timeout(Duration::from_secs(dbg!(timeout)))
        .show()
        .map(|handler| handler.on_close(print))
        .unwrap();
}
