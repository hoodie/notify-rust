#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    oslog::OsLogger::new("notify-rust")
        .level_filter(log::LevelFilter::Debug)
        .init()
        .unwrap();

    println!("⚠️  make sure you run this using ./run_bundled_example.rs");

    notify_rust::check_bundle()?;
    notify_rust::request_auth_blocking()?;
    log::debug!("{:?}", notify_rust::get_notification_settings_blocking());

    let handle = notify_rust::Notification::new()
        .summary("Pick one")
        .body("This notification has action buttons.")
        .action("yes", "Yes")
        .action("no", "No")
        .timeout(notify_rust::Timeout::Milliseconds(30_000))
        .show()?;

    match handle.response_blocking() {
        notify_rust::NotificationResponse::Action(ref key) if key == "yes" => {
            log::info!("User chose Yes");
        }
        notify_rust::NotificationResponse::Action(ref key) if key == "no" => {
            log::info!("User chose No");
        }
        notify_rust::NotificationResponse::Action(ref key) => {
            log::info!("Unknown action: {key}");
        }
        notify_rust::NotificationResponse::Reply(ref text) => {
            log::info!("User replied: {text}");
        }
        notify_rust::NotificationResponse::Closed(reason) => {
            log::info!("Notification closed: {reason:?}");
        }
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("this is a mac only feature")
}
