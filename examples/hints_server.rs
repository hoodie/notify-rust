#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
mod hint_server {

    use std::thread;
    use std::time::Duration;

    use notify_rust::server::NotificationServer;
    use notify_rust::Hint;
    use notify_rust::Notification;
    use notify_rust::Urgency::*;

    fn freeze(message: &str) {
        println!("{}", message);
        // let mut _devnull = String::new();
        // let _ = std::io::stdin().read_line(&mut _devnull);
    }

    pub fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let server = NotificationServer::create();
        // thread::spawn(move || NotificationServer::start(&server,|notification| println!(" -- {:#?} --", notification)));
        thread::spawn(move || {
            NotificationServer::start(&server, |notification| {
                println!(" --> {:?}\n", notification.hints)
            })
        });

        std::thread::sleep(Duration::from_millis(500));

        freeze("actionicons");
        Notification::new().hint(Hint::ActionIcons(true)).show()?;
        Notification::new().hint(Hint::ActionIcons(false)).show()?;

        freeze("urgency: low, medium, high");
        Notification::new().hint(Hint::Urgency(Low)).show()?;
        Notification::new().hint(Hint::Urgency(Normal)).show()?;
        Notification::new().hint(Hint::Urgency(Critical)).show()?;

        freeze("category");
        Notification::new()
            .hint(Hint::Category("device.removed".into()))
            .show()?;

        freeze("DesktopEntry");
        Notification::new()
            .hint(Hint::DesktopEntry("firefox".into()))
            .show()?;

        freeze("ImagePath");
        Notification::new()
            .hint(Hint::ImagePath(
                "/usr/share/icons/hicolor/128x128/apps/firefox.png".into(),
            ))
            .show()?;

        freeze("Resident");
        Notification::new().hint(Hint::Resident(true)).show()?;

        freeze("SoundFile");
        Notification::new()
            .hint(Hint::SoundFile(
                "/usr/share/sounds/alsa/Front_Left.wav".to_owned(),
            ))
            .hint(Hint::SoundName("system sound".to_owned()))
            .hint(Hint::SuppressSound(false))
            .show()?;

        freeze("Transient");
        Notification::new().hint(Hint::Transient(false)).show()?;

        freeze("X and Y");
        Notification::new()
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

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "macos")]
    println!("this is a xdg only feature");

    #[cfg(all(not(feature = "server"), unix, not(target_os = "macos")))]
    println!("please build with '--features=server'");

    #[cfg(all(feature = "server", unix, not(target_os = "macos")))]
    hint_server::main()?;

    Ok(())
}
