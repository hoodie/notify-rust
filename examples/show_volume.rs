extern crate notify_rust;
use self::notify_rust::Notification;
use self::notify_rust::NotificationHint;
use std::time::Duration;

enum Volume {
    Muted,
    Percent(i32),
}

fn show_volume(percent: Volume){
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
        .hint(NotificationHint::SoundName("audio-volume-change".to_owned()))
        .hint(NotificationHint::Custom("synchronous".to_owned(), "volume".to_owned()))
        .hint(NotificationHint::CustomInt("value".to_owned(), value))
        .show().unwrap();
}

fn main(){
    show_volume(Volume::Muted);
    for i in 1..11{
        std::thread::sleep(Duration::from_millis(1_000));
        show_volume(Volume::Percent(i*10));
    }
}
