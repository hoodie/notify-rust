use std::error::Error;
#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(feature = "server"), not(target_os = "macos")))]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
//#[async_std::main]
// async
fn main() -> Result<(), Box<dyn Error>> {
    // notify_rust::server::start().await
    notify_rust::server::start_with_blocking(|notification| eprintln!("{notification:#?}"))
}
