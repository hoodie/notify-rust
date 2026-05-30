mod common;

#[cfg(all(target_os = "macos", feature = "macos_legacy"))]
fn main() {
    println!("this example requires the default macOS backend (UNUserNotificationCenter)");
}

#[cfg(all(target_os = "macos", not(feature = "macos_legacy")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::{Action, Notification, UserResponse};

    if !common::setup() {
        return Ok(());
    }

    Notification::new()
        .summary("Safari Crashed")
        .body("Just kidding, this is just the notify_rust example.")
        .appname("Toastify")
        .icon("Toastify")
        .show()?;

    Notification::new()
        .summary(".image_path()")
        .body("Trying to open an image")
        .image_path("./examples/octodex.jpg")
        .show()?;

    let result_a = Notification::new()
        .summary("click me")
        .body("This action needs to be clicked")
        .action(Action::button("clicked_a", "OK"))
        .show();

    if let Ok(handle) = result_a {
        match handle.response_blocking() {
            UserResponse::Action(key) if key == "clicked_a" => println!("clicked OK"),
            UserResponse::Action(other) => println!("unknown action: {other}"),
            UserResponse::Reply(text) => println!("reply: {text}"),
            UserResponse::Closed(_) => println!("the notification was closed"),
        }
    }

    let result_b = Notification::new()
        .summary("pick one")
        .body("This menu has several options")
        .action(Action::button("clicked_a", "button a"))
        .action(Action::button("clicked_b", "button b"))
        .action(Action::button("clicked_c", "button c"))
        .show();

    if let Ok(handle) = result_b {
        match handle.response_blocking() {
            UserResponse::Action(key) if key == "clicked_a" => println!("clicked a"),
            UserResponse::Action(key) if key == "clicked_b" => println!("clicked b"),
            UserResponse::Action(key) if key == "clicked_c" => println!("clicked c"),
            UserResponse::Action(other) => println!("unknown action: {other}"),
            UserResponse::Reply(text) => println!("reply: {text}"),
            UserResponse::Closed(_) => println!("the notification was closed"),
        }
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("this is a mac only example")
}
