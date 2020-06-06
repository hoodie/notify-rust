use notify_rust::*;

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a xdg only feature")
}

fn wait_for_keypress(msg: &str) {
    println!("{}", msg);
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    let handle: notify_rust::NotificationHandle = Notification::new()
        .summary("oh no")
        .hint(notify_rust::Hint::Transient(true))
        .body("I'll be here till you close me!")
        .hint(Hint::Resident(true)) // does not work on kde
        .timeout(Timeout::Never) // works on kde and gnome
        .show()
        .unwrap();

    wait_for_keypress("press to close notification");
    handle.close();
    wait_for_keypress("press to exit");
}
