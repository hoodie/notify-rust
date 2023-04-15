#![cfg(feature = "server")]
use async_std::future::pending;
use notify_rust::{
    server::{self, NotificationHandler, ReceivedNotification},
    CloseReason, Notification, Timeout,
};
use std::time::Duration;

#[ctor::ctor]
fn init_color_backtrace() {
    color_backtrace::install();
    env_logger::init();
}

fn never_handle() -> impl NotificationHandler + Clone {
    move |received: ReceivedNotification| async move {
        eprintln!(
            "received notification (timeout: {}) I will never handle",
            received.timeout
        );
        pending::<()>().await;
    }
}

fn close_after(close_reason: CloseReason, sleep_ms: u64) -> impl NotificationHandler + Clone {
    move |received: ReceivedNotification| async move {
        eprintln!("received notification I will close with {close_reason:?}");
        async_std::task::sleep(Duration::from_millis(sleep_ms)).await;

        let (_action, closer) = received.channels().unwrap();
        closer.send(close_reason).await.unwrap();
        eprintln!("sent reason {close_reason:?}");
    }
}

#[async_std::test]
async fn expire_notification_after_default_timeout() -> std::io::Result<()> {
    let bus = "expire_notification_after_default_timeout";
    let running_server = async_std::task::spawn(async {
        server::start_at(bus, never_handle()).await.unwrap();
    });
    log::info!("expire server");

    let sent_notification = async {
        async_std::task::sleep(Duration::from_millis(10)).await;

        Notification::at_bus(bus)
            .unwrap()
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
    let bus = "close_notification_with_reason";
    let original_close_reason = CloseReason::from(3);

    log::info!("close server");

    let running_server = async_std::task::spawn(async move {
        let close_reason = original_close_reason;
        server::start_at(bus, close_after(close_reason, 10))
            .await
            .unwrap();
    });

    let sent_notification = async {
        async_std::task::sleep(Duration::from_millis(10)).await;

        Notification::at_bus(bus)
            .unwrap()
            .timeout(Timeout::Never)
            .show_async()
            .await
            .map(|handler| {
                handler.on_close(|received_reason: CloseReason| {
                    eprintln!("received close reason {received_reason:?}");
                    assert_eq!(dbg!(received_reason), original_close_reason)
                })
            })
            .unwrap();
        log::info!("sent notification waiting for reason");

        running_server.cancel().await;
    };

    sent_notification.await;
    Ok(())
}
