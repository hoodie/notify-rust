#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
fn main() {
    env_logger::init();
    notify_rust::server::stop("example").unwrap()
}

#[cfg(all(unix, not(feature = "server"), not(target_os = "macos")))]
fn main() {
    println!("server feature required")
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is a xdg only feature")
}
