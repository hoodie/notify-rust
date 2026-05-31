#[cfg(all(target_os = "macos", not(feature = "preview-macos-un")))]
fn main() -> Result<(), String> {
    use notify_rust::{
        error::MacOsError, get_bundle_identifier_or_default, set_application, Notification,
    };

    let safari_id = get_bundle_identifier_or_default("Safari");
    set_application(&safari_id).map_err(|f| format!("{f}"))?;

    match set_application(&safari_id) {
        Ok(_) => {}
        Err(MacOsError::Application(error)) => println!("{error}"),
        Err(MacOsError::Notification(error)) => println!("{error}"),
    }

    Notification::new()
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Safari")
        .icon("Safari")
        .show()
        .map_err(|f| format!("{f}"))?;

    Ok(())
}

#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
fn main() {
    println!("macos app-id is a feature that no longer applies to the un-usernotifications api")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("this is a mac only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a mac only feature")
}
