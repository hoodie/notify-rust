#![allow(unused_imports)]
use notify_rust::{Action, Hint, Notification, Timeout, UserResponse};
mod common;

#[cfg(target_os = "windows")]
fn main() {
    log::info!("this is a xdg only feature");
}

// XDG (Linux/BSD): still uses the deprecated &str-based wait_for_action API.
// On macOS the modern response_blocking() API is used instead (see below).
#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    common::setup();

    Notification::new()
        .summary("click me")
        .body("This will disappear by itself")
        .action(Action::button("clicked_a", "button a")) // IDENTIFIER, LABEL
        .hint(Hint::Transient(true)) // needed to work on kde
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "clicked_a" => log::info!("clicked a"),
            // FIXME: here "__closed" is a hardcoded keyword, it will be deprecated!!
            "__closed" => log::info!("the notification was closed"),
            _ => (),
        });

    Notification::new()
        .summary("click me")
        .body("This action needs to be clicked")
        .action(Action::button("default", "default")) // IDENTIFIER, LABEL
        .action(Action::button("clicked_a", "button a")) // IDENTIFIER, LABEL
        .action(Action::button("clicked_b", "button b")) // IDENTIFIER, LABEL
        .hint(Hint::Resident(true)) // does not work on kde
        .timeout(Timeout::Never) // works on kde and gnome
        .show()
        .unwrap()
        .wait_for_action(|action| match action {
            "default" => log::info!("default"),
            "clicked_a" => log::info!("clicked a"),
            "clicked_b" => log::info!("clicked b"),
            // FIXME: here "__closed" is a hardcoded keyword, it will be deprecated!!
            "__closed" => log::info!("the notification was closed"),
            _ => (),
        });

    // new API: response_blocking() returns a UserResponse directly
    match Notification::new()
        .summary("click me")
        .body("Using the new response API")
        .action(Action::button("default", "default"))
        .action(Action::button("clicked_a", "button a"))
        .action(Action::button("clicked_b", "button b"))
        .hint(Hint::Resident(true))
        .timeout(Timeout::Never)
        .show()
        .unwrap()
        .response_blocking()
    {
        UserResponse::Action(key) if key == "default" => log::info!("default"),
        UserResponse::Action(key) if key == "clicked_a" => log::info!("clicked a"),
        UserResponse::Action(key) if key == "clicked_b" => log::info!("clicked b"),
        UserResponse::Action(other) => log::info!("unknown action: {other}"),
        UserResponse::Reply(text) => log::info!("replied: {text}"),
        UserResponse::Closed(reason) => log::info!("closed: {reason:?}"),
    }
}

#[cfg(all(target_os = "macos", feature = "macos_legacy"))]
fn main() {
    println!("this example requires the default macOS backend (UNUserNotificationCenter)");
}

// macOS: uses the modern response_blocking() / response().await API.
// Hint and urgency methods are not available on macOS.
#[cfg(all(target_os = "macos", not(feature = "macos_legacy")))]
fn main() {
    if !common::setup() {
        return;
    }

    match Notification::new()
        .summary("click me")
        .body("This action needs to be clicked")
        .action(Action::button("clicked_a", "button a"))
        .action(Action::button("clicked_b", "button b"))
        .show()
        .unwrap()
        .response_blocking()
    {
        UserResponse::Action(key) if key == "clicked_a" => log::info!("clicked a"),
        UserResponse::Action(key) if key == "clicked_b" => log::info!("clicked b"),
        UserResponse::Action(other) => log::info!("unknown action: {other}"),
        UserResponse::Reply(text) => log::info!("replied: {text}"),
        UserResponse::Closed(reason) => log::info!("closed: {reason:?}"),
    }
}
