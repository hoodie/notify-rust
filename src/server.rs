extern crate dbus;

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

    //fn handle_notification
    pub fn start(&mut self)
    {
        let connection = Connection::get_private(BusType::Session).unwrap();
        connection.register_name("org.freedesktop.Notifications", NameFlag::ReplaceExisting as u32).unwrap();
        let mut objpath = ObjectPath::new(&connection, "/org/freedesktop/Notifications", true);

        // why not this way?
        //let interface1 = Fn Notify(app_name:String, replaces_id:u32, app_icon:String, body:String, actions:Vec<String>, hints:HashMap<String,String>, timeout:i32) -> i32;

        let notify_listener = Interface::new(
            vec!( Method::new("Notify",
                    vec!( //in_args
                        Argument::new("app_name",    "s"), // appname
                        Argument::new("replaces_id", "u"), // notification to update
                        Argument::new("app_icon",    "s"), // icon
                        Argument::new("summary",     "s"), // summary (title)
                        Argument::new("body",        "s"), // body
                        Argument::new("actions",    "as"), // actions
                        Argument::new("hints",   "a{sv}"), // hints
                        Argument::new("timeout",     "i"), // timeout
                        ),

                    // No input arguments
                    vec!(Argument::new("arg_0", "u")), //out_args

                    // Callback
                    // TODO how do I elegantly access self.counter?
                    Box::new(move |msg| {
                                    println!("appname: {:?} icon:{:?} summary:{:?} body:{:?}",
                                             msg.get_items().get(0).unwrap(),
                                             //msg.get_items().get(1).unwrap(),
                                             msg.get_items().get(2).unwrap(),
                                             msg.get_items().get(3).unwrap(),
                                             msg.get_items().get(4).unwrap(),
                                             );
                                    Ok(vec!(MessageItem::Int32(42)))
                                   }
                            )
                    )
                ),
                vec!(), vec!() // No properties or signals
                );
        objpath.insert_interface( "org.freedesktop.Notifications", notify_listener);

        objpath.set_registered(true).unwrap();

        for n in connection.iter(10) {
            match n {
                ConnectionItem::MethodCall(mut m) => {
                    //println!("\nMethodCall:");

                    if objpath.handle_message(&mut m).is_none() {
                        connection.send(Message::new_error(&m, DBUS_ERROR_FAILED, "Object path not found").unwrap()).unwrap();
                    }else{
                        self.counter += 1;
                    };
                },
                ConnectionItem::Signal(m) => { println!("Signal: {:?}", m); },
                ConnectionItem::Nothing => (),

            }
        }
        panic!("The server is out of cycles, sorry");
    }
}
