/// Strictly internal.
/// The NotificationServer implemented here exposes a "Stop" function.
/// stops the notification server
#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
#[doc(hidden)]
pub fn stop(sub_bus: &str) -> crate::error::Result<()> {
    let bus = NotificationBus::custom(sub_bus).ok_or("invalid subpath")?;
    let message = build_message("Stop", bus);
    let connection = Connection::get_private(BusType::Session)?;
    std::thread::sleep(std::time::Duration::from_millis(200));
    connection.send(message)?;
    Ok(())
}
