#![cfg(feature = "server")]
#![allow(unused_must_use)]
#![cfg(all(unix, not(target_os = "macos")))]

mod ownworld {
    use std::time::Duration;

    use notify_rust::{
        server::{self, Action, ReceivedNotification},
        Notification, Timeout, Hint,
    };
    use ntest::timeout;

    #[ctor::ctor]
    fn init_color_backtrace() {
        color_backtrace::install();
        std::env::set_var("RUST_LOG", "ownworld=debug,notify_rust=trace");

        use env_logger::{Builder, WriteStyle};

        Builder::from_default_env()
            // .format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
            .format_timestamp_millis()
            .write_style(WriteStyle::Always)
            .init();
    }

    #[async_std::test]
    #[timeout(500)]
    async fn actions_vec() {
        let bus = "actions_vec";
        let mut running_server =
            server::start_at(bus, move |notification: ReceivedNotification| async move {
                log::debug!("notification received {:#?}", notification);
                assert_eq!(notification.actions[0], Action::from_single("actions_vec0"));
                assert_eq!(notification.actions[1], Action::from_single("actions_vec1"));
                assert_eq!(notification.actions[2], Action::from_single("actions_vec2"));
                assert_eq!(notification.actions[3], Action::from_single("actions_vec3"));
                // TODO: remove requirement to sleep here!
                // sleeping a little to give the `NotificationHandler` time to
                // start listening for my close signal before I send it
                async_std::task::sleep(Duration::from_millis(10)).await;
                let (_action, closer) = notification.channels().unwrap();
                closer
                    .send(notify_rust::CloseReason::CloseAction)
                    .await
                    .unwrap();
                log::debug!("close sent");
                async_std::task::sleep(Duration::from_millis(10)).await;
                log::debug!("handler finished");
            })
            .await
            .unwrap();
        log::debug!("server running");

        #[allow(deprecated)]
        let notification_sent = Notification::at_bus(bus)
            .unwrap()
            .summary("Notification with actions")
            .body("action1=\"Action One\", something_else=\"Something Else\"")
            .icon("dialog-information")
            .timeout(Duration::from_secs(2))
            .actions(vec![
                "actions_vec0".into(), "actions_vec0".into(),
                "actions_vec1".into(), "actions_vec1".into(),
                "actions_vec2".into(), "actions_vec2".into(),
                "actions_vec3".into(), "actions_vec3".into(),
            ])
            .show_async()
            .await
            .unwrap_or_else(|_| panic!("should have been sent to {bus}"));
        log::debug!("notification sent");

        log::info!("waiting for close");
        async_std::task::sleep(Duration::from_millis(10)).await;
        notification_sent.closed().await;
        log::info!("waiting for close, done!");

        running_server.stop();
        log::debug!("stopped");
    }

    #[async_std::test]
    #[timeout(5000)]
    async fn multiple_actions() {
        let bus = "multiple_actions";
        let mut running_server =
            server::start_at(bus, move |notification: ReceivedNotification| async move {
                log::debug!("notification received");
                assert_eq!(notification.actions[0], Action::new("the one", "one"));
                assert_eq!(notification.actions[1], Action::new("the two", "two"));
                // TODO: remove requirement to sleep here!
                // sleeping a little to give the `NotificationHandler` time to
                // start listening for my close signal before I send it

                async_std::task::sleep(Duration::from_millis(100)).await;
                let (action, closer) = notification.channels().unwrap();

                async_std::task::sleep(Duration::from_millis(100)).await;
                action.send("one".into()).await.unwrap();
                log::debug!("action one sent");

                async_std::task::sleep(Duration::from_millis(100)).await;
                dbg!(action.send("two".into()).await).unwrap();
                log::debug!("action two sent");

                async_std::task::sleep(Duration::from_millis(1000)).await;
                closer
                    .send(notify_rust::CloseReason::CloseAction)
                    .await
                    .unwrap();
                log::debug!("close sent");

                async_std::task::sleep(Duration::from_millis(1000)).await;
                log::debug!("handler finished");
            })
            .await
            .unwrap();
        log::debug!("server running");

        let notification_sent = Notification::at_bus(bus)
            .unwrap()
            .summary("Notification with actions")
            .body("action1=\"Action One\", something_else=\"Something Else\"")
            .icon("dialog-information")
            .hint(Hint::Resident(true))
            .timeout(Duration::from_secs(2))
            .action("the one", "one")
            .action("the two", "two")
            .show_async()
            .await
            .unwrap_or_else(|_| panic!("should have been sent to {bus}"));
        log::debug!("notification sent");

        log::info!("waiting for actions");
        // TODO: add asserts
        // TODO: remove clone
        // wait for "one"
        notification_sent
            .clone()
            .wait_for_action(|action| log::info!("action called {action}"));

        // wait for "two"
        notification_sent
            .clone()
            .wait_for_action(|action| log::info!("action called {action}"));

        log::info!("waiting for close");
        async_std::task::sleep(Duration::from_millis(10)).await;
        notification_sent.closed().await;
        log::info!("waiting for close, done!");

        running_server.stop();
        log::debug!("stopped");
    }

    #[async_std::test]
    #[timeout(500)]
    async fn actions_automatic() {
        let bus = "actions_automatic";
        let running_server = async_std::task::spawn(async move {
            // FIXME: panicking in here has no back channel
            server::start_at(bus, move |notification: ReceivedNotification| async move {
                assert_eq!(notification.actions[0], Action::from_single("actions_vec2"));
                assert_eq!(notification.actions[1], Action::from_single("actions_vec1"));
                assert_eq!(notification.actions[2], Action::from_single("actions_vec2"));
                assert_eq!(notification.actions[3], Action::from_single("actions_vec3"));
                assert_eq!(notification.timeout, Timeout::Milliseconds(6000).into());
            })
            .await
            .unwrap()
        });

        Notification::at_bus(bus)
            .unwrap()
            .summary("Another notification with actions")
            .body("action0=\"Press me please\", action1=\"firefox\"")
            .icon("dialog-information")
            .timeout(6000) //miliseconds
            .action("actions_built0", "actions_built1")
            .action("actions_built2", "actions_built3")
            .show_async()
            .await;
        running_server.cancel().await;
    }
}
