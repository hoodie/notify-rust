#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
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

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a xdg only feature")
}
