#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    use notify_rust::*;
    fn wait_for_keypress(msg: &str) {
        println!("{}", msg);
        std::io::stdin().read_line(&mut String::new()).unwrap();
    }

    let handle: NotificationHandle = Notification::new()
        .summary("oh no")
        .hint(Hint::Transient(true))
        .body("I'll be here till you close me!")
        .hint(Hint::Resident(true)) // does not work on kde
        .timeout(Timeout::Never) // works on kde and gnome
        .show()
        .unwrap();

    wait_for_keypress("press to close notification");
    handle.close();
    wait_for_keypress("press to exit");
}
