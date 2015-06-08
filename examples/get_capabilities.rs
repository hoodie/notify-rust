extern crate notify_rust;
fn main()
{
    let capabilities:Vec<String> = notify_rust::get_capabilities();
    for capability in capabilities{
        println!("{}", capability);
    }
}
