extern crate notify_rust;
extern crate winrt;
use notify_rust::Notification;

fn main() {

    let rt = winrt::RuntimeContext::init();
    Notification::new()
        .summary("Notify Rust Windows")
        .appname("notify-rust windows")
        .body("yay, we have windows support")
        .icon("firefox")
        .show()
        .unwrap();
    rt.uninit();

}
