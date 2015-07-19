extern crate notify_rust;
use std::thread;

use notify_rust::Notification;
use notify_rust::server::NotificationServer;

fn main()
{
    let mut server = NotificationServer::new();
    thread::spawn(move||{
        server.start(
            |appname, _id, icon, summary, body, actions, hints, counter | 
            println!("[{counter}]  ({icon}) appname: {appname:?}\nsummary: {summary}\nbody:    {body}\nactions:     {actions}\nhints:     {hints}\n",
            appname = appname, icon = icon, summary = summary, body = body, actions = actions, hints = hints, counter = counter)
            );
    });

    println!("Press enter to exit.\n");

    std::thread::sleep_ms(1_000);

    Notification::new()
        .summary("Notification Logger")
        .body("If you can read this in the console, the server works fine.")
        .show();

    let mut _devnull = String::new();
    let _ = std::io::stdin().read_line(&mut _devnull);
    println!("Thank you for choosing notify-rust.");
}
