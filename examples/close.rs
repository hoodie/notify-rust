#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(any(all(unix, not(target_os = "macos")), target_os = "windows"))]
fn main() {
    use notify_rust::{Notification, NotificationHandle, Timeout};

    fn wait_for_keypress(msg: &str) {
        println!("{}", msg);
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    let handle: NotificationHandle = Notification::new()
        .summary("oh no")
        .body("I'll be here till you close me!")
        .timeout(Timeout::Never)
        .show()
        .unwrap();

    wait_for_keypress("press to close notification");
    handle.close();
    wait_for_keypress("press to exit");
}
