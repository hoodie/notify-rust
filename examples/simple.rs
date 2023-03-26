use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var(
        "RUST_LOG",
        "simple=trace,zbus=trace,server=trace,notify_rust=trace",
    );
    color_backtrace::install();
    env_logger::init();

    Notification::new()
        .summary("Critical Error")
        .body("Just <b>kidding</b>, this is just the notificationexample.")
        .icon("dialog-error")
        .show()?;
    log::trace!("sent");
    Ok(())
}
