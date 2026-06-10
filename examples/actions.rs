#![allow(unused_imports)]
#[cfg(target_os = "windows")]
use notify_rust::Urgency;
use notify_rust::{Hint, Notification, Timeout};

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature");
}

#[cfg(target_os = "windows")]
fn main() {
    Notification::new()
        .summary("click me")
        .body("This action needs to be clicked")
        .action("clicked_a", "button a")
        .action("clicked_b", "button b")
        .timeout(Timeout::Never)
        .urgency(Urgency::Critical)
        .show_handle()
        .unwrap()
        .wait_for_action(|action| match action {
            "default" => println!("default"),
            "clicked_a" => println!("clicked a"),
            "clicked_b" => println!("clicked b"),
            "__closed" => println!("the notification was closed"),
            _ => (),
        });
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    Notification::new()
        .summary("click me")
        .body("This will disappear by itself")
        .action("clicked_a", "button a") // IDENTIFIER, LABEL
        .hint(Hint::Transient(true)) // needed to work on kde
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "clicked_a" => println!("clicked a"),
            // FIXME: here "__closed" is a hardcoded keyword, it will be deprecated!!
            "__closed" => println!("the notification was closed"),
            _ => (),
        });

    Notification::new()
        .summary("click me")
        .body("This action needs to be clicked")
        .action("default", "default") // IDENTIFIER, LABEL
        .action("clicked_a", "button a") // IDENTIFIER, LABEL
        .action("clicked_b", "button b") // IDENTIFIER, LABEL
        .hint(Hint::Resident(true)) // does not work on kde
        .timeout(Timeout::Never) // works on kde and gnome
        .show()
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
