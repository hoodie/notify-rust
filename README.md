# notify_send-rs

[![Build Status](https://travis-ci.org/hoodie/notify_send-rs.png)](https://travis-ci.org/hoodie/notify_send-rs)

Send DBus notifications, purely in rust,
using [dbus-rs](https://github.com/diwic/dbus-rs/).


```toml
#Cargo.toml
[dependencies.notify_send]
git = "https://github.com/hoodie/notify_send-rs.git"
```
## Usage

```rust
extern crate notify_send;
use notify_send::Notification;
fn main()
{
    // use it this way
    Notification::new()
        .summary("this is the summary")
        .body("this is the body")
        .icon("firefox")
        .timeout(6000) //miliseconds
        .send();

    // using format!()
    Notification::new()
        .summary("Chromium Crashed")
        .appname("chromium")
        .body(&format!("This is <b>{}</b>!<br/>", "a lie"))
        .icon("chromium")
        .timeout(6000) //miliseconds
        .send();

    // possible, if you like
    let mut message = Notification::new();
    message.summary("invocation type 2");
    message.body("your <b>body</b> is a <u>wonderland</u>");
    message.send();

    // also possible, if you really really want to
    Notification {
        appname: "foobar".to_string(),
        timeout: 20,
        ..Notification::new()
    }.send();



    // can't see anything?
    Notification::new().summary("this will also print to stdout").send_debug();


}

```

## things to do

* [ ] actions
* [ ] hints
* [ ] make use of returned id
* [ ] GetCapabilities(), CloseNotification(), GetServerInformation()

checkout [the spec](http://www.galago-project.org/specs/notification/0.9/index.html)
