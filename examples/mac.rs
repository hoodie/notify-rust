
extern crate notify_rust;

#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<std::error::Error>> {
    use notify_rust::{Notification, get_bundle_identifier_or_default, set_application};

    let safari_id = get_bundle_identifier_or_default("Safari");
    set_application(&safari_id)?;
    set_application(&safari_id)?;

    Notification::new()
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Safari")
        .icon("Safari")
        .show()?;

    Ok(())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("this is a mac only feature")
}