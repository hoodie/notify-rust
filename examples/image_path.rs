use notify_rust::Notification;

/// This does NOT require the `images` feature
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(all(target_os = "macos", feature = "macos_legacy"))]
    {
        let bundle_id = notify_rust::get_bundle_identifier_or_default("zed");
        notify_rust::set_application(&bundle_id).unwrap();
    }

    #[cfg(all(target_os = "macos", not(feature = "macos_legacy")))]
    notify_rust::request_auth_blocking().unwrap();

    Notification::new()
        .summary(".image_path()")
        .body("Trying to open an image")
        .image_path("./examples/octodex.jpg")
        .show()?;

    Ok(())
}
