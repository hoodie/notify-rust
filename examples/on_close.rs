extern crate notify_rust;

use std::io;
use std::thread;

use notify_rust::Notification;

fn wait_for_keypress() {
    println!("halted until you hit the \"ANY\" key");
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn print(){
    println!("notification was closed");
}

fn main() {
    thread::spawn(||
                  Notification::new()
                  .summary("Time is running out")
                  .body("This will go away.")
                  .icon("clock")
                  .show()
                  .map(|mut handler| {
                      handler.wait_for_close();
                      print();}));
    wait_for_keypress();
}

