extern crate notify_send;
use notify_send::server::NotificationServer;

fn main()
{
    let mut server = NotificationServer::new();
    server.start();

}
