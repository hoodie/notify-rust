#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::Notification;
    std::env::set_var(
        "RUST_LOG",
        "simple=trace,zbus=trace,server=trace,notify_rust=trace",
    );
    color_backtrace::install();
    #[cfg(feature = "env_logger")]
    env_logger::init();

    #[allow(deprecated)]
    Notification::at_bus("example")
        .summary("Critical Error")
        .body("Just <b>kidding</b>, this is just the notification (example).")
        .icon("dialog-error")
        .show()?;
    Ok(())
}
