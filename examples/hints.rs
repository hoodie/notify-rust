#![allow(unused_must_use)]
extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationUrgency::*;
use notify_rust::NotificationHint as Hint;
use notify_rust::NotificationImage as Image;

fn freeze(message:&str)
{
    println!("{}\n",message);
    let mut _devnull = String::new();
    let _ = std::io::stdin().read_line(&mut _devnull);
}

fn main ()
{
    println!("Pay close attention to the output of:");
    println!("$ dbus-monitor \"interface=org.freedesktop.Notifications\"");

    freeze("actionicons");
    Notification::new().hint(Hint::ActionIcons(true)).show();

    freeze("urgency: low, medium, high");
    Notification::new().hint(Hint::Urgency(Low)).show();
    Notification::new().hint(Hint::Urgency(Normal)).show();
    Notification::new().hint(Hint::Urgency(Critical)).show();

    freeze("category");
    Notification::new().hint(Hint::Category("device.removed".into())).show();

    freeze("DesktopEntry");
    Notification::new().hint(Hint::DesktopEntry("firefox".into())).show();

    freeze("ImagePath");
    Notification::new().hint(Hint::ImagePath("/usr/share/icons/hicolor/128x128/apps/firefox.png".into())).show();

    freeze("Resident");
    Notification::new().hint(Hint::Resident(true)).show();

    freeze("SoundFile");
    Notification::new().hint(Hint::SoundFile("/usr/share/sounds/alsa/Front_Left.wav".to_owned()))
                       .hint(Hint::SoundName("system sound".to_owned()))
                       .hint(Hint::SuppressSound(false)).show();

    freeze("Transient");
    Notification::new().hint(Hint::Transient(false)).show();

    freeze("X and Y");
    Notification::new().hint(Hint::X(200))
                       .hint(Hint::Y(200))
                       .show();

    freeze("ImageData");
    let mut image_data = vec![0;64*64*3];
    for i in 0..64*64*3 {
        image_data[i] = (i % 256) as u8;
    }
    Notification::new().hint(Hint::ImageData(Image::from_rgb(64,64,image_data).unwrap()))
                       .show();

    //freeze("Custom");
    //Notification::new().hint(Hint::Custom("foo","bar")).show();
}
