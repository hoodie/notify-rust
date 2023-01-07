#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    env_logger::init();
    if let Err(err) = notify_rust::stop_server() {
        println!("{err}")
    }
}

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a xdg only feature")
}
