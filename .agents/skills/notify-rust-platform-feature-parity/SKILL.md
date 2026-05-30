---
name: notify-rust-platform-feature-parity
description: model macos and windows apis after existing zbus/dbus apis
disable-model-invocation: false
---

The implementation of notify-rust was originally for linux (dbus, xdg notifications) therefore the linux implementation is the most complete and comprehensive. When adding new apis for the macos and windows configuration they are to be modelled strictly after the linux implementation so that the behavior and function signature are reusable across platforms.
