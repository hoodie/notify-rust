//! **Experimental** server taking the place of your Desktop Environments Notification Server.
//!
//! This is not nearly meant for anything but testing, as it only prints notifications to stdout.
//! It does not respond properly either yet.
//!
//! This server will not replace an already running notification server.
//!

extern crate dbus;

use std::collections::HashSet;

use dbus::{Connection, BusType, NameFlag, ConnectionItem, Message, MessageItem};
use dbus::obj::{ObjectPath, Argument, Method, Interface};

use super::{Notification,NotificationHint};
use util::*;

static DBUS_ERROR_FAILED: &'static str = "org.freedesktop.DBus.Error.Failed";
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub struct NotificationServer
{
    pub counter: u32
}

impl NotificationServer
{
    pub fn new() -> NotificationServer
    {
        NotificationServer{counter:0}
    }

    //fn handle_notification
    pub fn start<F>(&mut self, closure: F)
        where F: Fn(&Notification)
    {
        let connection = Connection::get_private(BusType::Session).unwrap();
        connection.release_name("org.freedesktop.Notifications").unwrap();
        connection.register_name("org.freedesktop.Notifications", NameFlag::ReplaceExisting as u32).ok().expect("Was not able to register name.");
        let mut objpath = ObjectPath::new(&connection, "/org/freedesktop/Notifications", false);
        connection.register_object_path( "/org/freedesktop/Notifications").ok().expect("could not register object path");

        let notify_listener = Interface::new(
            //{{{
            vec![
            Method::new( "Notify",

                         vec![ Argument::new("app_name",    "s"),
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
                         Box::new(move |msg| {

                             let counter = self.counter;

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
                                 ..Notification::new()
                             };

                             closure(&notification); // send id and counter extra

                             self.counter += 1;
                             Ok(vec!(MessageItem::Int32(42)))
                         })
        ),

        Method::new( "GetCapabilities",

                     vec![], //No input arguments
                     vec![Argument::new("caps", "{s}")],
                     Box::new(|_msg|
                              Ok( vec![ MessageItem::new_array(
                                      vec![
                                      "body".to_owned().into(),
                                      ]
                                      ).unwrap()
                              ]
                              )
                             )
                   ),

                   Method::new(
                       "GetServerInformation",
                       // No input arguments
                       vec![],
                       vec![
                       Argument::new("name", "s"),
                       Argument::new("vendor", "s"),
                       Argument::new("version", "s"),
                       Argument::new("spec_version", "s"),
                       ],
                       Box::new(|_msg|
                                Ok( vec![ "notify-rust".to_owned().into(),
                                "notify-rust".to_owned().into(),
                                VERSION.to_owned().into(),
                                "1.1".to_owned().into() ]
                                ))
                       )
                       ],

            vec![],
            vec![] // No properties or signals
            //}}}
            );

        objpath.insert_interface("org.freedesktop.Notifications", notify_listener);
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
        }
    }
}
