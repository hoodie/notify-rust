#[cfg(target_os = "windows")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(target_os = "macos", feature = "macos_legacy"))]
fn main() {
    println!("this example requires the default macOS backend (UNUserNotificationCenter)");
}

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::Notification;

    Notification::new()
        .summary("async notification")
        .subtitle("subtitle")
        .body("this notification was sent via an async api")
        .icon("dialog-positive")
        .show_async()
        .await?;
    Ok(())
}

#[cfg(all(target_os = "macos", not(feature = "macos_legacy")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::Notification;

    notify_rust::request_auth().await?;

    Notification::new()
        .summary("async notification")
        .subtitle("subtitle")
        .body("this notification was sent via an async api")
        .show_async()
        .await?;
    Ok(())
}
