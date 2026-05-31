//! Desktop Notifications for Rust.
//!
//! Desktop notifications are popup messages generated to notify the user of certain events.
//!
//! ## Platform Support
//!
//! This library was originally conceived with the [XDG](https://en.wikipedia.org/wiki/XDG) notification specification in mind.
//! Since version 3.3 this crate also builds on macOS, however the semantics of the [XDG](https://en.wikipedia.org/wiki/XDG) specification and macOS `NSNotifications`
//! are quite different.
//! Therefore only a very small subset of functions is supported on macOS.
//! Certain methods don't have any effect there, others will explicitly fail to compile,
//! in these cases you will have to add platform specific toggles to your code.
//! For more see [platform differences](#platform-differences)
//!
//! # Examples
//!
//! ## Example 1: Simple Notification
//!
//! ```no_run
//! # use notify_rust::*;
//! Notification::new()
//!     .summary("Firefox News")
//!     .body("This will almost look like a real firefox notification.")
//!     .icon("firefox")
//!     .timeout(Timeout::Milliseconds(6000)) //milliseconds
//!     .show().unwrap();
//! ```
//!
//! ## Example 2: Persistent Notification
//!
//! ```no_run
//! # use notify_rust::*;
//! Notification::new()
//!     .summary("Category:email")
//!     .body("This has nothing to do with emails.\nIt should not go away until you acknowledge it.")
//!     .icon("thunderbird")
//!     .appname("thunderbird")
//!     .hint(Hint::Category("email".to_owned()))
//!     .hint(Hint::Resident(true)) // this is not supported by all implementations
//!     .timeout(Timeout::Never) // this however is
//!     .show().unwrap();
//! ```
//!
//! Careful! There are no checks whether you use hints twice.
//! It is possible to set `urgency=Low` AND `urgency=Critical`, in which case the behavior of the server is undefined.
//!
//! ## Example 3: Ask the user to do something
//!
//! ```no_run
//! # use notify_rust::*;
//! # #[cfg(all(unix, not(target_os = "macos")))]
//! Notification::new().summary("click me")
//!                    .action("default", "default")
//!                    .action("clicked", "click here")
//!                    .hint(Hint::Resident(true))
//!                    .show()
//!                    .unwrap()
//!                    .wait_for_action(|action| match action {
//!                                         "default" => println!("you clicked \"default\""),
//!                                         "clicked" => println!("that was correct"),
//!                                         // here "__closed" is a hard coded keyword
//!                                         "__closed" => println!("the notification was closed"),
//!                                         _ => ()
//!                                     });
//! ```
//!
//! ## Minimal Example
//!
//! You can omit almost everything
//!
//! ```no_run
//! # use notify_rust::Notification;
//! Notification::new().show();
//! ```
//!
//! more [examples](https://github.com/hoodie/notify-rust/tree/main/examples) in the repository.
//!
//! # Platform Differences
//! <details>
//! ✔︎ = works <br/>
//! ❌ = will not compile
//!
//! ## `Notification`
//! | method              | XDG   | macOS | windows |
//! |---------------------|-------|-------|---------|
//! |  `fn appname(...)`  |  ✔︎    |       |        |
//! |  `fn summary(...)`  |  ✔︎    | ✔︎     |  ✔︎    |
//! |  `fn subtitle(...)` |       | ✔︎     |  ✔︎    |
//! |  `fn body(...)`     |  ✔︎    | ✔︎     |  ✔︎    |
//! |  `fn icon(...)`     |  ✔︎    |       |        |
//! |  `fn image_path(...)`|  ✔︎   | ✔︎     |  ✔︎    |
//! |  `fn auto_icon(...)`|  ✔︎    |       |        |
//! |  `fn hint(...)`     |  ✔︎    | ❌    | ❌    |
//! |  `fn timeout(...)`  |  ✔︎    |       |  ✔︎    |
//! |  `fn urgency(...)`  |  ✔︎    | ❌    |  ✔︎    |
//! |  `fn action(...)`   |  ✔︎    |       |        |
//! |  `fn id(...)`       |  ✔︎    |       |        |
//! |  `fn finalize(...)` |  ✔︎    | ✔︎     |  ✔︎    |
//! |  `fn show(...)`     |  ✔︎    | ✔︎     |  ✔︎    |
//!
//! ## `NotificationHandle`
//!
//! | method                   | XDG | macOS | windows |
//! |--------------------------|-----|-------|---------|
//! | `fn wait_for_action(...)`|  ✔︎  |  ❌  |   ❌   |
//! | `fn close(...)`          |  ✔︎  |  ❌  |   ❌   |
//! | `fn on_close(...)`       |  ✔︎  |  ❌  |   ❌   |
//! | `fn update(...)`         |  ✔︎  |  ❌  |   ❌   |
//! | `fn id(...)`             |  ✔︎  |  ❌  |   ❌   |
//!
//! ## Functions
//!
//! |                                            | XDG | macOS | windows |
//! |--------------------------------------------|-----|-------|---------|
//! | `fn get_capabilities(...)`                 | ✔︎   |   ❌ |  ❌    |
//! | `fn get_server_information(...)`           | ✔︎   |   ❌ |  ❌    |
//! | `fn set_application(...)`                  | ❌  |   ✔︎  |  ❌    |
//! | `fn get_bundle_identifier_or_default(...)` | ❌  |   ✔︎  |  ❌    |
//!
//!
//! ### Toggles
//!
//! Please use `target_os` toggles if you plan on using methods labeled with ❌.
//!
//! ```ignore
//! #[cfg(target_os = "macos")]
//! // or
//! // #### #[cfg(all(unix, not(target_os = "macos")))]
//! ```
//! </details>
//!
//! ## Preview Backends
//!
//! ### `preview-macos-un` — `UNUserNotificationCenter` (macOS)
//!
//! Opt in to the modern `UNUserNotificationCenter` backend by enabling the feature:
//!
//! ```toml
//! notify-rust = { version = "4", features = ["preview-macos-un"] }
//! ```
//!
//! The binary must have a valid `CFBundleIdentifier` and be code-signed
//! (an ad-hoc signature is sufficient).
//!
//! With this feature, `show()` returns a [`NotificationHandle`] with richer capabilities:
//!
//! ```no_run
//! # #[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use notify_rust::{Notification, NotificationResponse};
//!
//! let handle = Notification::new()
//!     .summary("Pick one")
//!     .action("yes", "Yes")
//!     .action("no", "No")
//!     .timeout(notify_rust::Timeout::Milliseconds(30_000))
//!     .show()?;
//!
//! match handle.response_blocking() {
//!     NotificationResponse::Action(ref key) if key == "yes" => println!("confirmed"),
//!     NotificationResponse::Action(ref key) if key == "no"  => println!("declined"),
//!     NotificationResponse::Closed(reason) => println!("dismissed: {reason:?}"),
//!     other => println!("{other:?}"),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! An async version is also available via [`Notification::show_async`] and [`NotificationHandle::response`].
//!

#![deny(
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]
#![warn(
    missing_docs,
    clippy::doc_markdown,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match_else,
    clippy::inconsistent_struct_constructor,
    clippy::map_unwrap_or,
    clippy::match_same_arms
)]

#[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
extern crate dbus;

#[cfg(all(target_os = "macos", not(feature = "preview-macos-un")))]
extern crate mac_notification_sys;

#[cfg(target_os = "windows")]
extern crate winrt_notification;

#[macro_use]
#[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
extern crate lazy_static;

pub mod error;
mod hints;
mod miniver;
mod notification;
mod notification_id;
mod response;
mod timeout;
pub(crate) mod urgency;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
pub use mac_usernotifications::InterruptionLevel;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(all(unix, not(target_os = "macos")))]
mod xdg;

#[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
mod image;

/// macOS legacy path: get the bundle identifier or a default.
#[cfg(all(target_os = "macos", not(feature = "preview-macos-un")))]
pub use macos::{get_bundle_identifier_or_default, set_application, NotificationHandle};

/// macOS `UNUserNotificationCenter` preview path (requires `preview-macos-un` feature).
#[cfg(all(target_os = "macos", feature = "preview-macos-un"))]
pub use macos::{
    check_bundle, get_notification_settings, get_notification_settings_blocking, request_auth,
    request_auth_blocking, NotificationHandle,
};

#[cfg(all(
    any(feature = "dbus", feature = "zbus"),
    unix,
    not(target_os = "macos")
))]
pub use crate::xdg::{
    dbus_stack, get_capabilities, get_server_information, handle_action, DbusStack,
    NotificationHandle,
};

// Cross-platform response types (available on all platforms).
#[allow(deprecated)]
pub use crate::response::ActionResponse;
pub use crate::{
    notification_id::NotificationId,
    response::{CloseHandler, CloseReason, NotificationResponse, ResponseHandler},
};

pub use crate::hints::Hint;

#[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
pub use crate::image::{Image, ImageError};

#[cfg_attr(
    target_os = "macos",
    deprecated(note = "Urgency is not supported on macOS")
)]
pub use crate::urgency::Urgency;

pub use crate::{notification::Notification, timeout::Timeout};

#[cfg(all(feature = "images_no_default_features", unix, not(target_os = "macos")))]
lazy_static! {
    /// Read once at runtime. Needed for Images
    pub static ref SPEC_VERSION: miniver::Version =
        get_server_information()
        .and_then(|info| info.spec_version.parse::<miniver::Version>())
        .unwrap_or_else(|_| miniver::Version::new(1,1));
}
/// Return value of `get_server_information()`.
#[derive(Debug)]
pub struct ServerInformation {
    /// The product name of the server.
    pub name: String,
    /// The vendor name.
    pub vendor: String,
    /// The server's version string.
    pub version: String,
    /// The specification version the server is compliant with.
    pub spec_version: String,
}
