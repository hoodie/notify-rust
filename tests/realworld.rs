#![allow(unused_must_use)]
#![cfg(all(unix, not(target_os = "macos")))]
extern crate notify_rust;

#[cfg(test)]
mod realworld{

use notify_rust::*;
use notify_rust::NotificationHint as Hint;
use notify_rust::NotificationUrgency::*;
use notify_rust::NotificationImage as Image;

#[test]
fn burst()
{
    for msg in &[
        "These should each",
        "come in their own pop up.",
        "If they don't than",
        "I will have to complain about it."
    ]{
        assert!(
        Notification::new()
            .summary("burst")
            .appname(&msg)
            .body(&msg)
            .icon("media-floppy")
            .show()
            .is_ok());
    }

    for msg in &[
        "These may be grouped",
        "together by the server.",
        "that is because the all have the same",
        "appname."
    ]{
        assert!(
        Notification::new()
            .summary("merged burst")
            .body(&msg)
            .icon("applications-toys")
            .show()
            .is_ok());
    }
}

#[test]
fn closing()
{
    Notification::new()
        .summary("You see me")
        .body("you don't see me!")
        .show()
        .unwrap()
        .close();
}

#[test]
fn capabilities()
{
    let capabilities:Vec<String> = get_capabilities().unwrap();
    for capability in capabilities{
        assert!(Notification::new().summary("capability").body(&capability).show().is_ok());
    }
}

#[test]
fn build_pattern()
{
    let notification = Notification::new().summary("foo").finalize();
    assert_eq!(notification.summary, "foo");

    let mut notification = Notification::new();
    notification.body = "foo".to_string();
    assert_eq!(notification.body, "foo");

    let mut notification = Notification::new();
    notification.icon = "foo".to_string();
    assert_eq!(notification.icon, "foo");

    let mut notification = Notification::new();
    notification.summary = "foo".to_string();
    assert_eq!(notification.summary, "foo");

    let mut notification = Notification::new();
    notification.timeout = Timeout::Milliseconds(42);
    assert_eq!(notification.timeout, Timeout::Milliseconds(21*2));

    let mut notification = Notification::new();
    notification.summary = "foo".to_string();
    assert_eq!(notification.summary, "foo");
}

#[test]
fn init()
{
    let mut message = Notification::new();
    message.summary("invocation type 2");
    message.body("your <b>body</b> is a <u>wonderland</u>");
    message.show().unwrap();

    Notification::new()
        .summary("this is the summary")
        .summary("invocation type 3")
        .body("this is the body\nnewline<br/>linebreak")
        .show().unwrap();
}

#[test]
fn urgency()
{
    // use it this way
    for urgency in &[
        Hint::Urgency(Low),
        Hint::Urgency(Normal),
        Hint::Urgency(Critical)
    ]{
        assert!(
        Notification::new()
            .summary(&format!("Urgency {:?}", urgency))
            .hint(urgency.clone())
            .show().is_ok());
    }
}

#[test]
fn category()
{
    assert!(
    Notification::new()
        .appname("thunderbird")
        .summary("Category:email")
        .icon("thunderbird")
        .hint(Hint::Category("email".to_string()))
        .show().is_ok());
}

#[test]
fn persistent() {

    assert!(
    Notification::new()
        .summary("Incoming Call: Your Mom!")
        .body("Resident:True")
        .icon("call-start")
        .hint(Hint::Resident(true))
        .show().is_ok());

    assert!(
    Notification::new()
        .summary("Incoming Call: Your Mom!")
        .body("Resident:False, but Timeout=0")
        .icon("call-start")
        .hint(Hint::Resident(false))
        .timeout(0)
        .show().is_ok());

}

#[test]
fn imagedata() {

    let mut data : Vec<u8> = vec![0 as u8; 64*64*3];
    for x in 0..64 {
        for y in 0..64 {
            let offset = (y * 64 + x) * 3;
            data[ offset ] = (x + y) as u8;
            data[ offset + 1 ] = x as u8;
            data[ offset + 2 ] = y as u8;
        }
    }
    assert!(
    Notification::new()
        .summary("I can haz image data!")
        .hint(Hint::ImageData(Image::from_rgb(64,64,data).unwrap()))
        .show().is_ok());
}

}
