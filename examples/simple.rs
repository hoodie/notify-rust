use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("Critical Error")
        .body("Just <b>kidding</b>, this is just the notificationexample.")
        .icon("dialog-error")
        .show()?;
    Ok(())
}
