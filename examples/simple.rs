use notify_rust::Notification;
mod common;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::setup(file!());

    Notification::new()
        .summary("Critical Error")
        .body("Just <b>kidding</b>, this is just the notificationexample.")
        .icon("dialog-error")
        .show()?;
    Ok(())
}
