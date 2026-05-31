#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::Notification;

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
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Toastify")
        .icon("Toastify")
        .show()?;

    Notification::new()
        .summary(".image_path()")
        .body("Trying to open an image")
        .image_path("./examples/octodex.jpg")
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
