#![allow(unused_must_use)]
extern crate notify_rust;
use notify_rust::Notification;

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &'static str ="message-new-instant";

#[cfg(target_os = "windows")]
static SOUND: &'static str = "Mail";

fn main() {
    Notification::new()
        .summary("notification with sound")
        .sound_name(SOUND)
        .show();
}
