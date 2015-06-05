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
//!     .show();
//! ```

use std::env;
extern crate dbus;
use dbus::{Connection, BusType, Message, MessageItem};
use std::collections::HashSet;

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
/// A desktop notification is configured via builder pattern, before it is launched with `show()`.

pub struct Notification
{
    pub appname: String,
    pub summary: String,
    pub body:    String,
    pub icon:    String,
    pub hints:   HashSet<NotificationHint>,
    pub actions: Vec<String>,
    pub timeout: i32
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum NotificationCategory
{ // as found on https://developer.gnome.org/notification-spec/
    Device,                //A generic device-related notification that doesn't fit into any other category.
    DeviceAdded,           //A device, such as a USB device, was added to the system.
    DeviceError,           //A device had some kind of error.
    DeviceRemoved,         //A device, such as a USB device, was removed from the system.
    Email,                 //A generic e-mail-related notification that doesn't fit into any other category.
    EmailArrived,          //A new e-mail notification.
    EmailBounced,          //A notification stating that an e-mail has bounced.
    Im,                    //A generic instant message-related notification that doesn't fit into any other category.
    ImError,               //An instant message error notification.
    ImReceived,            //A received instant message notification.
    Network,               //A generic network notification that doesn't fit into any other category.
    NetworkConnected,      //A network connection notification, such as successful sign-on to a network service. This should not be confused with device.added for new network devices.
    NetworkDisconnected,   //A network disconnected notification. This should not be confused with device.removed for disconnected network devices.
    NetworkError,          //A network-related or connection-related error.
    Presence,              //A generic presence change notification that doesn't fit into any other category, such as going away or idle.
    PresenceOffline,       //An offline presence change notification.
    PresenceOnline,        //An online presence change notification.
    Transfer,              //A generic file transfer or download notification that doesn't fit into any other category.
    TransferComplete,      //A file transfer or download complete notification.
    TransferError,         //A file transfer or download error.
    Custom(String)
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum NotificationHint
{ // as found on https://developer.gnome.org/notification-spec/
    ActionIcon(String),
    ActionIcons(bool),
    Category(NotificationCategory),
    DesktopEntry(String),
    //ImageData(iiibiiay),
    ImagePath(String),
    //IconData(iiibiiay),
    Resident(bool),
    SoundFile(String),
    SoundName(String),
    SuppressSound(bool),
    Transient(bool),
    X(i32),
    Y(i32),
    Urgency(i32), // 0, 1, 2
    Custom(String)
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
            hints:   HashSet::new(),
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

    pub fn hint(&mut self, hint:NotificationHint) -> &mut Notification
    {
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
            hints:    self.hints.clone(),
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

    /// Sends Notification to D-Bus.
    ///
    /// Returns id from D-Bus.
    pub fn show(&self) -> u32
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

    /// Wraps show() but prints notification to stdout.
    pub fn show_debug(&self) -> u32
    {
        println!("Notification:\n{}: ({}) {} \"{}\"\n", self.appname, self.icon, self.summary, self.body);
        self.show()
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


