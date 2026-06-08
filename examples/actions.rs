#![allow(unused_imports)]
use notify_rust::{Hint, Notification, Timeout};

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature");
}

#[cfg(any(target_os = "windows", all(unix, not(target_os = "macos"))))]
fn main() {
    let mut first = Notification::new();
    first
        .summary("click me")
        .body("This will disappear by itself")
        .action("clicked_a", "button a");
    #[cfg(all(unix, not(target_os = "macos")))]
    first.hint(Hint::Transient(true));
    first
        .show_handle()
        .unwrap()
        .wait_for_action(|action| match action {
            "clicked_a" => println!("clicked a"),
            "default" => println!("default"),
            // FIXME: here "__closed" is a hardcoded keyword, it will be deprecated!!
            "__closed" => println!("the notification was closed"),
            _ => (),
        });

    let mut second = Notification::new();
    second
        .summary("click me")
        .body("This action needs to be clicked")
        .action("default", "default")
        .action("clicked_a", "button a")
        .action("clicked_b", "button b")
        .timeout(Timeout::Never);
    #[cfg(all(unix, not(target_os = "macos")))]
    second.hint(Hint::Resident(true));
    second
        .show_handle()
        .unwrap()
        .wait_for_action(|action| match action {
            "default" => println!("default"),
            "clicked_a" => println!("clicked a"),
            "clicked_b" => println!("clicked b"),
            // FIXME: here "__closed" is a hardcoded keyword, it will be deprecated!!
            "__closed" => println!("the notification was closed"),
            _ => (),
        });
}
