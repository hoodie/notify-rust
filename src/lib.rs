#![allow(dead_code)]
#![allow(unused_must_use)]
use std::env;
extern crate dbus;
use dbus::{Connection, BusType, Message, MessageItem};

#[test]
fn it_works() {
    NotifyMessage {
        appname: "foobar".to_string(),
        summary: "invocation type 1".to_string(),
        timeout: 20,
        ..NotifyMessage::new()
    }.send();

    let mut message = Notification::new();
    message.summary("invocation type 2");
    message.body("your body is a wonderland");
    message.send();

    Notification::new()
        .summary("this is the summary")
        .body("this is the body")
        .summary("invocation type 3")
        .send();
}

#[test]
fn properly_tested() {
    //assert!(false);
}

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

pub struct NotifyMessage {
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
            timeout:  5
        }
    }
    pub fn body(&mut self, body:&str) -> &mut Notification {
        self.body = body.to_string();
        self
    }
    pub fn summary(&mut self, summary:&str) -> &mut Notification {
        self.summary = summary.to_string();
        self
    }
    pub fn send(&self) {
        NotifyMessage{
            appname: self.appname.clone(),
            summary: self.summary.clone(),
            body:    self.body.clone(),
            icon:    self.icon.clone(),
            timeout: self.timeout
        }.send();
    }
}


impl NotifyMessage
{
    pub fn new() -> NotifyMessage{
        NotifyMessage {
            appname:  exe_name(),
            summary:  String::new(),
            body:     String::new(),
            icon:     String::new(),
            timeout:  5
        }
    }
    pub fn send(self)
    {
        let mut m = Message::new_method_call(
            "org.freedesktop.NotificationCommandImps",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
            "Notify"
            ).unwrap();

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
                               MessageItem::Int32(self.timeout),                       // timeout
        ]);
        let c = Connection::get_private(BusType::Session).unwrap();
        println!("{}: ({}) {} \"{}\"",
                 self.appname,
                 self.icon,
                 self.summary,
                 self.body);
        c.send_with_reply_and_block(m, self.timeout);

        //let mut r = c.send_with_reply_and_block(m, 2000).unwrap();
        //let reply = r.get_items();
        //println!("{:?}", reply);


    }
}


