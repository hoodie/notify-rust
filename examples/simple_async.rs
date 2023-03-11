use notify_rust::Notification;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("async notification")
        .body("this notification was sent via an async api")
        .icon("dialog-positive")
        .show_async()
        .await?;
    Ok(())
}
