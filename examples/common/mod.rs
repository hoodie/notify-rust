#![allow(dead_code)]

#[cfg(target_os = "macos")]
pub fn setup() -> bool {
    oslog::OsLogger::new("notify-rust")
        .level_filter(log::LevelFilter::Debug)
        .init()
        .unwrap();
    setup_mac_auth()
}

/// Legacy path: use the bundle-id trick from `mac-notification-sys`.
#[cfg(all(target_os = "macos", feature = "macos_legacy"))]
fn setup_mac_auth() -> bool {
    let bundle_id = notify_rust::get_bundle_identifier_or_default("zed");
    notify_rust::set_application(&bundle_id).unwrap();
    true
}

/// Default path: request UNUserNotificationCenter permission.
#[cfg(all(target_os = "macos", not(feature = "macos_legacy")))]
pub fn setup_mac_auth() -> bool {
    match notify_rust::request_auth_blocking() {
        Ok(true) => {
            log::info!("Notification permission granted.");
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

#[cfg(not(target_os = "macos"))]
pub fn setup() -> bool {
    env_logger::init();
    true
}

pub fn wait_for_keypress(msg: &str) {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    log::info!("{msg}");

    let (sender, receiver) = mpsc::channel();

    let timeout_sender = sender.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(4));
        let _ = timeout_sender.send(());
    });

    // NOTE: the stdin thread will keep blocking after the timeout.
    // There is no portable way to cancel a blocking read, so
    // the thread is intentionally leaked here. This is fine for an example.
    thread::spawn(move || {
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).is_ok() && !line.is_empty() {
            let _ = sender.send(());
        }
    });

    let _ = receiver.recv();
}

#[macro_export]
macro_rules! async_main {
    ($future:expr) => {
        fn main() {
            oslog::OsLogger::new("notify-rust")
                .level_filter(log::LevelFilter::Debug)
                .init()
                .unwrap();

            if !common::setup_mac_auth() {
                return;
            }
            mac_usernotifications::block_on_main($future)
        }
    };
}
