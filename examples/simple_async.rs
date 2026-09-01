#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::Notification;
    Notification::new()
        .summary("async notification")
        .body("this notification was sent via an async api")
        .icon("dialog-positive")
        .show_async()
        .await?;

    Notification::new()
        .summary("portal notification")
        .body("this notification was sent via an desktop portal api")
        .icon("dialog-positive")
        .show_via_portal()
        .await?;
    Ok(())
}
