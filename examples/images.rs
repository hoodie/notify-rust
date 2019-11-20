#![allow(unused_imports)]

use notify_rust::Notification;
use notify_rust::Hint as Hint;
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use notify_rust::Image as Image;

#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(not(feature = "images"), unix, not(target_os = "macos")))]
fn main() {
    println!("please build with '--features=images'")
}

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image_data = || {
        let mut image_data = vec![0; 128 * 128 * 3];
        for i in 0..128 * 128 * 3 {
            image_data[i] = (i % 256) as u8;
        }
        image_data
    };

    Notification::new()
        .summary("Generated Image (.hint())")
        .body("You should see stripes in this notification")
        .hint(Hint::ImageData(Image::from_rgb(128, 128, image_data())?))
        .show()?;

    Notification::new()
        .summary("Generated Image (.image_data())")
        .body("You should see stripes in this notification")
        .image_data(Image::from_rgb(128, 128, image_data())?)
        .show()?;

    Notification::new()
        .summary(".image()")
        .body("Trying to open an image")
        .image("./examples/octodex.jpg")?
        .show()?;

    Notification::new()
        .summary(".image_path()")
        .body("Trying to open an image")
        .image_path("./examples/octodex.jpg")
        .show()?;

    Ok(())
}
