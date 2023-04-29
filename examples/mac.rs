#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::Notification;

    Notification::new()
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Toastify")
        .icon("Toastify")
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
