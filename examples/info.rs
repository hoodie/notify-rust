extern crate notify_rust;
fn main()
{
    println!("server information:\n  {:?}", notify_rust::get_server_information());
    println!("\ncapabilities:\n  {:?}", notify_rust::get_capabilities());
}

