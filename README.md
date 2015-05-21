# notify_send-rs
Send DBus notifications, purely in rust,
using [dbus-rs](https://github.com/diwic/dbus-rs/).


```toml
#Cargo.toml
[dependencies.notify_send]
git = "https://github.com/hoodie/notify_send-rs.git"
```

```rust
#[macro_use] extern crate notify_send;
use notify_send::*;

fn main() {
  notify_send!("title1-t", t 5000);
  notify_send!("title1");
  notify_send!("title2", "with message");
  notify_send!("title3", "with message and icon", "dialog-ok");
  notify_send!("title4", "with message, icon and timeout", "dialog-ok", t 3000);
}
```
