//! Desktop Notifications for Rust.
//!
//! Desktop notifications are popup messages generated to notify the user of certain events.
//!
//! # Examples
//! ## Example 1 (Simple Notification)
//! ```no_run
//! # use notify_rust::Notification;
//! # use notify_rust::NotificationHint as Hint;
//! Notification::new()
//!     .summary("Firefox News")
//!     .body("This will almost look like a real firefox notification.")
//!     .icon("firefox")
//!     .timeout(6000) //milliseconds
//!     .show().unwrap();
//! ```
//!
//! ## Example 2 (Persistent Notification)
//! ```no_run
//! # use notify_rust::Notification;
//! # use notify_rust::NotificationHint as Hint;
//! Notification::new()
//!     .summary("Category:email")
//!     .body("This has nothing to do with emails.\nIt should not go away until you acknoledge it.")
//!     .icon("thunderbird")
//!     .appname("thunderbird")
//!     .hint(Hint::Category("email".to_owned()))
//!     .hint(Hint::Resident(true)) // this is not supported by all implementations
//!     .timeout(0) // this however is
//!     .show().unwrap();
//! ```
//!
//! Careful! There are no checks whether you use hints twice.
//! It is possible to set `urgency=Low` AND `urgency=Critical`, in which case the behavior of the server is undefined.
//!
//! ## Example 3 (Ask the user to do something)
//! ```no_run
//! # use notify_rust::Notification;
//! # use notify_rust::NotificationHint as Hint;
//! Notification::new()
//!     .summary("click me")
//!     .action("default", "default")
//!     .action("clicked", "click here")
//!     .hint(Hint::Resident(true))
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
//! more [examples](https://github.com/hoodie/notify-rust/tree/master/examples) in the repository.

#![deny(missing_docs,
        missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        //unstable_features,
        unused_import_braces, unused_qualifications)]
#![warn(missing_debug_implementations)]

#![cfg_attr(feature = "lints", allow(unstable_features))]
#![cfg_attr(feature = "lints", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]

use std::env;
use std::collections::HashSet;
use std::borrow::Cow;
use std::ops::{Deref,DerefMut};
use std::default::Default;

#[cfg(all(unix, not(target_os = "macos")))]
extern crate dbus;
#[cfg(all(unix, not(target_os = "macos")))]
use dbus::{Connection, ConnectionItem, BusType, Message, MessageItem};
pub use dbus::Error;

mod util;
pub mod server;
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
    /// Multiple lines possible, may support simple markup,
    /// checkout `get_capabilities()` -> `body-markup` and `body-hyperlinks`.
    pub body:    String,
    /// Use a file:// URI or a name in an icon theme, must be compliant freedesktop.org.
    pub icon:    String,
    /// Checkout `NotificationHint`
    pub hints:   HashSet<NotificationHint>,
    /// See `Notification::actions()` and `Notification::action()`
    pub actions: Vec<String>,
    /// Lifetime of the Notification in ms. Often not respected by server, sorry.
    /// -1 -> expires according server default
    /// 0 -> expires never
    pub timeout: i32, // both gnome and galago want allow for -1
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
    pub fn icon(&mut self, icon:&str) -> &mut Notification {
        self.icon = icon.to_owned();
        self
    }

    /// Set the `icon` field automatically.
    ///
    /// This looks at your binaries name and uses it to set the icon.t
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
    ///
    pub fn hint(&mut self, hint:NotificationHint) -> &mut Notification {
        self.hints.insert(hint);
        self
    }

    /// Set the `timeout`.
    ///
    /// This sets the time (in miliseconds) from the time the notification is displayed until it is
    /// closed again by the Notification Server.
    /// According to [specification](https://developer.gnome.org/notification-spec/)
    /// -1 will leave the timeout to be set by the server and
    /// 0 will cause the notification never to expire.
    pub fn timeout(&mut self, timeout: i32) -> &mut Notification {
        self.timeout = timeout;
        self
    }

    /// Set the `urgency`.
    ///
    /// Pick between Medium, Low and High.
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
    pub fn actions(&mut self, actions:Vec<String>) -> &mut Notification {
        self.actions = actions;
        self
    }

    /// Add an action.
    ///
    /// This adds a single action to the internal list of actions.
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
    pub fn id(&mut self, id:u32) -> &mut Notification {
        self.id = Some(id);
        self
    }

    /// Finalizes a Notification.
    ///
    /// Part of the builder pattern, returns a complete copy of the built notification.
    pub fn finalize(&self) -> Notification {
        Notification {
            appname:  self.appname.clone(),
            summary:  self.summary.clone(),
            body:     self.body.clone(),
            icon:     self.icon.clone(),
            hints:    self.hints.clone(),
            actions:  self.actions.clone(),
            timeout:  self.timeout,
            id:       self.id
        }
    }

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
    pub fn show(&self) -> Result<NotificationHandle, Error> {
        let connection = try!(Connection::get_private(BusType::Session));
        let inner_id = self.id.unwrap_or(0);
        let id = try!(self._show(inner_id, &connection));
        Ok(NotificationHandle::new(id, connection, self.clone()))
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    fn _show(&self, id:u32, connection: &Connection) -> Result<u32, Error> {
        //TODO catch this
        let mut message = build_message("Notify");

        message.append_items(&[
                             self.appname.to_owned().into(), // appname
                             id.into(),                      // notification to update
                             self.icon.to_owned().into(),    // icon
                             self.summary.to_owned().into(), // summary (title)
                             self.body.to_owned().into(),    // body
                             self.pack_actions().into(),     // actions
                             self.pack_hints().into(),       // hints
                             self.timeout.into()             // timeout
        ]);

        let reply = try!(connection.send_with_reply_and_block(message, 2000));

        match  reply.get_items().get(0) {
            Some(&MessageItem::UInt32(ref id)) => Ok(*id),
            _ => Ok(0)
        }
    }

    #[cfg(target_os="macos")]
    fn _show(&self, id:u32, connection: &Connection) -> Result<u32, Error> {
    }

    /// Wraps show() but prints notification to stdout.
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

impl Default for Notification {
    fn default() -> Notification {
        Notification {
            appname:  exe_name(),
            summary:  String::new(),
            body:     String::new(),
            icon:     String::new(),
            hints:    HashSet::new(),
            actions:  Vec::new(),
            timeout:  -1,
            id:       None
        }
    }
}




/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
#[derive(Debug)]
pub struct NotificationHandle {
    id: u32,
    connection: Connection,
    notification: Notification
}

impl NotificationHandle {
    fn new(id: u32, connection: Connection, notification: Notification) -> NotificationHandle {
        NotificationHandle {
            id: id,
            connection: connection,
            notification: notification
        }
    }

    /// Waits for the user to act on a notification and then calls
    /// `invokation_closure` with the name of the corresponding action.
    pub fn wait_for_action<F>(self, invokation_closure:F) where F:FnOnce(&str) {
        wait_for_action_signal(&self.connection, self.id, invokation_closure);
    }

    /// Manually close the notification
    pub fn close(self) {
        let mut message = build_message("CloseNotification");
        message.append_items(&[ self.id.into() ]);
        let _ = self.connection.send(message); // If closing fails there's nothing we could do anyway
    }


    /// Executes a closure after the notification has closed.
    pub fn on_close<F>(self, closure:F) where F: FnOnce(){
        self.wait_for_action(|action|
            if action == "__closed" { closure(); }
        );
    }

    /// Replace the original notification with an updated version
    /// ## Example
    /// ```no_run
    /// # use notify_rust::Notification;
    /// let mut notification = Notification::new()
    ///     .summary("Latest News")
    ///     .body("Bayern Dortmund 3:2")
    ///     .show().unwrap();
    ///
    /// std::thread::sleep_ms(1_500);
    ///
    /// notification
    ///     .summary("Latest News (Correction)")
    ///     .body("Bayern Dortmund 3:3");
    ///
    /// notification.update();
    /// ```
    /// Watch out for different implementations of the
    /// notification server! On plasma5 or instance, you should also change the appname, so the old
    /// message is really replaced and not just amended. Xfce behaves well, all others have not
    /// been tested by the developer.
    pub fn update(&mut self) {
        self.id = self.notification._show(self.id, &self.connection).unwrap();
    }

    /// Returns the Handle's id.
    pub fn id(&self) -> u32{
        self.id
    }
}

/// Required for `DerefMut`
impl Deref for NotificationHandle {
    type Target = Notification;
    fn deref(&self) -> &Notification {
        &self.notification
    }
}

/// Allow to easily modify notification properties
impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.notification
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




// here be public functions


/// Get list of all capabilities of the running notification server.
pub fn get_capabilities() -> Result<Vec<String>, Error> {
    let mut capabilities = vec![];

    let message    = build_message("GetCapabilities");
    let connection = try!(Connection::get_private(BusType::Session));
    let reply      = try!(connection.send_with_reply_and_block(message, 2000));

    if let Some(&MessageItem::Array(ref items, Cow::Borrowed("s"))) = reply.get_items().get(0) {
        for item in items.iter(){
            if let MessageItem::Str(ref cap) = *item{
                capabilities.push(cap.clone());
            }
        }
    }

    Ok(capabilities)
}

/// Returns a struct containing `ServerInformation`.
///
/// This struct contains `name`, `vendor`, `version` and `spec_version` of the notification server
/// running.
pub fn get_server_information() -> Result<ServerInformation, Error> {
    let message    = build_message("GetServerInformation");
    let connection = try!(Connection::get_private(BusType::Session));
    let reply      = try!(connection.send_with_reply_and_block(message, 2000));

    let items = reply.get_items();

    Ok( ServerInformation{
        name:          unwrap_message_string(items.get(0)),
        vendor:        unwrap_message_string(items.get(1)),
        version:       unwrap_message_string(items.get(2)),
        spec_version:  unwrap_message_string(items.get(3))
    })
}

/// Strictly internal.
/// The Notificationserver implemented here exposes a "Stop" function.
/// stops the notification server
pub fn stop_server() {
    let message    = build_message("Stop");
    let connection = Connection::get_private(BusType::Session).unwrap();
    let _reply     = connection.send_with_reply_and_block(message, 2000).unwrap();
}



/// Listens for the `ActionInvoked(UInt32, String)` Signal.
///
/// No need to use this, check out `Notification::show_and_wait_for_action(FnOnce(action:&str))`
pub fn handle_actions<F>(id:u32, func:F) where F: FnOnce(&str) {
    let connection = Connection::get_private(BusType::Session).unwrap();
    wait_for_action_signal(&connection, id, func);
}



// here be non public functions


// Listens for the `ActionInvoked(UInt32, String)` signal.
fn wait_for_action_signal<F>(connection: &Connection, id: u32, func: F) where F: FnOnce(&str) {
    connection.add_match("interface='org.freedesktop.Notifications',member='ActionInvoked'").unwrap();
    connection.add_match("interface='org.freedesktop.Notifications',member='ActionInvoked'").unwrap();
    connection.add_match("interface='org.freedesktop.Notifications',member='NotificationClosed'").unwrap();

    for item in connection.iter(1000) {
        if let ConnectionItem::Signal(s) = item {
            let (_, protocol, iface, member) = s.headers();
            let items = s.get_items();
            match (&*protocol.unwrap(), &*iface.unwrap(), &*member.unwrap()) {

                // Action Invoked
                ("/org/freedesktop/Notifications", "org.freedesktop.Notifications", "ActionInvoked") => {
                    if let (&MessageItem::UInt32(nid), &MessageItem::Str(ref action)) = (&items[0], &items[1]) {
                        if nid == id { func(action); break; }
                    }
                },


                // Notification Closed
                ("/org/freedesktop/Notifications", "org.freedesktop.Notifications", "NotificationClosed") => {
                    if let (&MessageItem::UInt32(nid), &MessageItem::UInt32(_)) = (&items[0], &items[1]) {
                        if nid == id  { func("__closed"); break; }
                    }
                },
                (_, _, _) => ()
            }
        }
    }
}

// Returns the name of the current executable, used as a default for `Notification.appname`.
fn exe_name() -> String {
    env::current_exe().unwrap()
    .file_name().unwrap().to_str().unwrap().to_owned()
}

fn build_message(method_name:&str) -> Message {
    Message::new_method_call(
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications",
        method_name).expect(&format!("Error building message call {:?}.", method_name))
}

fn unwrap_message_string(item: Option<&MessageItem>) -> String {
    match item{
        Some(&MessageItem::Str(ref value)) => value.to_owned(),
        _ => "".to_owned()
    }
}
