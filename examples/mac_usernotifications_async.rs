//! Demonstrates the `preview_macos_un` backend with async response handling.
//!
//! Uses [`mac_usernotifications::block_on_main`] to drive the async future on the
//! main thread while keeping the `NSRunLoop` pumped — required for macOS to deliver
//! notification response callbacks.
//!
//! Run with:
//! ```text
//! cargo run --example mac_usernotifications_async --features preview-macos-un
//! ```

#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    notify_rust::check_bundle()?;
    notify_rust::request_auth_blocking()?;

    mac_usernotifications::block_on_main(async {
        let handle = notify_rust::Notification::new()
            .summary("Async notification")
            .body("Waiting for your response…")
            .action("ok", "OK")
            .action("cancel", "Cancel")
            .timeout(notify_rust::Timeout::Milliseconds(30_000))
            .show_async()
            .await
            .expect("failed to show notification");

        let response = handle.response().await;
        println!("Response: {response:?}");
    });

    Ok(())
}
