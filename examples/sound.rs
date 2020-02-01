#![allow(unused_must_use)]
use notify_rust::Notification;

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &str = "message-new-instant";

fn main() {
    Notification::new()
        .summary("notification with sound")
        .sound_name(SOUND)
        .show();
}
