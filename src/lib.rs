//! Desktop Notifications for Rust.
//!
//! Desktop notifications are popup messages generated to notify the user of certain events.
//!
//! ## Platform Support
//!
//! Since Version 3.3 this crate builds on macOS, however since the semantic of notifications is
//! quite different between the [XDG](https://en.wikipedia.org/wiki/XDG) specification and macOS, only the a very small subset of
//! functions is supported.
//!
//! # Examples
//! ## Example 1: Simple Notification
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
//! ```no_run
//! # use notify_rust::*;
//! Notification::new()
//!     .summary("Category:email")
//!     .body("This has nothing to do with emails.\nIt should not go away until you acknoledge it.")
//!     .icon("thunderbird")
//!     .appname("thunderbird")
//!     .hint(NotificationHint::Category("email".to_owned()))
//!     .hint(NotificationHint::Resident(true)) // this is not supported by all implementations
//!     .timeout(Timeout::Never) // this however is
//!     .show().unwrap();
//! ```
//!
//! Careful! There are no checks whether you use hints twice.
//! It is possible to set `urgency=Low` AND `urgency=Critical`, in which case the behavior of the server is undefined.
//!
//! ## Example 3: Ask the user to do something
//! ```no_run
//! # use notify_rust::*;
//! # #[cfg(all(unix, not(target_os = "macos")))]
//! Notification::new()
//!     .summary("click me")
//!     .action("default", "default")
//!     .action("clicked", "click here")
//!     .hint(NotificationHint::Resident(true))
//!     .show()
//!     .unwrap()
//!     .wait_for_action({|action|
//!         match action {
//!             "default" => {println!("you clicked \"default\"")},
//!             "clicked" => {println!("that was correct")},
//!             // here "__closed" is a hardcoded keyword
//!             "__closed" => {println!("the notification was closed")},
//!             _ => ()
//!         }
//!     });
//!
//! ```
//!
//! ## Minimal Example
//!
//! You can ommit almost everything
//!
//! ```no_run
//! # use notify_rust::Notification;
//! Notification::new()
//!     .show();
//! ```
//!
//! more [examples](https://github.com/hoodie/notify-rust/tree/master/examples) in the repository.

#![deny(missing_docs,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]

use std::env;
use std::collections::HashSet;
use std::default::Default;

#[cfg(all(unix, not(target_os = "macos")))] use std::borrow::Cow;
#[cfg(all(unix, not(target_os = "macos")))]
extern crate dbus;

#[cfg(target_os = "macos")]
extern crate mac_notification_sys;
#[cfg(target_os = "macos")]
pub use mac_notification_sys::{get_bundle_identifier_or_default, set_application};


#[cfg(all(unix, not(target_os = "macos")))] use dbus::{Connection, BusType, MessageItem};
#[cfg(all(unix, not(target_os = "macos")))] pub use dbus::Error;
#[cfg(all(unix, not(target_os = "macos")))] mod util;
#[cfg(all(unix, not(target_os = "macos")))] pub mod server;

#[cfg(target_os = "macos")] mod macos;
#[cfg(target_os = "macos")] pub use macos::*;
#[cfg(all(unix, not(target_os = "macos")))] mod xdg;
#[cfg(all(unix, not(target_os = "macos")))] use xdg::NotificationHandle;
#[cfg(all(unix, not(target_os = "macos")))] pub use xdg::{ get_capabilities, get_server_information, handle_actions, stop_server };

#[cfg(all(unix, not(target_os = "macos")))] use xdg::build_message;

pub mod hints;
pub use hints::NotificationHint;

/// Desktop notification.
///
/// A desktop notification is configured via builder pattern, before it is launched with `show()`.
#[derive(Debug,Clone)]
pub struct Notification {
    /// Filled by default with executable name.
    pub appname: String,
    /// Single line to summarize the content.
    pub summary: String,
    /// Subtitle for macOS
    pub subtitle: Option<String>,
    /// Multiple lines possible, may support simple markup,
    /// checkout `get_capabilities()` -> `body-markup` and `body-hyperlinks`.
    pub body:    String,
    /// Use a file:// URI or a name in an icon theme, must be compliant freedesktop.org.
    pub icon:    String,
    /// Checkout `NotificationHint`
    pub hints:   HashSet<NotificationHint>,
    /// See `Notification::actions()` and `Notification::action()`
    pub actions: Vec<String>,
    #[cfg(target_os="macos")] sound_name: Option<String>,
    /// Lifetime of the Notification in ms. Often not respected by server, sorry.
    pub timeout: Timeout, // both gnome and galago want allow for -1
    /// Only to be used on the receive end. Use Notification hand for updating.
    id: Option<u32>
}

impl Notification {
    /// Constructs a new Notification.
    ///
    /// Most fields are empty by default, only `appname` is initialized with the name of the current
    /// executable.
    /// The appname is used by some desktop environments to group notifications.
    pub fn new() -> Notification {
        Notification::default()
    }

    /// Overwrite the appname field used for Notification.
    ///
    /// (xdg only)
    pub fn appname(&mut self, appname:&str) -> &mut Notification {
        self.appname = appname.to_owned();
        self
    }

    /// Set the `summary`.
    ///
    /// Often acts as title of the notification. For more elaborate content use the `body` field.
    pub fn summary(&mut self, summary:&str) -> &mut Notification {
        self.summary = summary.to_owned();
        self
    }
    /// Set the `subtitle`.
    ///
    /// This is only useful on macOS, it's not part of the XDG specification and will therefore be eaten by gremlins under your CPU 😈🤘.
    pub fn subtitle(&mut self, subtitle:&str) -> &mut Notification {
        self.subtitle = Some(subtitle.to_owned());
        self
    }


    /// Wrapper for `NotificationHint::SoundName`
    #[cfg(all(unix,not(target_os="macos")))]
    pub fn sound_name(&mut self, name:&str) -> &mut Notification {
        self.hint(NotificationHint::SoundName(name.to_owned()));
        self
    }

    /// Set the sound_name for the NSUserNotification
    #[cfg(taget_os="macos")]
    pub fn sound_name(&mut self, name:&str) -> &mut Notification {
        self.sound_name = Some(name.to_owned());
        self
    }

    /// Set the content of the `body` field.
    ///
    /// Multiline textual content of the notification.
    /// Each line should be treated as a paragraph.
    /// Simple html markup should be supported, depending on the server implementation.
    pub fn body(&mut self, body:&str) -> &mut Notification {
        self.body = body.to_owned();
        self
    }

    /// Set the `icon` field.
    ///
    /// You can use common icon names here, usually those in `/usr/share/icons`
    /// can all be used.
    /// You can also use an absolute path to file.
    ///
    /// # Platform support
    /// macOS does not have support manually setting the icon. However you can pretend to be another app using [`set_application()`](fn.set_application.html)
    pub fn icon(&mut self, icon:&str) -> &mut Notification {
        self.icon = icon.to_owned();
        self
    }

    /// Set the `icon` field automatically.
    ///
    /// This looks at your binaries name and uses it to set the icon.t
    ///
    /// # Platform support
    /// macOS does not have support manually setting the icon. However you can pretend to be another app using [`set_application()`](fn.set_application.html)
    pub fn auto_icon(&mut self) -> &mut Notification {
        self.icon = exe_name();
        self
    }

    /// Adds a hint.
    ///
    /// This method will add a hint to the internal hint hashset.
    /// Hints must be of type NotificationHint.
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// # use notify_rust::NotificationHint;
    /// Notification::new()
    ///     .summary("Category:email")
    ///     .body("This should not go away until you acknoledge it.")
    ///     .icon("thunderbird")
    ///     .appname("thunderbird")
    ///     .hint(NotificationHint::Category("email".to_owned()))
    ///     .hint(NotificationHint::Resident(true))
    ///     .show();
    /// ```
    ///
    /// # Platform support
    /// Most of these hints don't even have an effect on the big XDG Desktops, they are completely tossed on macOS.
    pub fn hint(&mut self, hint:NotificationHint) -> &mut Notification {
        self.hints.insert(hint);
        self
    }

    /// Set the `timeout`.
    ///
    /// This sets the time (in milliseconds) from the time the notification is displayed until it is
    /// closed again by the Notification Server.
    /// According to [specification](https://developer.gnome.org/notification-spec/)
    /// -1 will leave the timeout to be set by the server and
    /// 0 will cause the notification never to expire.
    ///
    /// # Platform support
    /// This only works on XDG Desktops, macOS does not support manually setting the timeout.
    pub fn timeout<T: Into<Timeout>>(&mut self, timeout: T) -> &mut Notification {
        self.timeout = timeout.into();
        self
    }

    /// Set the `urgency`.
    ///
    /// Pick between Medium, Low and High.
    ///
    /// # Platform support
    /// Most Desktops on linux and bsd are far too relaxed to pay any attention to this. macOS it to cool to even have something like this in it's spec 😊.
    pub fn urgency(&mut self, urgency: NotificationUrgency) -> &mut Notification {
        self.hint( NotificationHint::Urgency( urgency ));
        self
    }

    /// Set `actions`.
    ///
    /// To quote http://www.galago-project.org/specs/notification/0.9/x408.html#command-notify
    ///
    /// >  Actions are sent over as a list of pairs.
    /// >  Each even element in the list (starting at index 0) represents the identifier for the action.
    /// >  Each odd element in the list is the localized string that will be displayed to the user.
    ///
    /// There is nothing fancy going on here yet.
    /// **Carefull! This replaces the internal list of actions!**
    ///
    /// (xdg only)
    #[deprecated(note="please use .action() only")]
    pub fn actions(&mut self, actions:Vec<String>) -> &mut Notification {
        self.actions = actions;
        self
    }

    /// Add an action.
    ///
    /// This adds a single action to the internal list of actions.
    ///
    /// (xdg only)
    pub fn action(&mut self, identifier:&str, label:&str) -> &mut Notification {
        self.actions.push(identifier.to_owned());
        self.actions.push(label.to_owned());
        self
    }

    /// Set an Id ahead of time
    ///
    /// Setting the id ahead of time allows overriding a known other notification.
    /// Though if you want to update a notification, it is easier to use the `update()` method of
    /// the `NotificationHandle` object that `show()` returns.
    ///
    /// (xdg only)
    pub fn id(&mut self, id:u32) -> &mut Notification {
        self.id = Some(id);
        self
    }

    /// Finalizes a Notification.
    ///
    /// Part of the builder pattern, returns a complete copy of the built notification.
    pub fn finalize(&self) -> Notification {
        self.clone()
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    fn pack_hints(&self) -> MessageItem {
        if !self.hints.is_empty() {
            let hints:Vec<MessageItem> = self.hints.iter().map(|hint| hint.into() ).collect();

            if let Ok(array) = MessageItem::new_array(hints){
                return array;
            }
        }

        let sig = Cow::Borrowed("{sv}"); // cast to TypeSig makes rust1.0 and rust1.1 panic

        MessageItem::Array(vec![], sig)
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    fn pack_actions(&self) -> MessageItem {
        if !self.actions.is_empty() {
            let mut actions = vec![];
            for action in &self.actions {
                actions.push(action.to_owned().into());
            }
            if let Ok(array) = MessageItem::new_array(actions){
                return array;
            }
        }
        let sig = Cow::Borrowed("s"); // cast to TypeSig makes rust1.0 and rust1.1 panic
        MessageItem::Array(vec![], sig)
    }

    /// Sends Notification to D-Bus.
    ///
    /// Returns a handle to a notification
    #[cfg(all(unix, not(target_os = "macos")))]
    pub fn show(&self) -> Result<NotificationHandle, Error> {
        let connection = try!(Connection::get_private(BusType::Session));
        let inner_id = self.id.unwrap_or(0);
        let id = try!(self._show(inner_id, &connection));
        Ok(NotificationHandle::new(id, connection, self.clone()))
    }

    /// Sends Notification to NSUserNotificationCenter.
    ///
    /// Returns an `Ok` no matter what, since there is currently no way of telling the success of
    /// the notification.
    #[cfg(target_os = "macos")]
    pub fn show(&self) -> Result<NotificationHandle, mac_notification_sys::error::ErrorKind> {
        mac_notification_sys::send_notification(
            &self.summary, //title
            &self.subtitle.as_ref().map(|s| &**s), // subtitle
            &self.body, //message
            &self.sound_name.as_ref().map(|s| &**s) // sound
        ).map(|_| NotificationHandle::new(self.clone()))
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    fn _show(&self, id:u32, connection: &Connection) -> Result<u32, Error> {
        //TODO catch this
        let mut message = build_message("Notify");
        let timeout: i32 = self.timeout.into();
        message.append_items(&[
                             self.appname.to_owned().into(), // appname
                             id.into(),                      // notification to update
                             self.icon.to_owned().into(),    // icon
                             self.summary.to_owned().into(), // summary (title)
                             self.body.to_owned().into(),    // body
                             self.pack_actions().into(),     // actions
                             self.pack_hints().into(),       // hints
                             timeout.into()                  // timeout
        ]);

        let reply = try!(connection.send_with_reply_and_block(message, 2000));

        match  reply.get_items().get(0) {
            Some(&MessageItem::UInt32(ref id)) => Ok(*id),
            _ => Ok(0)
        }
    }

    /// Wraps show() but prints notification to stdout.
    #[cfg(all(unix, not(target_os = "macos")))]
    pub fn show_debug(&mut self) -> Result<NotificationHandle, Error> {
        println!("Notification:\n{appname}: ({icon}) {summary:?} {body:?}\nhints: [{hints:?}]\n",
            appname = self.appname,
            summary = self.summary,
            body    = self.body,
            hints   = self.hints,
            icon    = self.icon,);
        self.show()
    }
}


/// Describes the timeout of a notification
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Timeout {
    /// Expires according to server default.
    ///
    /// Whatever that might be...
    Default,
    /// Do not expire, user will have to close this manually.
    Never,
    /// Expire after n milliseconds.
    Milliseconds(u32)
}

impl From<i32> for Timeout {
    fn from(int: i32) -> Timeout {
        if int < 0 { Timeout::Default }
        else if int == 0 { Timeout::Never }
        else { Timeout::Milliseconds(int as u32) }
    }
}

impl Into<i32> for Timeout {
    fn into(self) -> i32 {
        match self {
            Timeout::Default => -1,
            Timeout::Never => 0,
            Timeout::Milliseconds(ms) => ms as i32
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl<'a> dbus::FromMessageItem<'a> for Timeout {
    fn from(i: &'a MessageItem) -> Result<Timeout,()> {
        if let &MessageItem::Int32(ref b) = i {
            let timeout_millis: i32 = *b;
            Ok(timeout_millis.into())
        } else {
            Err(())
        }
    }
}

impl Default for Notification {
    #[cfg(all(unix, not(target_os="macos")))]
    fn default() -> Notification {
        Notification {
            appname:  exe_name(),
            summary:  String::new(),
            subtitle:  None,
            body:     String::new(),
            icon:     String::new(),
            hints:    HashSet::new(),
            actions:  Vec::new(),
            timeout:  Timeout::Default,
            id:       None
        }
    }
    #[cfg(target_os="macos")]
    fn default() -> Notification {
        Notification {
            appname:  exe_name(),
            summary:  String::new(),
            subtitle:  None,
            body:     String::new(),
            icon:     String::new(),
            hints:    HashSet::new(),
            actions:  Vec::new(),
            timeout:  Timeout::Default,
            sound_name: Default::default(),
            id:       None
        }
    }
}


/// Levels of Urgency.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum NotificationUrgency{
    /// The behaviour for `Low` urgency depends on the notification server.
    Low = 0,
    /// The behaviour for `Normal` urgency depends on the notification server.
    Normal = 1,
    /// A critical notification will not time out.
    Critical = 2
}

impl<'a> From<&'a str> for NotificationUrgency {
    fn from(string:&'a str) -> NotificationUrgency {
        match string.to_lowercase().as_ref() {
            "low"      |
            "lo"       => NotificationUrgency::Low,
            "normal"   |
            "medium"   => NotificationUrgency::Normal,
            "critical" |
            "high"     |
            "hi"       => NotificationUrgency::Critical,
            _ => unimplemented!()
        }
    }
}




/// Return value of `get_server_information()`.
#[derive(Debug)]
pub struct ServerInformation {
    /// The product name of the server.
    pub name:          String,
    /// The vendor name.
    pub vendor:        String,
    /// The server's version string.
    pub version:       String,
    /// The specification version the server is compliant with.
    pub spec_version:  String
}




// Returns the name of the current executable, used as a default for `Notification.appname`.
fn exe_name() -> String {
    env::current_exe().unwrap()
    .file_name().unwrap().to_str().unwrap().to_owned()
}
