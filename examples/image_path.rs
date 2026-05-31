use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    cfg_if::cfg_if! {
        if #[cfg(feature = "preview-macos-un")] {
            notify_rust::check_bundle().unwrap();
            notify_rust::request_auth_blocking().unwrap();
        } else {
            let bundle_id = notify_rust::get_bundle_identifier_or_default("safari");
            notify_rust::set_application(&bundle_id).unwrap();
        }
    }
    Notification::new()
        .summary(".image_path()")
        .body("Trying to open an image")
        .image_path("./examples/octodex.jpg")
        .show()?;

    Ok(())
}
