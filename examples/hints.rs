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

    freeze("ImageData");
    let mut image_data = vec![0;128*128*3];
    for i in 0..128*128*3 {
        image_data[i] = (i % 256) as u8;
    }
    Notification::new().hint(Hint::ImageData(Image::from_rgb(128,128,image_data).unwrap()))
                       .summary("You should see stripes in this notification")
                       .show();

    //freeze("Custom");
    //Notification::new().hint(Hint::Custom("foo","bar")).show();
}
