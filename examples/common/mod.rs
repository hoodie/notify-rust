#![allow(unused)]
pub fn setup(example_file: &str) -> bool {
    cfg_if::cfg_if! {
        if #[cfg(all(target_os = "macos", feature = "preview-macos-un"))] {
            oslog::OsLogger::new("notify-rust")
                .level_filter(log::LevelFilter::Debug)
                .init()
                .unwrap();
        } else {
            env_logger::init();
            log::trace!("setup env_logger");
            true
        }
    }

    // if we don't bundle we must log to stdout
    #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
    {
        let example_name = std::path::PathBuf::from(example_file)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap()
            .to_string();

        if let Err(error) = mac_usernotifications::check_bundle() {
            eprintln!(
            "\x1b[1mError:\x1b[0m {error}\n \x1b[1mHelp:\x1b[0m Please run the examples via \x1b[1;35m`./run_bundled_example.sh {example_name}`\x1b[0m"
        );
        }

        match notify_rust::request_auth_blocking() {
            Ok(true) => {
                println!("Notification permission granted.");
                true
            }
            Ok(false) => {
                log::warn!(
                    "Notification permission denied. \
                 Allow it in System Settings -> Notifications."
                );
                false
            }
            Err(error) => {
                log::error!("Authorization error: {error}");
                false
            }
        }
    }
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
