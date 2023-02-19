#[cfg(target_os = "macos")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is an xdg only feature")
}

use notify_rust::Notification;

#[cfg(all(unix, not(target_os = "macos")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    zbus::block_on(
        Notification::new()
            .summary("async notification")
            .body("this notification was sent via an async api")
            .icon("dialog-positive")
            .show_async(),
    )?;
    Ok(())
}
