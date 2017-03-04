//! **Experimental** server taking the place of your Desktop Environments Notification Server.
//!
//! This is not nearly meant for anything but testing, as it only prints notifications to stdout.
//! It does not respond properly either yet.
//!
//! This server will not replace an already running notification server.
//!

use std::collections::HashSet;
use std::cell::Cell;

use dbus::{Connection, BusType, NameFlag, ConnectionItem, Message, MessageItem};
use dbus::obj::{ObjectPath, Argument, Method, Interface};

use super::{Notification,NotificationHint};
use util::*;

static DBUS_ERROR_FAILED: &'static str = "org.freedesktop.DBus.Error.Failed";
/// Version of the crate equals the version server.
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// An **experimental** notification server.
/// See [the module level documentation](index.html) for more.
#[derive(Default)]
pub struct NotificationServer {
    /// Counter for generating notification ids
    pub counter: Cell<u32>,
    /// A flag that stops the server
    pub stop: Cell<bool>
}

impl NotificationServer {
    fn count_up(&self) {
        self.counter.set( self.counter.get() + 1);
    }

    /// Create a new `NotificationServer` instance.
    pub fn new() -> NotificationServer {
        NotificationServer::default()
    }

    //pub fn notify_mothod<F>(&mut self, closure: F)
    //    -> Method
    //    where F: Fn(&Notification)
    //{

    //fn handle_notification

    /// Start listening for incoming notifications
    pub fn start<F>(&mut self, closure: F) where F: Fn(&Notification) {
        let connection = Connection::get_private(BusType::Session).unwrap();
        connection.release_name("org.freedesktop.Notifications").unwrap();
        connection.register_name("org.freedesktop.Notifications", NameFlag::ReplaceExisting as u32).expect("Was not able to register name.");
        let mut objpath = ObjectPath::new(&connection, "/org/freedesktop/Notifications", false);
        connection.register_object_path( "/org/freedesktop/Notifications").expect("could not register object path");

        let server_interface = Interface::new(
            vec![
                Method::new("Notify",
                            vec![   Argument::new("app_name",    "s"),
                                    Argument::new("replaces_id", "u"),
                                    Argument::new("app_icon",    "s"),
                                    Argument::new("summary",     "s"),
                                    Argument::new("body",        "s"),
                                    Argument::new("actions",    "as"),
                                    Argument::new("hints",   "a{sv}"),
                                    Argument::new("timeout",     "i")
                            ],

                            vec![Argument::new("arg_0", "u")], //out_args

                            // Callback
                            Box::new(|msg| {

                                // TODO this must be prettier!
                                let hint_items = msg.get_items().get(6).unwrap().clone();
                                let hint_items:&Vec<MessageItem> = hint_items.inner().unwrap();
                                let hints = hint_items.iter().map(|item|item.into()).collect::<HashSet<NotificationHint>>();

                                let action_items = msg.get_items().get(5).unwrap().clone();
                                let action_items:&Vec<MessageItem> = action_items.inner().unwrap();
                                let actions:Vec<String> = action_items.iter().map(|action|action.inner::<&String>().unwrap().to_owned()).collect();

                                let notification = Notification{
                                    appname: unwrap_message_str(msg.get_items().get(0).unwrap()),
                                    summary: unwrap_message_string(msg.get_items().get(3)),
                                    body:    unwrap_message_string(msg.get_items().get(4)),
                                    icon:    unwrap_message_string(msg.get_items().get(2)),
                                    timeout: msg.get_items().get(7).unwrap().inner().unwrap(),
                                    hints:   hints,
                                    actions: actions,
                                    id: Some(self.counter.get())
                                };

                                closure(&notification); // send id and counter extra

                                self.count_up();
                                Ok(vec!(MessageItem::Int32(42)))
                             })
                ),

                Method::new("CloseNotification",
                            vec![Argument::new("id", "u")], //No input arguments
                            vec![],
                            //MessageItem::new_array( vec![ "body".into(), ]).unwrap()
                            Box::new(|msg| {
                                println!("{:?}", msg);
                                Ok( vec![])}
                                )
                           ),

                Method::new("Stop",
                            vec![], //No input arguments
                            vec![],
                            //MessageItem::new_array( vec![ "body".into(), ]).unwrap()
                            Box::new(|_msg| {
                                self.stop.set(true);
                                Ok( vec![])}
                                )
                           ),

                Method::new("GetCapabilities",
                             vec![], //No input arguments
                             vec![Argument::new("caps", "{s}")],
                             Box::new(|_msg| Ok( vec![
                                     MessageItem::new_array( vec![ "body".into(), ]).unwrap()
                             ]))
                ),

                Method::new("GetServerInformation",
                            vec![], // No input arguments
                            vec![
                                Argument::new("name", "s"),
                                Argument::new("vendor", "s"),
                                Argument::new("version", "s"),
                                Argument::new("spec_version", "s"),
                            ],
                            Box::new(|_msg| Ok( vec![
                                        "notify-rust daemon".into(), "notify-rust".into(), VERSION.into(), "1.1".into()
                            ]))
                )
            ],

            vec![], // no properties
            vec![]  // no signals
        );

        objpath.insert_interface("org.freedesktop.Notifications", server_interface);
        //objpath.set_registered(true).unwrap();

        for n in connection.iter(10) {
            match n {
                ConnectionItem::MethodCall(mut m) =>
                    if objpath.handle_message(&mut m).is_none() {
                        connection.send(Message::new_error(&m, DBUS_ERROR_FAILED, "Object path not found").unwrap()).unwrap();
                    }
                ,
                ConnectionItem::Signal(_m) => { /*println!("Signal: {:?}", _m);*/ },
                _ => (),
            }
            if self.stop.get() {
                println!("stopping server");
                break;
            }
        }
    }
}
