use notify_rust::Notification;
fn main() {
    // here we create a notification and then reuse a second time

    let mut notification = Notification::new()
        .summary("News update")
        .icon("computer")
        .body("Something bad happened")
        .finalize();

    notification.show().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1500));

    notification.body("just kidding, nothing happened");
    notification.show().unwrap();
}
