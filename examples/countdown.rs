extern crate notify_rust;
use notify_rust::Notification;
use std::time::Duration;

#[cfg(target_os = "macos")] fn main() { println!("this is a xdg only feature") }
#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    let mut notification = Notification::new()
        .summary("Rocket launch in ...")
        .body("count down")
        .icon("clock")
        .timeout(0)
        .show()
        .unwrap();


    for i in 0..11{
        std::thread::sleep(Duration::from_millis(1_000));
        notification.body(&format!("T-minus {}", 10-i))
        .appname(&format!("countdown_{}", 10-i));
        notification.update();
        println!("{}", 10-i);
    }

    notification.body("TAKE OFF")
    .appname("countdown_takeoff")
    .show()
    .unwrap();

}

