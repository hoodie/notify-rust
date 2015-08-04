2.0.0 / 2015-08-4
==================
* breaking change: changed API of actions

1.1.0 / 2015-08-3
==================
* changed: introduced NotificationHandles instead of integer IDs

1.0.1 / 2015-07-19
==================
* fixed little refactoring hickup

1.0.0 / 2015-07-13
==================
* added: show_debug()
* stabilization


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
  * deprecated: `send()` and `send_debug()`, these methods are now called
	`show()` and `show_debug()`
  * changed: `show()` returns Notification ID
  * changed: set default timeout to -1
  * changed: renamed into notify-rust
  * added: further examples
  * added: documentation
  * moved: tests out of ./src
  * added: NotificationServer (for testing only)
  * added: `Notification::actions()`
  * added: `Notification::get_capabilities()`

0.0.4 / 2015-05-30
==================
  * added: `Notification::send_debug()`
  * added: `Notification::appname()`

0.0.3 / 2015-05-24
==================
  * dropped macro, using builder pattern from now on

