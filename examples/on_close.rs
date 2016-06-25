#![allow(unused_must_use)]
extern crate notify_rust;
use notify_rust::Notification;
fn main()
{
    Notification::new()
        .summary("Time is running out")
        .body("This will go away.")
        .icon("clock")
        .show().unwrap()
        .on_close(||{println!("closed")});
}

