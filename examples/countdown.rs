extern crate notify_rust;
use notify_rust::Notification;
use std::time::Duration;

fn main()
{
    let mut notification = Notification::new()
        .summary("Firefox Crashed")
        .body("count down")
        .icon("clock")
        .timeout(2000)
        .show()
        .unwrap();


    for i in 0..11{
        std::thread::sleep(Duration::from_millis(1_000));
        notification.body(&format!("{}", 10-i))
        .appname(&format!("countdown_{}", 10-i));
        notification.update();
        println!("{}", 10-i);
    }

    notification.body("booom");
    notification.appname("countdown_boom");
    notification.update();

}

