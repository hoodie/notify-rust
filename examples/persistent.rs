use notify_rust::*;

#[cfg(target_os = "macos")]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(windows)]
fn main() {
    println!("this is a xdg only feature")
}


#[cfg(all(unix, not(target_os = "macos")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Notification::new()
        .summary("Persistent notification")
        .body("This should not go away unless you want it to.")
        .icon("firefox")
        .hint(Hint::Resident(true)) // does not work on kde
        .timeout(Timeout::Never) // works on kde and gnome
        .show()?;
    Ok(())
}
