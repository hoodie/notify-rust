#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::*;
    use std::time::Duration;

    Notification::new()
        .summary("Notification Duration timeout")
        .body("this one should stay for 2s")
        .icon("timer")
        .timeout(Duration::from_secs(2))
        .show()?;

    Notification::new()
        .summary("Notification ms timeout")
        .body("this one should stay for 2s (2000ms)")
        .icon("timer")
        .timeout(2_000)
        .show()?;

    Notification::new()
        .summary("Notification string timeout")
        .body("this one should stay for 2s (\"2000\")")
        .icon("timer")
        .timeout("2000".parse::<Timeout>().unwrap())
        .show()?;
    Ok(())
}
