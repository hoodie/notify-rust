use notify_rust::Notification;

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
