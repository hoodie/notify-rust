#![allow(unused_imports)]
mod common;

fn main() {
    cfg_if::cfg_if! {
        if #[cfg(any(
            all(unix, not(target_os = "macos")),
            all(unix, all(target_os = "macos", feature = "preview-macos-un"))
        ))] {
        use notify_rust::{Hint, Notification, Timeout};
        common::setup("actions");

        Notification::new()
            .summary("click me")
            .body("This will disappear by itself")
            .action("clicked_a", "button a") // IDENTIFIER, LABEL
            .show()
            .unwrap()
            .wait_for_action(|action| match action {
                "clicked_a" => log::info!("clicked a"),
                // FIXME: here "__closed" is a hardcoded keyword, it will be deprecated!!
                "__closed" => log::info!("the notification was closed"),
                _ => (),
            });

        Notification::new()
            .summary("click me")
            .body("This action needs to be clicked")
            .action("default", "default") // IDENTIFIER, LABEL
            .action("clicked_a", "button a") // IDENTIFIER, LABEL
            .action("clicked_b", "button b") // IDENTIFIER, LABEL
            .timeout(Timeout::Never) // works on kde and gnome
            .show()
            .unwrap()
            .wait_for_action(|action| match action {
                "default" => log::info!("default"),
                "clicked_a" => log::info!("clicked a"),
                "clicked_b" => log::info!("clicked b"),
                // FIXME: here "__closed" is a hardcoded keyword, it will be deprecated!!
                "__closed" => log::info!("the notification was closed"),
                _ => (),
            });

    } else {
        println!("this is a xdg/macos only feature"); }
    }
}
