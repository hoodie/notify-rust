#[cfg(target_os = "macos")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::Notification;

    let duration = chrono::Duration::milliseconds(4321);
    let timestamp =  (chrono::Utc::now() + duration).timestamp() as f64;

    Notification::new()
        .summary("Oh by the way")
        .body(&format!("this was scheduled {:?} ago", duration))
        .schedule(chrono::Utc::now() + duration)?;

    Notification::new()
        .summary("Oh by the way")
        .body(&format!("this was scheduled for timestamp {}", timestamp))
        .schedule_raw(timestamp)?;

    Ok(())
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("this is a mac only feature")
}

#[cfg(windows)]
fn main() {
    println!("this is a mac only feature")
}
