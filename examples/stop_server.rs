#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    env_logger::init();
    notify_rust::stop_server().unwrap();
}

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a xdg only feature")
}
