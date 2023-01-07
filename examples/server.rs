use notify_rust::server;
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
    std::env::set_var("RUST_LOG", "notify_rust=trace");
    env_logger::init();
    // notify_rust::server::blocking_start_with(|notification| eprintln!("{notification:#?}"))
    server::blocking_start_with(server::print_notification)?;

    Ok(())
}
