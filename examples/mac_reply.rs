mod common;

#[cfg(target_os = "macos")]
fn main() {
    if !common::setup(file!()) {
        return;
    }

    use notify_rust::{Notification, NotificationResponse, NsNotificationExt};

    let print_response = |response: &NotificationResponse| match response {
        NotificationResponse::Default => log::info!("body clicked"),
        NotificationResponse::Action(key) => log::info!("button {key:?} clicked"),
        NotificationResponse::Reply(text) => log::info!("user replied: {text}"),
        NotificationResponse::Closed(reason) => log::info!("closed: {reason:?}"),
    };

    Notification::new()
        .summary("Lunch?")
        .body("type where you want to go")
        .reply("Pizza place around the corner")
        .close_button("Not hungry")
        .show()
        .unwrap()
        .wait_for_response(print_response)
        .unwrap();

    Notification::new()
        .summary("Pick a pill")
        .body("choose wisely")
        .action("red", "🔴 Red Pill")
        .action("blue", "🔵 Blue Pill")
        .actions_label("Pills")
        .close_button("Neither")
        .show()
        .unwrap()
        .wait_for_response(print_response)
        .unwrap();
}

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("this is a mac only feature")
}
