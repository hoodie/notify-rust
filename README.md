<div align="center">

# notify-rust

[![Crates.io](https://img.shields.io/crates/d/notify-rust)](https://crates.io/crates/notify-rust)
[![contributors](https://img.shields.io/github/contributors/hoodie/notify-rust)](https://github.com/hoodie/notify-rust/graphs/contributors)
![maintenance](https://img.shields.io/maintenance/yes/2021)

[![version](https://img.shields.io/crates/v/notify-rust)](https://crates.io/crates/notify-rust/)
[![documentation](https://img.shields.io/badge/docs-latest-blue.svg)](https://docs.rs/notify-rust/)
[![license](https://img.shields.io/crates/l/notify-rust.svg?style=flat)](https://crates.io/crates/notify-rust/)

</div>

A not so well-named library for displaying desktop notifications.


```toml
[dependencies]
notify-rust = "4"
```

## Usage & Documentation
Please see the [documentation](https://docs.rs/crate/notify-rust/) for current examples.


### Simple Notification
```rust
use notify_rust::Notification;
Notification::new()
    .summary("Firefox News")
    .body("This will almost look like a real firefox notification.")
    .icon("firefox")
    .show()?;
```

### Persistent Notification
```rust
use notify_rust::{Notification, Hint};
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
### Commandline tool
Checkout [toastify](https://github.com/hoodie/toastify), it exposes most of the functionality of the lib to the commandline.

## Requirements

* `rustc` >= 1.44
* libdbus (linux)

### macOS support
This library shines on linux and bsd, which is its original target platform.
Lately it gained support for macOS thanks to [mac-notification-sys](https://crates.io/crates/mac-notification-sys).
However this only includes a small subset of the current functionality, since [`NSNotification`](https://developer.apple.com/reference/foundation/nsnotification)s don't have as many features. Please refer to the You are a versed macOS UI developer with mad Objective-C skillz? <abbr title="pull request sil vous plait">PRSV</abbr>.

### windows support
Similar to macOS we support windows via the help of [winrt-notification](https://crates.io/crates/winrt-notification).

## Contribution
Any help in form of descriptive and friendly [issues](https://github.com/hoodie/notify-rust/issues) or comprehensive pull requests are welcome! 


Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in notify-rust by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

### Conventions
The Changelog of this library is generated from its commit log, there any commit message must conform with https://www.conventionalcommits.org/en/v1.0.0/. For simplicity you could make your commits with [convco](https://crates.io/crates/convco).