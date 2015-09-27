# notify-rust

[![Build Status](https://img.shields.io/travis/hoodie/notify-rust.svg)](https://travis-ci.org/hoodie/notify-rust)
[![license](https://img.shields.io/crates/l/notify-rust.svg)](https://crates.io/crates/notify-rust/)
[![version](https://img.shields.io/crates/v/notify-rust.svg)](https://crates.io/crates/notify-rust/)

Shows desktop notifications.
This implementation does not rely on libnotify, as it is using [dbus-rs](https://github.com/diwic/dbus-rs/).
Basic notification features are supported, more sophisticated functionality will follow.
The API shown below should be stable.


```toml
#Cargo.toml
[dependencies]
notify-rust = "2.1"
```

## Usage & Documentation
please see the [documentation](http://hoodie.github.io/notify-rust/) for current examples.

### Commandline tool
Checkout ./notify-cli, it exposes most of the functionality of the lib to the commandline.

```
$ ./target/debug/notify send --help
notify-send 
Shows a notification

USAGE:
	notify send [FLAGS] [OPTIONS] [ARGS] <summary>

FLAGS:
    -d, --debug      Also prints notification to stdout
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --app-name <app-name>          Set a specific app-name manually.
    -c, --category <category>          Set a category.
    -t, --expire-time <expire-time>    Time until expiration in milliseconds. 0 means forever.
    -i, --icon <icon>                  Icon of notification.
    -u, --urgency <urgency>            How urgent is it. [values: high low normal]

ARGS:
    summary    Title of the Notification.
    body       Message body

```
