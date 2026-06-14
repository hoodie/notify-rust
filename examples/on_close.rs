#![allow(unused_imports)]
mod common;

fn main() {
    if !common::setup(file!()) {
        return;
    }

    use notify_rust::{CloseReason, Hint, Notification, Timeout};
    use std::thread;

    Notification::new()
        .summary("Time is running out")
        .body("Main Thread Notification")
        .icon("clock")
        .timeout(Timeout::Milliseconds(5000))
        .show()
        .unwrap()
        .on_close(|reason| log::info!("❎ fg notification was closed ({reason:?})"));

    common::run_main_loop_while(thread::spawn(|| {
        Notification::new()
            .summary("Time is running out")
            .body("BG Thread Notification")
            .icon("clock")
            .show()
            .unwrap()
            .on_close(|reason| log::info!("❎ bg notification was closed ({reason:?})"));
    }))
    .unwrap();
}
