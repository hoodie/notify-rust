extern crate notify_rust;
use notify_rust::server::NotificationServer;

fn main()
{
    let mut server = NotificationServer::new();
    server.start();
}
