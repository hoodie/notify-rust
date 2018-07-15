#![allow(unused_imports)]
extern crate notify_rust;

use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use notify_rust::NotificationImage as Image;


#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(not(feature = "images"), unix, not(target_os = "macos")))]
fn main() {
    println!("please build with '--features=images'")
}

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
fn main() {
    let mut image_data = vec![0; 128 * 128 * 3];
    for i in 0..128 * 128 * 3 {
        image_data[i] = (i % 256) as u8;
    }

    Notification::new()
        .summary("Generated Image")
        .body("You should see stripes in this notification")
        //.hint(Hint::ImageData(Image::from_rgb(128,128,image_data).unwrap()))
        .image_data(Image::from_rgb(128,128,image_data).unwrap())
        .show()
        .unwrap();

    Notification::new()
        .summary("Images")
        .body("Trying to open an image")
        .image("./examples/octodex.jpg")
        //.image_path("./examples/octodex.jpg")
        .show()
        .unwrap();
}
