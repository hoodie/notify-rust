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

#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    thread::spawn(|| {
        Notification::new()
            .summary("Time is running out")
            .body("This will go away.")
            .icon("clock")
            .show()
            .map(|handler| {
                handler.on_close(print);
                handler.on_close(print);
            })
    });
    wait_for_keypress();
}
