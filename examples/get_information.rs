extern crate notify_rust;
fn main()
{
    println!("capabilities: {:?}", notify_rust::get_capabilities());
    println!("server information: {:?}", notify_rust::get_server_information());
}

