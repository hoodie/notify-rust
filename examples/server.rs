#![allow(unused_must_use)]
extern crate notify_rust;
use std::thread;

use notify_rust::Notification;
use notify_rust::server::NotificationServer;

fn main()
{
    let mut server = NotificationServer::new();
    thread::spawn(move||{
        server.start( |notification| println!("{:#?}", notification))
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
