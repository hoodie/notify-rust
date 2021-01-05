#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    println!("{:?}", notify_rust::dbus_stack());
}

#[cfg(target_os = "macos")]
fn main() {
    println!("mac-notification-sys")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("winrt-notification")
}
