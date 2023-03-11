use notify_rust::CloseReason;

#[cfg(target_os = "windows")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]

fn main() {
    use zbus::export::futures_util::FutureExt;

    zbus::block_on(async {
        notify_rust::Notification::new()
            .summary("Don't Mind me")
            .hint(notify_rust::Hint::Transient(true))
            .body("I'll be gone soon enough.\nSorry for the inconvenience.")
            .show_async()
            .then(|handle| async move {
                match handle {
                    Ok(handle) => handle.on_close(|reason: CloseReason| {
                        println!("the notification was closed reason: {reason:?}")
                    }),
                    Err(error) => println!("failed to send notification {error}"),
                }
            })
            .await;
    })
}
