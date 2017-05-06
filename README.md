# notify-rust

[![license](https://img.shields.io/crates/l/notify-rust.svg)](https://crates.io/crates/notify-rust/)
[![version](https://img.shields.io/crates/v/notify-rust.svg)](https://crates.io/crates/notify-rust/)
[![documentation](https://docs.rs/notify-rust/badge.svg)](https://docs.rs/notify-rust/)
[![ci](https://travis-ci.org/hoodie/notify-rust.svg?branch=master)](https://travis-ci.org/hoodie/notify-rust)

Shows desktop notifications.
This implementation does not rely on libnotify, as it is using [dbus-rs](https://github.com/diwic/dbus-rs/).
Basic notification features are supported, more sophisticated functionality will follow.
The API shown below should be stable.


```toml
#Cargo.toml
[dependencies]
notify-rust = "3"
```

# Examples
## Example 1 (Simple Notification)
```rust
use notify_rust::Notification;
Notification::new()
    .summary("Firefox News")
    .body("This will almost look like a real firefox notification.")
    .icon("firefox")
    .show().unwrap();
```

## Example 2 (Persistent Notification)
```rust
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;
Notification::new()
    .summary("Category:email")
    .body("This has nothing to do with emails.\nIt should not go away until you acknoledge it.")
    .icon("thunderbird")
    .appname("thunderbird")
    .hint(Hint::Category("email".to_owned()))
    .hint(Hint::Resident(true)) // this is not supported by all implementations
    .timeout(0) // this however is
    .show().unwrap();
```
## Usage & Documentation
Please see the [documentation](http://hoodie.github.io/notify-rust/) for current examples.

### Commandline tool
Checkout [toastify](https://github.com/hoodie/toastify), it exposes most of the functionality of the lib to the commandline.

### Platform support

This library shines on linux and bsd, which is it's original target platform. Lately it gained support for macOS, however this only includes a small subset of the current functionality, since [`NSNotification`](https://developer.apple.com/reference/foundation/nsnotification)s don't have as many features. Please refer to the You are a versed macOS UI developer with mad Objective-C skillz? <abbr title="pull request sil vous plait">PRSV</abbr>.
