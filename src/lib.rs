//! Desktop Notifications for Rust.
//!
//! Desktop notifications are popup messages generated to notify the user of certain events.
//!
//! # Examples
//! ```
//! // Example 1
//! use notify_rust::Notification;
//! use notify_rust::NotificationHint as Hint;
//!
//! Notification::new()
//!     .summary("Firefox News")
//!     .body("This will almost look like a real firefox notification.")
//!     .icon("firefox")
//!     .show();
//!
//! // Example 2
//! Notification::new()
//!     .summary("Another notification with actions")
//!     .body("Here each one was added separately.")
//!     .icon("firefox")
//!     .timeout(6000) //miliseconds
//!     .action("action0", "Press me please")
//!     .action("action1", "firefox")
//!     .show();
//!
//! // Example 3
//! Notification::new()
//!     .summary("Category:email")
//!     .body("This has nothing to do with emails.\nIt should not go away untill you acknoledge it.")
//!     .icon("thunderbird")
//!     .appname("thunderbird")
//!     .hint(Hint::Category("email".to_string()))
//!     .hint(Hint::Resident(true))
//!     .show();
//! ```

use std::env;
use std::collections::HashSet;
use std::borrow::Cow;

extern crate dbus;
use dbus::{Connection, BusType, Message, MessageItem, TypeSig};

pub mod server;

/// Executable Name
///
/// Returns the name of the current executable, used as a default for `Notification.appname`.
fn exe_name() -> String
{
    let exe = env::current_exe().unwrap();
    exe.file_name().unwrap().to_str().unwrap().to_string()
}

fn build_message(method_name:&str) -> Message
{
    Message::new_method_call(
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications",
        method_name).unwrap()
}

/// Desktop Notification.
///
/// A desktop notification is configured via builder pattern, before it is launched with `show()`.

pub struct Notification
{
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
    pub timeout: i32
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum NotificationHint
{ // as found on https://developer.gnome.org/notification-spec/
    ActionIcons(bool),
    Category(String),
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
    Custom(String,String)
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

    /// Adds a hint.
    ///
    /// This method will add a hint to the internal hint hashset.
    /// Hints must be of type NotificationHint.
    ///
    /// ```
    /// use notify_rust::Notification;
    /// use notify_rust::NotificationHint;
    /// Notification::new()
    ///     .summary("Category:email")
    ///     .body("This should not go away until you acknoledge it.")
    ///     .icon("thunderbird")
    ///     .appname("thunderbird")
    ///     .hint(NotificationHint::Category("email".to_string()))
    ///     .hint(NotificationHint::Resident(true))
    ///     .show();
    /// ```
    ///
    ///
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
    /// **Carefull! This replaces the internal list of actions!**
    pub fn actions(&mut self, actions:Vec<String>) -> &mut Notification
    {
        self.actions = actions;
        self
    }

    /// Add an action.
    ///
    /// This adds a single action to the internal list of actions.
    pub fn action(&mut self, identifier:&str, label:&str) -> &mut Notification
    {
        self.actions.push(identifier.to_string());
        self.actions.push(label.to_string());
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

    fn pack_hints(&self) -> MessageItem
    {
        if self.hints.len() > 0 {
            let mut hints = vec![];
            for hint in self.hints.iter(){
                let entry:(String,String) = match hint {
                    &NotificationHint::ActionIcons(ref value)  => ("action-icons".to_string(),    format!("{}",  value)), // bool
                    &NotificationHint::Category(ref value)     => ("category".to_string(),        value.clone()),
                    &NotificationHint::DesktopEntry(ref value) => ("desktop-entry".to_string(),   value.clone()),
                  //&NotificationHint::ImageData(iiibiiay)     => ("image-data".to_string(),      format!("{:?}",  value)),
                    &NotificationHint::ImagePath(ref value)    => ("image-path".to_string(),      value.clone()),
                  //&NotificationHint::IconData(iiibiiay)      => ("icon_data".to_string(),       format!("{:?}",  value)),
                    &NotificationHint::Resident(ref value)     => ("resident".to_string(),        format!("{}",  value)), // bool
                    &NotificationHint::SoundFile(ref value)    => ("sound-file".to_string(),      value.clone()),
                    &NotificationHint::SoundName(ref value)    => ("sound-name".to_string(),      value.clone()),
                    &NotificationHint::SuppressSound(value)    => ("suppress-sound".to_string(),  format!("{}",  value)),
                    &NotificationHint::Transient(value)        => ("transient".to_string(),       format!("{}",  value)),
                    &NotificationHint::X(value)                => ("x".to_string(),               format!("{}",  value)),
                    &NotificationHint::Y(value)                => ("y".to_string(),               format!("{}",  value)),
                    &NotificationHint::Urgency(value)          => ("urgency".to_string(),         format!("{}",  value)),
                    _                                          => ("Foo".to_string(),"bar".to_string())
                };

                hints.push( MessageItem::DictEntry(
                        Box::new(MessageItem::Str(entry.0)),
                        Box::new(MessageItem::Variant( Box::new(MessageItem::Str(entry.1))))
                        ));
            }
            return MessageItem::new_array(hints);
        }

        // let sig = Cow::Borrowed("{sv}") as TypeSig;
        // return MessageItem::Array(vec![], sig);
        return MessageItem::new_array(vec![
            MessageItem::DictEntry(
                Box::new(MessageItem::Str("".to_string())),
                Box::new(MessageItem::Variant( Box::new(MessageItem::Str("".to_string()))))
                )
            ]);
    }

    fn pack_actions(&self) -> MessageItem
    {
        if self.actions.len() > 0 {
            let mut actions = vec![];
            for action in self.actions.iter()
            {
                actions.push(MessageItem::Str(action.to_string()))
            }
            return MessageItem::new_array(actions);
        }
        //let sig = Cow::Borrowed("s") as TypeSig;
        //return MessageItem::Array(vec![], sig);
        return MessageItem::new_array(vec![ MessageItem::Str("".to_string()) ]);
    }

    /// Sends Notification to D-Bus.
    ///
    /// Returns id from D-Bus.
    pub fn show(&self) -> u32
    {
        //println!("{} hints:    {:?}", self.hints.len() ,self.pack_hints());
        //println!("{} actions:  {:?}", self.actions.len()/2 ,self.pack_actions());
        //TODO catch this
        let mut message = build_message("Notify");

        //TODO implement hints and actions
        message.append_items(&[
           MessageItem::Str(self.appname.to_string()),      // appname
           MessageItem::UInt32(0),                          // notification to update
           MessageItem::Str(self.icon.to_string()),         // icon
           MessageItem::Str(self.summary.to_string()),      // summary (title)
           MessageItem::Str(self.body.to_string()),         // body
                                  self.pack_actions() ,     // actions
                                  self.pack_hints(),        // hints
           MessageItem::Int32(self.timeout)                 // timeout
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
}

/// Get list of all capabilities of the running Notification Server.
pub fn get_capabilities() -> Vec<String>
{
    let mut capabilities = vec![];

    let message = build_message("GetCapabilities");
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

/// Close a Notification given by id.
pub fn close_notification(id:u32)
{
    let mut message = build_message("CloseNotification");
    message.append_items(&[ MessageItem::UInt32(id) ]);
    let connection = Connection::get_private(BusType::Session).unwrap();
    connection.send(message);
}
 
//pub fn get_server_information() -> (name:String, vendor:String, version:String, versionspec:String)
