mod common;

fn main() {
    if !common::setup(file!()) {
        return;
    }

    use notify_rust::{CloseReason, Notification, NotificationResponse, Timeout};

    Notification::new()
        .summary("How was your day?")
        .body("Click a button or just dismiss me.")
        .action("good", "good")
        .action("bad", "bad")
        .timeout(Timeout::Never)
        .show()
        .unwrap()
        // typed, forward-compatible replacement for `wait_for_action`
        .wait_for_response(|response: &NotificationResponse| match response {
            NotificationResponse::Default => log::info!("body clicked"),
            NotificationResponse::Action(key) => log::info!("button {key:?} clicked"),
            // inline replies only come from the macOS `preview-macos-un` backend
            NotificationResponse::Reply(text) => log::info!("user replied: {text}"),
            // no more hardcoded `"__closed"`, the close reason is typed
            NotificationResponse::Closed(CloseReason::Dismissed) => {
                log::info!("dismissed by the user")
            }
            NotificationResponse::Closed(reason) => log::info!("closed: {reason:?}"),
        })
        .unwrap();
}
