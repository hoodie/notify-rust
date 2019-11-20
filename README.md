<div align="center">

# notify-rust

[![Travis](https://img.shields.io/travis/hoodie/notify-rust.svg)](https://travis-ci.org/hoodie/notify-rust/)
[![license](https://img.shields.io/crates/l/notify-rust.svg)](https://crates.io/crates/notify-rust/)
[![Crates.io](https://img.shields.io/crates/d/notify-rust.svg)](https://crates.io/crates/notify-rust)
[![version](https://img.shields.io/crates/v/notify-rust.svg)](https://crates.io/crates/notify-rust/)
[![documentation](https://docs.rs/notify-rust/badge.svg)](https://docs.rs/notify-rust/)
![maintenance](https://img.shields.io/maintenance/yes/2021)
[![contributors](https://img.shields.io/github/contributors/hoodie/notify-rust)](https://github.com/hoodie/notify-rust/graphs/contributors)
</div>

A not so well-named library for displaying desktop notifications.
On linux/bsd it uses [dbus-rs](https://github.com/diwic/dbus-rs/), so it does not rely on libnotify.
On macos it uses [mac-notification-sys](https://github.com/h4llow3En/mac-notification-sys/).


```toml
[dependencies]
notify-rust = "3"
```

# Requirements

* `rustc` >= 1.32
* linux, with `libdbus` (see [dbus-rs](https://github.com/diwic/dbus-rs#requirements))
* macos
* no windows support, yet

# Examples

## Example 1 (Simple Notification)
```rust
use notify_rust::Notification;
Notification::new()
    .summary("Firefox News")
    .body("This will almost look like a real firefox notification.")
    .icon("firefox")
    .show()?;
```

## Example 2 (Persistent Notification)
```rust
use notify_rust::Notification;
use notify_rust::Hint;
Notification::new()
    .summary("Category:email")
    .body("This has nothing to do with emails.\nIt should not go away until you acknowledge it.")
    .icon("thunderbird")
    .appname("thunderbird")
    .hint(Hint::Category("email".to_owned()))
    .hint(Hint::Resident(true)) // this is not supported by all implementations
    .timeout(0) // this however is
    .show()?;
```
## Usage & Documentation
Please see the [documentation](https://docs.rs/crate/notify-rust/) for current examples.

### Commandline tool
Checkout [toastify](https://github.com/hoodie/toastify), it exposes most of the functionality of the lib to the commandline.

### macOS support

This library shines on linux and bsd, which is its original target platform.
Lately it gained support for macOS thanks to [mac-notification-sys](https://crates.io/crates/mac-notification-sys).
However this only includes a small subset of the current functionality, since [`NSNotification`](https://developer.apple.com/reference/foundation/nsnotification)s don't have as many features. Please refer to the You are a versed macOS UI developer with mad Objective-C skillz? <abbr title="pull request sil vous plait">PRSV</abbr>.

## Contribution

Any help in form of descriptive and friendly [issues](https://github.com/hoodie/notify-rust/issues) or comprehensive pull requests are welcome! 


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in notify-rust by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
