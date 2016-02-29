extern crate notify_rust;
use self::notify_rust::Notification;
use self::notify_rust::NotificationHint;
use self::notify_rust::NotificationHandle;
use std::time::Duration;

fn main(){
    let icon = "notification-audio-volume-muted";

    let value = 100i32;
    let mut notification = Notification::new()
        .summary(" ")
        .icon(icon)
        .appname(&format!("countdown_{}", value))
        .hint(NotificationHint::SoundName("audio-volume-change".to_string()))
        .hint(NotificationHint::Custom("synchronous".to_string(), "volume".to_string()))
        .hint(NotificationHint::CustomInt("value".to_string(), value))
        .show().unwrap();

    for i in 0..11{
        std::thread::sleep(Duration::from_millis(1_000));
        notification.hint(NotificationHint::CustomInt("value".to_string(), (10-i)*100));
        notification.appname(&format!("volume_{}", i));
        notification.update();
        println!("{}", 10-i);
    }

}
