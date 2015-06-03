extern crate notify_send;
use notify_send::Notification;
fn main()
{
    let capabilities = Notification::get_capabilities();
    for capability in capabilities{
        println!("{}", capability);
    }
}
