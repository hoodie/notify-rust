#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::{get_bundle_identifier_or_default, set_application, Notification};

    let bundle_id = get_bundle_identifier_or_default("zed");
    set_application(&bundle_id).unwrap();

    let handle = Notification::new()
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Toastify")
        .icon("Toastify")
        .action("open", "Open")
        .show()?;

    handle.wait_for_action(|action| {
        println!("action: {action}");
    });

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
