# notify-rust

[![Build Status](https://travis-ci.org/hoodie/notify-rust.svg?branch=master)](https://travis-ci.org/hoodie/notify-rust)

Shows desktop notifications.
This implementation does not rely on libnotify, as it is using [dbus-rs](https://github.com/diwic/dbus-rs/).
Basic notification features are supported, more sophisticated functionality will follow.
The API shown below should be stable.


```toml
#Cargo.toml
[dependencies]
notify-rust = "0.0.5"
```
## Usage

```rust
extern crate notify_rust;
use notify_rust::Notification;
fn main()
{
    // use it this way
    Notification::new()
        .summary("this is the summary")
        .body("this is the body")
        .icon("firefox")
        .timeout(6000) //miliseconds
        .show();

    // using format!()
    Notification::new()
        .summary("Chromium Crashed")
        .appname("chromium")
        .body(&format!("This is <b>{}</b>!<br/>", "a lie"))
        .icon("chromium")
        .timeout(6000) //miliseconds
        .show();

    // possible, if you like
    let mut message = Notification::new();
    message.summary("invocation type 2");
    message.body("your <b>body</b> is a <u>wonderland</u>");
    message.show();

    // also possible, if you really really want to
    Notification {
        appname: "foobar".to_string(),
        timeout: 20,
        ..Notification::new()
    }.show();



    // can't see anything?
    Notification::new().summary("this will also print to stdout").show_debug();


}
```

## Documentation
http://hoodie.github.io/notify-rust/

## things to do

* [x] actions
* [x] hints
* [ ] make use of returned id
* [ ] GetCapabilities(), CloseNotification(), GetServerInformation()
* [ ] become good enough to make this [list](https://wiki.archlinux.org/index.php/Desktop_notifications#Usage_in_programming)

[check](http://www.galago-project.org/specs/notification/0.9/index.html)
[out](https://developer.gnome.org/notification-spec/)
[the](https://wiki.ubuntu.com/NotifyOSD)
[specifications](https://wiki.archlinux.org/index.php/Desktop_notifications)
