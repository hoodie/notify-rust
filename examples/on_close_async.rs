#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is an xdg only feature")
}

fn print() {
    println!("notification was closed, don't know why");
}
fn print2() {
    println!("this is an extra callback");
}

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use notify_rust::Notification;
    async_std::task::spawn(|| {
        Notification::new()
            .summary("Time is running out")
            .body("This will go away.")
            .icon("clock")
            .show()
            .map(|handler| {
                handler.on_close_async(print);
                handler.on_close_async(print2);
            })
    });
    Ok(())
}
