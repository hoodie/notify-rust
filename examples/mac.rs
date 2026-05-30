#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::Notification;

    // a bundled app cannot log to stdout
    oslog::OsLogger::new("notify-rust")
        .level_filter(log::LevelFilter::Debug)
        .init()
        .unwrap();

    #[cfg(feature = "macos_legacy")]
    {
        let bundle_id = notify_rust::get_bundle_identifier_or_default("zed");
        notify_rust::set_application(&bundle_id).unwrap();
    }

    #[cfg(not(feature = "macos_legacy"))]
    notify_rust::request_auth_blocking().unwrap();

    Notification::new()
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Toastify")
        .icon("Toastify")
        .show()?;

    Notification::new()
        .summary(".image_path()")
        .body("Trying to open an image")
        .image_path(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/octodex.jpg"))
        .show()?;

    Ok(())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("this is a mac only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a mac only feature")
}
