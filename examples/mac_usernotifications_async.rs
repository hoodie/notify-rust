#[cfg(all(target_os = "macos", feature = "macos_legacy"))]
fn main() {
    println!("this example requires the default macOS backend (UNUserNotificationCenter)");
}

#[cfg(all(target_os = "macos", not(feature = "macos_legacy")))]
fn main() {
    use futures_lite::future::zip;
    use mac_usernotifications::block_on_main;
    use notify_rust::{Action, Notification, UserResponse};

    // a bundled app can not log to stdout
    oslog::OsLogger::new("notify-rust")
        .level_filter(log::LevelFilter::Debug)
        .init()
        .unwrap();

    block_on_main(async {
        notify_rust::request_auth_blocking().unwrap();

        // Send all notifications concurrently and collect their handles.
        let ((result_plain, result_image), (result_a, result_b)) = zip(
            zip(
                Notification::new()
                    .summary("Safari Crashed")
                    .body("Just kidding, this is just the notify_rust example.")
                    .appname("Toastify")
                    .icon("Toastify")
                    .show_async(),
                Notification::new()
                    .summary(".image_path()")
                    .body("Trying to open an image")
                    .image_path("./examples/octodex.jpg")
                    .show_async(),
            ),
            zip(
                Notification::new()
                    .summary("click me (async)")
                    .body("This action needs to be clicked")
                    .action(Action::button("clicked_a", "OK"))
                    .show_async(),
                Notification::new()
                    .summary("pick one (async)")
                    .body("This menu has several options")
                    .action(Action::button("clicked_a", "button a"))
                    .action(Action::button("clicked_b", "button b"))
                    .action(Action::button("clicked_c", "button c"))
                    .show_async(),
            ),
        )
        .await;

        if let Err(e) = result_plain {
            eprintln!("plain notification failed: {e}");
        }
        if let Err(e) = result_image {
            eprintln!("image notification failed: {e}");
        }

        if let Ok(handle) = result_a {
            match handle.response_blocking() {
                UserResponse::Action(key) if key == "clicked_a" => println!("clicked OK"),
                UserResponse::Action(other) => println!("notification A: unknown action: {other}"),
                UserResponse::Reply(text) => println!("notification A: reply: {text}"),
                UserResponse::Closed(_) => println!("notification A was closed"),
            }
        }

        if let Ok(handle) = result_b {
            match handle.response_blocking() {
                UserResponse::Action(key) if key == "clicked_a" => println!("clicked a"),
                UserResponse::Action(key) if key == "clicked_b" => println!("clicked b"),
                UserResponse::Action(key) if key == "clicked_c" => println!("clicked c"),
                UserResponse::Action(other) => println!("notification B - unknown action: {other}"),
                UserResponse::Reply(text) => println!("notification B - reply: {text}"),
                UserResponse::Closed(_) => println!("notification B was closed"),
            }
        }
    });
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("this is a macOS only example - see `actions.rs` for the XDG version");
}
