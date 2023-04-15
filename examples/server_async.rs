#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(feature = "server"), not(target_os = "macos")))]
fn main() {
    println!("server feature required")
}

#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Duration;

    use notify_rust::{
        server::{self, ReceivedNotification},
        CloseReason, Notification, Timeout,
    };

    fn print(reason: CloseReason) {
        println!("notification was closed {reason:?}");
        log::info!("✅ close_Handler done")
    }

    env_logger::init();

    let running_server = async_std::task::spawn(async {
        if let Err(error) =
            server::start_at("example", |received: ReceivedNotification| async move {
                log::debug!("received_notification");
                async_std::task::sleep(Duration::from_secs(1)).await;
                log::debug!("handling notification after sleep");

                if let Some((_action, closer)) = received.channels() {
                    let reason = CloseReason::from(3);
                    log::debug!("sending reason {reason:?}");
                    closer.send(reason).await.unwrap();
                    log::debug!("sent reason {reason:?}");
                } else {
                    log::debug!("channel upgrade failed, can no longer send action or close");
                }
                log::info!("✅ handler done")
            })
            .await
        {
            log::warn!("handler failed {error}")
        }
    });

    async_std::task::sleep(std::time::Duration::from_secs(1)).await;

    let sent_notification = async {
        async_std::task::sleep(Duration::from_secs(1)).await;
        log::debug!("sending notification after sleep");
        if let Err(error) = Notification::new()
            .timeout(Timeout::Never)
            .show_async_at_bus("example")
            .await
            .map(|handler| handler.on_close(print))
        {
            log::warn!("sending failed {error}")
        }
        log::info!("✅ sending done");

        running_server.cancel().await;
        log::info!("✅ server stopped again");
    };

    sent_notification.await;
    Ok(())
}
