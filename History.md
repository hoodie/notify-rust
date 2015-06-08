
0.0.6 / 2015-06-08
==================

  * added my own gh-pages branch as submodule as branch as submodule
  * added note about Notification::hint()
  * added link to doc to README
  * added: `Notification.action(identifer, label)` for your convenience
  * added: Hints and Categories API frontend
  * removed `send()`
  * published documentation under https://hoodie.github.io/notify-rust

0.0.5 / 2015-04-04
==================
  * deprecated send() and send_debug(), these methods are now called show() and show_debug()
  * Notification IDs are returned
  * changed default timeout to -1
  * renamed into notify-rust
  * added further examples
  * added documentation
  * moved tests out of ./src
  * added NotificationServer (for testing only)
  * added `Notification::actions()`
  * added `Notification::get_capabilities()`

0.0.4 / 2015-05-30
==================
  * added Notification::send_debug()
  * added Notification::appname()

0.0.3 / 2015-05-24
==================
  * dropped macro, using builder pattern from now on

