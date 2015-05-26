#![allow(dead_code)]
#![allow(unused_must_use)]
use std::env;
extern crate dbus;
use dbus::{Connection, BusType, Message, MessageItem};

#[test]
fn it_works() {

    Notification {
        //appname: "foobar".to_string(),
        summary: "invocation type 1".to_string(),
        body: Notification::new().appname,
        timeout: 20,
        ..Notification::new()
    }.send();

    let mut message = Notification::new();
    message.summary("invocation type 2");
    message.body("your <b>body</b> is a <u>wonderland</u>");
    message.send();

    Notification::new()
        .summary("this is the summary")
        .summary("invocation type 3")
        .body("this is the body\nnewline<br/>linebreak")
        .send();
}

//#[test]
//fn properly_tested() {
//    //assert!(false);
//}

pub fn exe_name() -> String {
    let exe = env::current_exe().unwrap();
    exe.file_name().unwrap().to_str().unwrap().to_string()
}

pub struct Notification {
    pub appname: String,
    pub summary: String,
    pub body:    String,
    pub icon:    String,
    pub timeout: i32
}


impl Notification {
    pub fn new() -> Notification {
        Notification {
            appname:  exe_name(),
            summary:  String::new(),
            body:     String::new(),
            icon:     String::new(),
            timeout:  2000
        }
    }

    pub fn appname(&mut self, appname:&str) -> &mut Notification {
        self.appname = appname.to_string();
        self
    }

    pub fn body(&mut self, body:&str) -> &mut Notification {
        self.body = body.to_string();
        self
    }

    pub fn icon(&mut self, icon:&str) -> &mut Notification {
        self.icon = icon.to_string();
        self
    }

    pub fn timeout(&mut self, timeout: i32) -> &mut Notification {
        self.timeout = timeout;
        self
    }

    pub fn summary(&mut self, summary:&str) -> &mut Notification {
        self.summary = summary.to_string();
        self
    }

    pub fn send_debug(&self)
    {
        println!("Notification:\n{}: ({}) {} \"{}\"\n", self.appname, self.icon, self.summary, self.body);
        self.send();
    }

    pub fn send(&self)
    {
        //TODO catch this
        let mut m = Message::new_method_call(
            "org.freedesktop.Notifications",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
            "Notify").unwrap();

        //TODO implement hints and actions
        m.append_items(&[
                       MessageItem::Str(self.appname.to_string()),      // appname
                       MessageItem::UInt32(0),                          // notification to update
                       MessageItem::Str(self.icon.to_string()),         // icon
                       MessageItem::Str(self.summary.to_string()),      // summary (title)
                       MessageItem::Str(self.body.to_string()),         // body
                       MessageItem::new_array(                          // actions
                           vec!( MessageItem::Str("".to_string()))),
                           MessageItem::new_array(                      // hints
                               vec!(
                                   MessageItem::DictEntry(
                                       Box::new(MessageItem::Str("".to_string())),
                                       Box::new(MessageItem::Variant(
                                               Box::new(MessageItem::Str("".to_string()))
                                               ))
                                       ),
                                   )
                           ),
                       MessageItem::Int32(self.timeout),                // timeout
        ]);
        let c = Connection::get_private(BusType::Session).unwrap();
        //println!("{}: ({}) {} \"{}\"", self.appname, self.icon, self.summary, self.body);
        c.send_with_reply_and_block(m, 2000);

        ////TODO make use of reply
        //let mut r = c.send_with_reply_and_block(m, 2000).unwrap();
        //let reply = r.get_items();
        //println!("{:?}", reply);
    }
}


