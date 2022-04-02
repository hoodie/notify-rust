#![allow(unused_imports, dead_code)]
use std::{io, thread};

use notify_rust::Notification;

fn wait_for_keypress() {
    println!("halted until you hit the \"ANY\" key");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn print() {
    println!("notification was closed, don't know why");
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn main() { println!("this is a xdg only feature") }

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() {
    use notify_rust::CloseReason;


    std::env::set_var("RUST_LOG", "notify_rust=trace");
    env_logger::init();
    async_std::task::spawn(async move {
        Notification::new()
            .summary("Time is running out")
            .body("This will go away.")
            .icon("clock")
            .on_close(|_: CloseReason| println!("closed"))
            .show()
    });
    wait_for_keypress();
}
