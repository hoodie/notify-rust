cfg_if::cfg_if! {

if #[cfg(
    any(
        all(target_os = "macos", not(feature = "preview-macos-un")),
        target_os = "windows"
    )
)]{
    fn main() {
        println!("this is a xdg only feature")
    }

} else if #[cfg( unix )]{

    use std::thread;

    use notify_rust::{CloseReason, Notification};
    mod common;

    fn print_reason(reason: CloseReason) {
        println!("notification was closed ({:?})", reason);
    }
    fn main() {
        if !common::setup("on_close") {
            return;
        }

        thread::spawn(|| {
            Notification::new()
                .summary("Time is running out")
                .body("This will go away.")
                .icon("clock")
                .show()
                .map(|handler| handler.on_close(print_reason))
        });
        common::wait_for_keypress("close the notification and then press any key to exit");
    }
}
}
