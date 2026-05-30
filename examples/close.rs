mod common;

#[cfg(any(
    target_os = "windows",
    all(target_os = "macos", feature = "macos_legacy")
))]

fn main() {
    println!("this is a xdg only feature")
}
#[cfg(any(
    all(unix, not(target_os = "macos")),
    all(target_os = "macos", not(feature = "macos_legacy"))
))]
fn main() {
    use notify_rust::*;
    common::setup();

    let handle: NotificationHandle = Notification::new()
        .summary("oh no")
        .hint(Hint::Transient(true))
        .body("I'll be here till you close me!")
        .hint(Hint::Resident(true)) // does not work on kde
        .timeout(Timeout::Never) // works on kde and gnome
        .show()
        .unwrap();

    common::wait_for_keypress("press to close notification");
    handle.close();
    common::wait_for_keypress("press to exit");
}
