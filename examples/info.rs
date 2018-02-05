extern crate notify_rust;

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    if let Ok(info) = notify_rust::get_server_information(){
        println!("{}:", info.name);
        println!("  ServerInformation:");
        println!("    name: {}", info.name);
        println!("    vendor: {}", info.vendor);
        println!("    version: {}", info.version);
        println!("    spec_version: {}", info.spec_version);
        println!("  capabilities:  {:#?}", notify_rust::get_capabilities().unwrap_or(Vec::new()));
    } else {
        println!("Error getting ServerInformation");
    }

}

#[cfg(target_os = "macos")] fn main() { println!("this is a xdg only feature") }

#[cfg(target_os = "windows")]
fn main() { println!("this is a xdg only feature") }