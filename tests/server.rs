use async_std::future::pending;
use notify_rust::{
    server::{self, NotificationHandler, ReceivedNotification},
    CloseReason, Notification, Timeout,
};
use std::time::Duration;

fn never_handle() -> impl NotificationHandler + Clone {
    move |_received: ReceivedNotification| async move {
        pending::<()>().await;
    }
}

fn close_after(close_reason: CloseReason, sleep_ms: u64) -> impl NotificationHandler + Clone {
    move |received: ReceivedNotification| async move {
        async_std::task::sleep(Duration::from_millis(sleep_ms)).await;

        let (_action, closer) = received.channels().unwrap();
        closer.send(close_reason).await.unwrap();
        eprintln!("sent reason {close_reason:?}");
    }
}

#[async_std::test]
async fn expire_notification_after_default_timeout() -> std::io::Result<()> {
    env_logger::init();
    let running_server = async_std::task::spawn(async {
        server::start_at("expire_notification_after_default_timeout", never_handle()).await.unwrap();
    });
    log::info!("expire server");

    let sent_notification = async {
        async_std::task::sleep(Duration::from_millis(10)).await;

        Notification::new()
            .timeout(Timeout::Default)
            .show_async()
            .await
            .map(|handler| {
                handler.on_close(|received_reason: CloseReason| {
                    eprintln!("received close reason {received_reason:?}");
                    assert_eq!(received_reason, CloseReason::Expired)
                })
            })
            .unwrap();
        log::info!("sent notification with default timeout");

        running_server.cancel().await;
    };

    sent_notification.await;
    Ok(())
}

#[async_std::test]
async fn close_notification_with_reason() -> std::io::Result<()> {
    env_logger::init();
    let original_close_reason = CloseReason::from(3);
    
    log::info!("close server");

    let running_server = async_std::task::spawn(async move {
        let close_reason = original_close_reason;
        server::start(close_after(close_reason, 10)).await.unwrap();
    });

    let sent_notification = async {
        async_std::task::sleep(Duration::from_millis(10)).await;

        Notification::new()
            .timeout(Timeout::Never)
            .show_async()
            .await
            .map(|handler| {
                handler.on_close(|received_reason: CloseReason| {
                    eprintln!("received close reason {received_reason:?}");
                    assert_eq!(received_reason, original_close_reason)
                })
            })
            .unwrap();
        log::info!("sent notification waiting for reason");

        running_server.cancel().await;
    };

    sent_notification.await;
    Ok(())
}
