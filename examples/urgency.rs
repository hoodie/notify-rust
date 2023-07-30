#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::{Notification, Urgency::*};
    // use it this way
    for urgency in &[Low, Normal, Critical] {
        Notification::new()
            .summary(&format!("Urgency {:?}", urgency))
            .body("This notification uses hints")
            .icon("firefox")
            .urgency(*urgency)
            .show()?;
    }

    Notification::new()
        .body("Urgency from String")
        .icon("dialog-warning")
        .urgency("high".try_into()?)
        .show()?;

    Ok(())
}
