//! Show Volume example
//!
//! Only works on unity
//!

#![allow(unused_imports, dead_code)]
use notify_rust::Hint;
use notify_rust::Notification;
use std::time::Duration;

enum Volume {
    Muted,
    Percent(i32),
}

#[cfg(all(unix, not(target_os = "macos")))]
fn show_volume(percent: Volume) {
    let icon = match percent {
        Volume::Muted => "notification-audio-volume-muted",
        Volume::Percent(x) if x == 0 => "notification-audio-volume-off",
        Volume::Percent(x) if x < 33 => "notification-audio-volume-low",
        Volume::Percent(x) if x < 67 => "notification-audio-volume-medium",
        _ => "notification-audio-volume-high",
    };

    let value = match percent {
        Volume::Muted => 0,
        Volume::Percent(p) => p,
    };

    Notification::new()
        .summary(" ")
        .icon(icon)
        .hint(Hint::SoundName("audio-volume-change".into()))
        .hint(Hint::Custom("synchronous".into(), "volume".into()))
        .hint(Hint::CustomInt("value".into(), value))
        .show()
        .unwrap();
}

#[cfg(any(windows, target_os = "macos"))]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    show_volume(Volume::Muted);
    for i in 1..11 {
        std::thread::sleep(Duration::from_millis(1_000));
        show_volume(Volume::Percent(i * 10));
    }
}
