mod common;

fn main() {
    use notify_rust::{Notification, Timeout};
    common::setup(file!());

    Notification::new()
        .summary("click me")
        .body("This will disappear by itself")
        .action("clicked_a", "action") // IDENTIFIER, LABEL
        // .hint(notify_rust::Hint::Transient(true)) // needed to work on kde
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
}
