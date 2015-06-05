extern crate notify_rust;
use notify_rust::Notification;
fn main()
{
    let capabilities:Vec<String> = Notification::get_capabilities();
    for capability in capabilities{
        println!("{}", capability);
    }
}
