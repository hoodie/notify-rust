#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
mod hint_server {

    use std::thread;
    use std::time::Duration;

    use notify_rust::server::ReceivedNotification;
    use notify_rust::Urgency::*;
    use notify_rust::{Hint, Notification};

    fn freeze(message: &str) {
        println!("{}", message);
        // let mut _devnull = String::new();
        // let _ = std::io::stdin().read_line(&mut _devnull);
    }

    pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
        // thread::spawn(move || NotificationServer::start(&server,|notification| println!(" -- {:#?} --", notification)));
        async_std::task::spawn(notify_rust::server::start_at(
            "example",
            |notification: ReceivedNotification| async move {
                println!(" --> {:?}\n", notification.hints);
            },
        ));

        std::thread::sleep(Duration::from_millis(500));

        freeze("actionicons");
        Notification::at_bus("example")?
            .hint(Hint::ActionIcons(true))
            .show()?;
        Notification::at_bus("example")?
            .hint(Hint::ActionIcons(false))
            .show()?;

        freeze("urgency: low, medium, high");
        Notification::at_bus("example")?
            .hint(Hint::Urgency(Low))
            .show()?;
        Notification::at_bus("example")?
            .hint(Hint::Urgency(Normal))
            .show()?;
        Notification::at_bus("example")?
            .hint(Hint::Urgency(Critical))
            .show()?;

        freeze("category");
        Notification::at_bus("example")?
            .hint(Hint::Category("device.removed".into()))
            .show()?;

        freeze("DesktopEntry");
        Notification::at_bus("example")?
            .hint(Hint::DesktopEntry("firefox".into()))
            .show()?;

        freeze("ImagePath");
        Notification::at_bus("example")?
            .hint(Hint::ImagePath(
                "/usr/share/icons/hicolor/128x128/apps/firefox.png".into(),
            ))
            .show()?;

        freeze("Resident");
        Notification::at_bus("example")?
            .hint(Hint::Resident(true))
            .show()?;

        freeze("SoundFile");
        Notification::at_bus("example")?
            .hint(Hint::SoundFile(
                "/usr/share/sounds/alsa/Front_Left.wav".to_owned(),
            ))
            .hint(Hint::SoundName("system sound".to_owned()))
            .hint(Hint::SuppressSound(false))
            .show()?;

        freeze("Transient");
        Notification::at_bus("example")?
            .hint(Hint::Transient(false))
            .show()?;

        freeze("X and Y");
        Notification::at_bus("example")?
            .hint(Hint::X(200))
            .hint(Hint::Y(200))
            .show()?;

        // println!("Press enter to exit.\n");
        // let mut _devnull = String::new();
        // let _ = std::io::stdin().read_line(&mut _devnull);
        // println!("Thank you for choosing notify-rust.");

        Ok(())
    }
}

#[async_std::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    println!("this is a xdg only feature");

    #[cfg(all(not(feature = "server"), unix, not(target_os = "macos")))]
    println!("please build with '--features=server'");

    #[cfg(all(feature = "server", unix, not(target_os = "macos")))]
    hint_server::main().await?;

    Ok(())
}
