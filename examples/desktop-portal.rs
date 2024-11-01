#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_backtrace::install();
    use std::time::Duration;

    use async_std::task::sleep;
    use notify_rust::{Notification, Urgency};
    // TODO: add special portal type with only those properties
    let handle = Notification::new()
        .summary("portal notification")
        // .body("this notification was sent via an async api")
        // .urgency(Urgency::Critical)
        // .icon("dialog-positive")
        .icon("./examples/octodex.jpg")
        .show_via_portal("de.hoodie.notify-rust.examples.portal-notification")
        .await?;

    sleep(Duration::from_secs(2)).await;

    handle.close();

    Ok(())
}
