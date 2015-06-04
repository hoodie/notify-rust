//! Desktop Notifications for Rust.
//!
//! Desktop notifications are popup messages generated to notify the user of certain events.
//!
//! # Example
//! ```
//! Notification::new()
//!     .summary("Firefox News")
//!     .body("This will almost look like a real firefox notification.")
//!     .icon("firefox")
//!     .send();
//! ```

use std::env;
extern crate dbus;
use dbus::{Connection, BusType, Message, MessageItem};

pub mod server;

/// Executable Name
///
/// Returns the name of the current executable, used as a default for `Notification.appname`.
pub fn exe_name() -> String
{
    let exe = env::current_exe().unwrap();
    exe.file_name().unwrap().to_str().unwrap().to_string()
}

/// Desktop Notification.
///
/// A desktop notification is configured via builder pattern, before it is launched with `send()`.

pub struct Notification
{
    pub appname: String,
    pub summary: String,
    pub body:    String,
    pub icon:    String,
    pub actions: Vec<String>,
    pub timeout: i32
}


impl Notification
{
    /// Constructs a new Notification.
    ///
    /// Most fields are empty by default, only `appname` is prefilled with the name of the current
    /// executable.
    /// The appname is used by some desktop environments to group notifications.
    pub fn new() -> Notification
    {
        Notification {
            appname:  exe_name(),
            summary:  String::new(),
            body:     String::new(),
            icon:     String::new(),
            actions:  Vec::new(),
            timeout:  -1
        }
    }

    /// Overwrite the appname field used for Notification.
    pub fn appname(&mut self, appname:&str) -> &mut Notification
    {
        self.appname = appname.to_string();
        self
    }

    /// Set the `summary`.
    ///
    /// Often acts as title of the notification. For more elaborate content use the `body` field.
    pub fn summary(&mut self, summary:&str) -> &mut Notification
    {
        self.summary = summary.to_string();
        self
    }

    /// Set the content of the `body` field.
    ///
    /// Multiline textual content of the notification.
    /// Each line should be treated as a paragraph.
    /// Simple html markup should be supported, depending on the server implementation.
    pub fn body(&mut self, body:&str) -> &mut Notification
    {
        self.body = body.to_string();
        self
    }

    /// Set the `icon` field.
    ///
    /// You can use commom icon names here, usually those in `/usr/share/icons`
    /// can all be used.
    /// You can also use an absolute path to file.
    pub fn icon(&mut self, icon:&str) -> &mut Notification
    {
        self.icon = icon.to_string();
        self
    }

    /// Set the `timeout`.
    ///
    /// This sets the time (in miliseconds) from the time the notification is displayed until it is
    /// closed again by the Notification Server.
    /// According to [specification](https://developer.gnome.org/notification-spec/)
    /// -1 will leave the timeout to be set by the server and
    /// 0 will cause the notification never to expire.
    pub fn timeout(&mut self, timeout: i32) -> &mut Notification
    {
        self.timeout = timeout;
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
    pub fn actions(&mut self, actions:Vec<String>) -> &mut Notification
    {
        self.actions = actions;
        self
    }

    /// Finalizes a Notification.
    ///
    /// Part of the builder pattern, returns a complete copy of the built notification.
    pub fn finalize(&self) -> Notification
    {
        Notification {
            appname:  self.appname.clone(),
            summary:  self.summary.clone(),
            body:     self.body.clone(),
            icon:     self.icon.clone(),
            actions:  self.actions.clone(),
            timeout:  self.timeout.clone(),
        }
    }

    fn pack_actions(&self) -> Vec<MessageItem>
    {
        if self.actions.len() > 0 {
        let mut actions = vec![];
        for action in self.actions.iter()
        {
            actions.push(MessageItem::Str(action.to_string()))
        }
        return actions;
        }
        return vec!( MessageItem::Str("".to_string()))
    }

    /// Sends Notification to DBus.
    ///
    /// Returns id from DBus. 
    pub fn send(&self) -> u32
    {
        //TODO catch this
        let mut message = Message::new_method_call(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
            "Notify").unwrap();

        //TODO implement hints and actions
        message.append_items(&[
           MessageItem::Str(self.appname.to_string()),      // appname
           MessageItem::UInt32(0),                          // notification to update
           MessageItem::Str(self.icon.to_string()),         // icon
           MessageItem::Str(self.summary.to_string()),      // summary (title)
           MessageItem::Str(self.body.to_string()),         // body
           MessageItem::new_array(self.pack_actions()),     // actions
           MessageItem::new_array(                          // hints
               vec!(
                   MessageItem::DictEntry(
                       Box::new(MessageItem::Str("".to_string())),
                       Box::new(MessageItem::Variant( Box::new(MessageItem::Str("".to_string()))))
                       ),
                   )
           ),
           MessageItem::Int32(self.timeout)                // timeout
           ]);
        let connection = Connection::get_private(BusType::Session).unwrap();
        let mut r = connection.send_with_reply_and_block(message, 2000).unwrap();
        if let Some(&MessageItem::UInt32(ref id)) = r.get_items().get(0) { return *id }
        else {return 0}
    }

    /// Wraps send() but prints notification to stdout.
    pub fn send_debug(&self) -> u32
    {
        println!("Notification:\n{}: ({}) {} \"{}\"\n", self.appname, self.icon, self.summary, self.body);
        self.send()
    }

    /// Get list of all capabilities of the running Notification Server.
    pub fn get_capabilities() -> Vec<String>
    {
        use std::borrow::Cow;
        let mut capabilities = vec![];

        let message = Message::new_method_call(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
            "GetCapabilities").unwrap();
        let connection = Connection::get_private(BusType::Session).unwrap();
        let mut r = connection.send_with_reply_and_block(message, 2000).unwrap();

        if let Some(&MessageItem::Array(ref items, Cow::Borrowed("s"))) = r.get_items().get(0) {
            for item in items.iter(){
                if let &MessageItem::Str(ref cap) = item{
                    capabilities.push(cap.clone());
                }
            }
        }
        return capabilities;
    }


}


