use notify_rust::Notification;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Notification::new().summary("minimal notification").show()?;
    Ok(())
}
