#![allow(unused_must_use)]
extern crate notify_rust;
use notify_rust::Notification;

#[cfg(target_os = "macos")]
static SOUND: &'static str = "Ping";

#[cfg(all(unix, not(target_os = "macos")))]
static SOUND: &'static str = "message-new-instant";


fn main() {
    Notification::new().summary("notification with sound")
                       .sound_name(SOUND)
                       .show();
}
