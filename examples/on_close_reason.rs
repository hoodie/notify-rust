#![allow(unused_imports, dead_code)]
use std::{io, thread};

#[cfg(all(unix, not(target_os = "macos")))]
use notify_rust::{Notification, CloseReason};

fn wait_for_keypress() {
    println!("halted until you hit the \"ANY\" key");
    io::stdin().read_line(&mut String::new()).unwrap();
}

#[cfg(all(unix, not(target_os = "macos")))]
fn print_reason(reason: CloseReason) {
    println!("notification was closed ({:?})", reason);
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn main() { println!("this is a xdg only feature") }

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    thread::spawn(|| {
        Notification::new()
            .summary("Time is running out")
            .body("This will go away.")
            .icon("clock")
            .show()
            .map(|handler| handler.on_close(print_reason))
    });
    wait_for_keypress();
}
