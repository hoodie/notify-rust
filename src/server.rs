//! **Experimental** server taking the place of your Desktop Environments Notification Server.
//!
//! This is not nearly meant for anything but testing, as it only prints notifications to stdout.
//! It does not respond properly either yet.
//!
//! This server will not replace an already running notification server.
//!
extern crate dbus;

use std::borrow::Cow;

use dbus::{Connection, BusType, NameFlag, ConnectionItem, Message, MessageItem};
use dbus::obj::{ObjectPath, Argument, Method, Interface};

static DBUS_ERROR_FAILED: &'static str = "org.freedesktop.DBus.Error.Failed";

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

    fn unwrap_message_string(&self,item: Option<&MessageItem>) -> String {
        match item{
            Some(&MessageItem::Str(ref value)) => value.clone(),
            Some(&MessageItem::Array(ref items, Cow::Borrowed("{sv}"))) => format!("DICT   {:?}", items),
            Some(&MessageItem::Array(ref items, Cow::Borrowed("s"))) => format!("ARRAY  {:?}", items),
            Some(&MessageItem::Array(ref items, ref sig )) => format!("{sig:?} {items:?}", items=items, sig=sig),
            _ => "".to_owned()
        }
    }


    //fn handle_notification
    pub fn start<F>(&mut self, closure: F)
        where F: Fn(&str, &str, &str, &str, &str, &str, &str, &u32)
    {
        let connection = Connection::get_private(BusType::Session).unwrap();
        connection.release_name("org.freedesktop.Notifications").unwrap();
        connection.register_name("org.freedesktop.Notifications", NameFlag::ReplaceExisting as u32).ok().expect("Was not able to register name.");
        let mut objpath = ObjectPath::new(&connection, "/org/freedesktop/Notifications", false);
        connection.register_object_path( "/org/freedesktop/Notifications").ok().expect("could not register object path");

        let notify_listener = Interface::new(
            //{{{
            vec!( Method::new(
                    "Notify",
                    vec!( Argument::new("app_name",    "s"),
                          Argument::new("replaces_id", "u"),
                          Argument::new("app_icon",    "s"),
                          Argument::new("summary",     "s"),
                          Argument::new("body",        "s"),
                          Argument::new("actions",    "as"),
                          Argument::new("hints",   "a{sv}"),
                          Argument::new("timeout",     "i")
                        ),

                        // No input arguments
                        vec!(Argument::new("arg_0", "u")), //out_args

                        // Callback
                        Box::new(move |msg| {
                            let appname = self.unwrap_message_string(msg.get_items().get(0));
                            let id      = self.unwrap_message_string(msg.get_items().get(1));
                            let icon    = self.unwrap_message_string(msg.get_items().get(2));
                            let summary = self.unwrap_message_string(msg.get_items().get(3));
                            let body    = self.unwrap_message_string(msg.get_items().get(4));
                            let actions = self.unwrap_message_string(msg.get_items().get(5));
                            let hints   = self.unwrap_message_string(msg.get_items().get(6));
                            let counter = self.counter;

                            closure(&appname, &id, &icon, &summary, &body, &actions, &hints, &counter);

                            self.counter += 1;
                            Ok(vec!(MessageItem::Int32(42)))
                        })
                )
            ),
            vec!(),
            vec!() // No properties or signals
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
                ConnectionItem::Nothing => (),

            }
        }
    }
}
