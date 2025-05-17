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
//! |  `fn auto_icon(...)`|  ✔︎    |       |        |
//! |  `fn hint(...)`     |  ✔︎    | ❌    | ❌    |
//! |  `fn timeout(...)`  |  ✔︎    |       |  ✔︎    |
//! |  `fn urgency(...)`  |  ✔︎    | ❌    | ❌    |
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

#[cfg(target_os = "macos")]
extern crate mac_notification_sys;

#[cfg(target_os = "windows")]
extern crate winrt_notification;

pub mod error;
mod hints;
mod miniver;
mod notification;
mod timeout;
pub(crate) mod urgency;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(all(unix, not(target_os = "macos")))]
mod xdg;

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
mod image;

#[cfg(target_os = "macos")]
pub use mac_notification_sys::{get_bundle_identifier_or_default, set_application};

#[cfg(target_os = "macos")]
pub use macos::NotificationHandle;

#[cfg(all(
    any(feature = "dbus", feature = "zbus"),
    unix,
    not(target_os = "macos")
))]
pub use crate::xdg::{
    dbus_stack, get_capabilities, get_server_information, handle_action, ActionResponse,
    CloseHandler, CloseReason, DbusStack, NotificationHandle,
};

// #[cfg(all(feature = "server", unix, not(target_os = "macos")))]
// pub use crate::xdg::stop_server;

pub use crate::hints::Hint;

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
pub use crate::image::{Image, ImageError};

#[cfg_attr(
    target_os = "macos",
    deprecated(note = "Urgency is not supported on macOS")
)]
pub use crate::urgency::Urgency;

pub use crate::{notification::Notification, timeout::Timeout};

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
/// Read once at runtime. Needed for Images
pub static SPEC_VERSION: std::sync::LazyLock<miniver::Version> = std::sync::LazyLock::new(|| {
    get_server_information()
        .and_then(|info| info.spec_version.parse::<miniver::Version>())
        .unwrap_or_else(|_| miniver::Version::new(1, 1))
});

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
