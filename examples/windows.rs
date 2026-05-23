extern crate notify_rust;
use notify_rust::Notification;

fn main() {
    let handle = Notification::new()
        .summary("Notify Rust Windows")
        .body("yay, we have limited windows support\nWith multiple lines\nSound\nImages")
        .action("open", "Open")
        .show()
        .unwrap();

    handle.wait_for_action(|action| {
        println!("action: {action}");
    });
}
