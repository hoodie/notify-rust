extern crate notify_rust;
use std::time::Duration;
use std::thread;

#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
use notify_rust::server::NotificationServer;
use notify_rust::Notification;

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(not(feature = "server"), unix, not(target_os = "macos")))]
fn main() {
    println!("please build with '--features=server'")
}

#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
fn main() {
    let server = NotificationServer::create();
    thread::spawn(move || NotificationServer::start(&server, |notification| println!("{:#?}", notification)));

    std::thread::sleep(Duration::from_millis(500));

    Notification::new().summary("Notification Logger")
                       .body("If you can read this in the console, the server works fine.")
                       .show()
                       .unwrap();

    println!("Press enter to exit.\n");
    let mut _devnull = String::new();
    let _ = std::io::stdin().read_line(&mut _devnull);
    println!("Thank you for choosing notify-rust.");
}
