# notify-rust

[![Build Status](https://travis-ci.org/hoodie/notify-rust.svg?branch=master)](https://travis-ci.org/hoodie/notify-rust)

Shows desktop notifications.
This implementation does not rely on libnotify, as it is using [dbus-rs](https://github.com/diwic/dbus-rs/).
Basic notification features are supported, more sophisticated functionality will follow.
The API shown below should be stable.


```toml
#Cargo.toml
[dependencies]
notify-rust = "0.8.0"
```

## Usage & Documentation
please see the [documentation](http://hoodie.github.io/notify-rust/) for current examples.

## Things TODO

* [x] actions
* [x] hints
* [x] make use of returned id (can be used by `close_notification(id)`)
* [x] implement methods: `GetServerInformation()`
* [x] listen to signals: `ActionInvoke` (0.8.0)
* [ ] become good enough to make this [list](https://wiki.archlinux.org/index.php/Desktop_notifications#Usage_in_programming)

[check](http://www.galago-project.org/specs/notification/0.9/index.html)
[out](https://developer.gnome.org/notification-spec/)
[the](https://wiki.ubuntu.com/NotifyOSD)
[specifications](https://wiki.archlinux.org/index.php/Desktop_notifications)
