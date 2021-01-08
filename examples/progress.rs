#[cfg(target_os = "macos")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut notification = notify_rust::Notification::new().summary("progress").show()?;
    for i in 0..=10 {
        let value = i * 10;
        notification
            .body(&format!("progress {}%", value))
            .hint(notify_rust::Hint::CustomInt("value".to_string(), value));
        std::thread::sleep(std::time::Duration::from_secs(1));
        notification.update();
    }
    Ok(())
}
