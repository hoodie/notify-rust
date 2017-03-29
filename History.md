
3.3.0 / 2017-03-30
==================

  * builds on macOS, no full feature set though
  * added an extra type for timeout
  * deprecated .actions
  * examples compile on macOS
  * a little more macOS documentation
  * Merge branch 'feature/macos_notifications'

3.2.0 / 2016-08-07
==================

  * documentation: more examples
  * added fn auto_icon()
  * added on_close(F) method to NotificationHandle
  * `.show(&mut self)` -> `.show(&self)`
  * added use of id when sending
  * pubbed NotificaionHint const strings and added hint_from_key_val
  * more examples
  * cleanups

3.1.0 / 2016-03-01
==================
  * added custom int NotificationHint
    this enables things like [volume bars](examples/show_volume.rs) on notify-osd

  * made clippy happy
  * removed redundant example
  * info example print a valid yaml
  * updated examples
  * added examples for custom_int and reuse of notifications
  * Merge pull request #15 from illegalprime/master
  * updated examples to current api

3.0.0 / 2015-10-01
==================

  * new: better error handling: `show()` now returns a result instead of panicking
  * added: server deserializes notifications
  * fixed: hint types other than just strings
  * added: convenience from::<str>() for Urgency
  * added: Urgency::from(&str) for convenience
  * renamed urgencies: Low, Normal and Critical (as in both standards)
  * Tidying up
  * more documentation
  * added: internal `stop_server()`
  * fixed: urgency in example
  * tiny things, better looking Rust


2.1.0 / 2015-09-27
==================

  * fixed: Hint Types are not only strings

2.0.0 / 2015-08-04
==================

  * fixed: example code to match new api
  * added: Implemented update, close, etc. for NotificationHandle
  * removed: old API
  * update: examples and tests
  * Set transient hint in `wait_for_closing` example


1.1.0 / 2015-08-03
==================

  * added: `NotificationHandle` to keep dbus connections alive
  * added: notify-cli as child project
  * added: GetCapabilities() and `GetServerinformation()` to server
  * added: a few possible panic messages
  * small refactoring

1.0.1 / 2015-07-19
==================

  * fixed: actions and hints were not passed along

1.0.0 / 2015-07-01
==================

  * added: hints to `show_debug()`
  * added: urgency to `show_debug()`
  * changed: building agains dbus v0.1.1
  * improved: documentation

0.9.0 / 2015-07-01
==================

  * added: updating notifications ( see examples/update.rs )

0.0.8 / 2015-06-19
==================

  * added: can listen for ActionInvoked signals with `show_and_wait_for_action(Fn(actionname:&str))`
  * added: `"__closed"` action that is 'invoked' when the Notification closes

0.0.7 / 2015-06-13
==================

  * added: `close_notification()`
  * added: `get_server_information()`
  * changed: Urgency is now an enum
  * changed: pack methods return actual empty arrays now (builds with 1.0.0)
  * changed: moved to dbus-rs 0.1.0
  * changed: moved examples into tests
  * changed: made `get_capabilities()` global
  * changed: made `exe_name()` private
  * elaborated documentation

0.0.6 / 2015-06-08
==================

  * added: my own gh-pages branch as submodule as branch as submodule
  * added: note about Notification::hint()
  * added: link to doc to README
  * added: `Notification.action(identifer, label)` for your convenience
  * added: Hints and Categories API frontend
  * removed `send()`
  * published documentation under https://hoodie.github.io/notify-rust

0.0.5 / 2015-04-04
==================
  * deprecated: `send()` and `send_debug()`, these methods are now called `show()` and `show_debug()`
  * changed: `show()` returns Notification ID
  * changed: set default timeout to -1
  * changed: renamed into notify-rust
  * added: further examples
  * added: documentation
  * moved: tests out of ./src
  * added: `NotificationServer` (for testing only)
  * added: `Notification::actions()`
  * added: `Notification::get_capabilities()`

0.0.4 / 2015-05-30
==================
  * added: `Notification::send_debug()`
  * added: `Notification::appname()`

0.0.3 / 2015-05-24
==================
  * dropped macro, using builder pattern from now on
