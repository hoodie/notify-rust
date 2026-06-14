#![allow(unused)]

use std::thread;

pub fn setup(example_file: &str) -> bool {
    cfg_if::cfg_if! {
        if #[cfg(all(target_os = "macos", not(feature = "preview-macos-un")))] {
            env_logger::init();
            log::trace!("setup env_logger");

            // for NSUserNotifications we need to set the application bundle identifier to the default of safari
            let bundle_id = notify_rust::get_bundle_identifier_or_default("safari");
            notify_rust::set_application(&bundle_id).unwrap();
            log::trace!("set_application: {bundle_id}");
            true
        } else if #[cfg(all(target_os = "macos", feature = "preview-macos-un"))] {
            // for UNUserNotifications we need to log to the system log
            // and request authorization
            oslog::OsLogger::new("notify-rust")
                .level_filter(log::LevelFilter::Trace)
                .init()
                .unwrap();
            log::trace!("setup oslog");
            notify_rust::request_auth_blocking();

            true
        } else {
            env_logger::init();
            log::trace!("setup env_logger");
            true
        }
    }
}

pub fn run_main_loop_while<T>(thread: thread::JoinHandle<T>) -> thread::Result<T> {
    #[cfg(target_os = "macos")]
    {
        use objc2_foundation::{NSDate, NSDefaultRunLoopMode, NSRunLoop};
        let run_loop = NSRunLoop::mainRunLoop();
        while !thread.is_finished() {
            let until = NSDate::dateWithTimeIntervalSinceNow(0.05);
            unsafe { run_loop.runMode_beforeDate(NSDefaultRunLoopMode, &until) };
        }
        thread.join()
    }

    #[cfg(not(target_os = "macos"))]
    thread.join()
}

/// Blocks until the user presses a key or timeout is reached (in bundle context).
pub fn wait_for_keypress(msg: &str) {
    #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
    {
        if mac_usernotifications::check_bundle().is_ok() {
            // Running in a bundle - use timeout instead
            use std::{sync::mpsc, thread, time::Duration};
            log::info!("{msg}, will timeout in 4 seconds");

            let (sender, receiver) = mpsc::channel();
            let timeout_sender = sender.clone();

            thread::spawn(move || {
                thread::sleep(Duration::from_secs(4));
                let _ = timeout_sender.send(());
            });

            let _ = receiver.recv();
            return;
        }
    }

    // Not in bundle (or not macOS) - wait for keypress
    log::info!("{}", msg);
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
