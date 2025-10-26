#[cfg(target_os = "macos")]
fn main() {
    println!("Urgency is not supported on macOS")
}

#[cfg(any(all(unix, not(target_os = "macos")), target_os = "windows"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::{Notification, Urgency::*};

    #[cfg(all(unix, not(target_os = "macos")))]
    println!("Testing urgency on Linux/BSD (XDG)");
    #[cfg(all(unix, not(target_os = "macos")))]
    println!("Urgency is sent as a hint. Critical notifications should not timeout.\n");

    #[cfg(target_os = "windows")]
    println!("Testing urgency on Windows");
    #[cfg(target_os = "windows")]
    println!("Low/Normal → Default scenario, Critical → Reminder scenario (stays on screen)\n");

    // Test all urgency levels
    for urgency in &[Low, Normal, Critical] {
        freeze(&format!("next urgency: {urgency}"));
        let mut notification = Notification::new();
        notification
            .summary(&format!("Urgency {:?}", urgency))
            .body("This notification demonstrates urgency levels")
            .urgency(*urgency);

        #[cfg(all(unix, not(target_os = "macos")))]
        notification.icon("firefox");

        notification.show()?;
    }

    // Test urgency from string
    let mut notification = Notification::new();
    freeze(&format("next urgency: {:?}", "high"));
    notification
        .summary("Urgency from String")
        .body("This uses 'high' which maps to Critical")
        .urgency("high".try_into()?);

    #[cfg(all(unix, not(target_os = "macos")))]
    notification.icon("dialog-warning");

    notification.show()?;

    Ok(())
}
