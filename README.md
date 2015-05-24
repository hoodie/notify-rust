# notify_send-rs
Send DBus notifications, purely in rust,
using [dbus-rs](https://github.com/diwic/dbus-rs/).


```toml
#Cargo.toml
[dependencies.notify_send]
git = "https://github.com/hoodie/notify_send-rs.git"
```

```rust
extern crate notify_send;
use notify_send::*;
fn main()
{
    // use it this way
    Notification::new()
        .summary("this is the summary")
        .body("this is the body")
        .icon("firefox")
        .timeout(16)
        .send();


    //possible, but don't do this
    NotifyMessage {
        appname: "foobar".to_string(),
        timeout: 20,
        ..NotifyMessage::new()
    }.send();


}

```
