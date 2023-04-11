#[cfg(all(
    unix,
    not(target_os = "macos"),
    feature = "async",
    feature = "config_bus"
))]
#[async_std::main]
async fn main() {
    #[cfg(feature = "env_logger")]
    env_logger::init();

    let bus_name = "example";

    match notify_rust::at_bus::get_server_information(bus_name) {
        Ok(info) => {
            println!("{}:", info.name);
            println!("  ServerInformation:");
            println!("    name: {}", info.name);
            println!("    vendor: {}", info.vendor);
            println!("    version: {}", info.version);
            println!("    spec_version: {}", info.spec_version);
            println!(
                "  capabilities:  {:#?}",
                notify_rust::at_bus::get_capabilities(bus_name).unwrap_or_default()
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
