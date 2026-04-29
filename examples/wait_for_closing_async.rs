#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
fn main() {
    use notify_rust::{CloseReason, Notification};

    zbus::block_on(async {
        Notification::new()
            .summary("Don't Mind me")
            .hint(notify_rust::Hint::Transient(true))
            .body("I'll be gone soon enough.\nSorry for the inconvenience.")
            .show_async()
            .await
            .unwrap()
            .on_close(|reason: CloseReason| {
                println!("the notification was closed reason: {reason:?}")
            })
            .unwrap();

        Notification::new()
            .summary("Pick an option")
            .body("Click one of the action buttons below.")
            .action("option-a", "Option A")
            .action("option-b", "Option B")
            .show_async()
            .await
            .unwrap()
            .wait_for_action_async(|action| {
                println!("action invoked: {action:?}");
            })
            .await
            .unwrap();
    })
}
