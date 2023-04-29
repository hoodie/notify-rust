#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    #[cfg(feature = "env_logger")]
    env_logger::init();
    match notify_rust::get_server_information() {
        Ok(info) => {
            println!("{}:", info.name);
            println!("  ServerInformation:");
            println!("    name: {}", info.name);
            println!("    vendor: {}", info.vendor);
            println!("    version: {}", info.version);
            println!("    spec_version: {}", info.spec_version);
            println!(
                "  capabilities:  {:#?}",
                notify_rust::get_capabilities().unwrap_or_default()
            );
        }
        Err(err) => eprintln!("error: {}", err),
    }
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is a xdg only feature")
}
