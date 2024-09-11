#[cfg(any(target_os = "macos", target_os = "windows"))]
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
    color_backtrace::install();

    use notify_rust::{
        server::{self, ReceivedNotification},
        CloseReason, Notification, Timeout,
    };

    fn print(reason: CloseReason) {
        println!("notification was closed {reason:?}");
        log::info!("✅ close_Handler done")
    }

    env_logger::init();
    log::debug!("lets go");

    let running_server = async_std::task::spawn(async {
        let running = server::start_at("example", |received: ReceivedNotification| async move {
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
            log::info!("✅ handler done");
            Ok(None)
        })
        .await;
    });

    // TODO: figure out that the server is ready instead of sleeping
    async_std::task::sleep(std::time::Duration::from_secs(1)).await;

    let waiting_to_close = async {
        async_std::task::sleep(Duration::from_secs(1)).await;
        log::debug!("sending notification after sleep");
        if let Err(error) = Notification::new()
            .summary("waiting to close")
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

    // let actions = async {
    //     async_std::task::sleep(Duration::from_secs(1)).await;
    //     log::debug!("sending notification after sleep");
    //     let notification = Notification::new()
    //         .summary("waiting to close")
    //         .timeout(Timeout::Never)
    //         .hint(notify_rust::Hint::Resident(true))
    //         .show_async_at_bus("example")
    //         .await
    //         .unwrap();

    //     let notification2: NotificationHandle = notification.clone();
    //     notification.wait_for_action(|action| log::info!("action handled {action}"));
    //     notification2.wait_for_action(|action| log::info!("action handled {action}"));
    // };

    waiting_to_close.await;
    // actions.await;

    Ok(())
}
