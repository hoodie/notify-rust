#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
fn main() {
    env_logger::init();
    if let Err(err) = notify_rust::server::stop("example") {
        println!("{err}")
    }
}

#[cfg(all(unix, not(feature = "server"), not(target_os = "macos")))]
fn main() {
    println!("server feature required")
}

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a xdg only feature")
}
