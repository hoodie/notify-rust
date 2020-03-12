#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::{
        get_bundle_identifier_or_default, set_application, Notification,
    };

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

#[cfg(linux)]
fn main() {
    println!("this is a mac only feature")
}

#[cfg(windows)]
fn main() {
    println!("this is a mac only feature")
}
