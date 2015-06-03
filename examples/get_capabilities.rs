extern crate notify_send;
use notify_send::Notification;
fn main()
{
    let capabilities:Vec<String> = Notification::get_capabilities();
    for capability in capabilities{
        println!("{}", capability);
    }
}
