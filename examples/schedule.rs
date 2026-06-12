#![allow(unused_imports)]
mod common;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cfg_if::cfg_if! {
        if #[cfg(target_os = "macos")] {
            if !common::setup(file!()) {
                return Ok(());
            }
            use notify_rust::Notification;

            let duration = chrono::Duration::milliseconds(4321);
            let timestamp = (chrono::Utc::now() + duration).timestamp() as f64;

            Notification::new()
                .summary("Oh by the way")
                .body(&format!("this was scheduled {:?} ago", duration))
                .schedule(chrono::Utc::now() + duration)?;

            Notification::new()
                .summary("Oh by the way")
                .body(&format!("this was scheduled for timestamp {}", timestamp))
                .schedule_raw(timestamp)?;

            Ok(())
        } else {
            println!("this is a mac only feature");
            Ok(())
        }
    }
}
