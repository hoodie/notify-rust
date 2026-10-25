mod common;

// A notification without any buttons. `wait_for_response` should still block until the user
// clicks the body (or dismisses the notification) instead of returning right away.
fn main() {
    if !common::setup(file!()) {
        return;
    }

    use notify_rust::{Notification, NotificationResponse, Timeout};

    Notification::new()
        .summary("click me")
        .body("no buttons, click the body")
        .timeout(Timeout::Never)
        .show()
        .unwrap()
        .wait_for_response(|response: &NotificationResponse| match response {
            NotificationResponse::Default => log::info!("body clicked"),
            NotificationResponse::Closed(reason) => log::info!("closed: {reason:?}"),
            other => log::info!("unexpected: {other:?}"),
        })
        .unwrap();
}
