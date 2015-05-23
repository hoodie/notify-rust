extern crate dbus;
use dbus::{Connection, BusType, Message, MessageItem};

use std::env;
use std::borrow::Cow;

#[test]
fn it_works() {
    NotifyMessage {
        appname: "foobar".into(),
        timeout: 20,
        ..NotifyMessage::default()
    }.send("Build");

    let mes = NotifyMessage::new();
    mes.send("Built from new()");
    let message = NotifyMessage::new()
        //.summary("Title")
        //.body("Description")
        //.icon("news-feed")
        .send("empty");
}

#[test]
fn properly_tested() {
    //assert!(false);
}

pub fn exe_name() -> String{
    let exe = env::current_exe().unwrap();
    exe.file_name().unwrap().to_str().unwrap().to_string()
}

struct NotifyMessage<'a> // is 'a necessary ? http://rustbyexample.com/scope/lifetime.html
{
    appname: Cow<'a, str>,
    summary: Cow<'a, str>,
    body:    Cow<'a, str>,
    icon:    Cow<'a, str>,
    timeout: i32
}

impl<'a> Default for NotifyMessage<'a> {
    fn default() -> NotifyMessage<'a> {
        NotifyMessage {
            appname:  exe_name().into(),
            summary:  "".into(),
            body:     "".into(),
            icon:     "".into(),
            timeout:  5
        }
    }
}

impl<'a> NotifyMessage<'a>
{
    fn new() -> NotifyMessage<'a> {
        NotifyMessage {
            appname:  exe_name().into(),
            summary:  "".into(),
            body:     "".into(),
            icon:     "".into(),
            timeout:  5
        }
    }

    pub fn send(self, summary: &str)
    {
        let mut m = Message::new_method_call(
            "org.freedesktop.NotificationCommandImps",
            "/org/freedesktop/Notifications",
            "org.freedesktop.Notifications",
            "Notify"
            ).unwrap();

        m.append_items(&[
                       MessageItem::Str(self.appname.to_string()),         // appname
                       MessageItem::UInt32(0),                        // notification to update
                       MessageItem::Str(self.icon.to_string()),            // icon
                       MessageItem::Str(summary.to_string()),         // summary (title)
                       MessageItem::Str(summary.to_string()),            // body
                       MessageItem::new_array(                        // actions
                           vec!( MessageItem::Str("".to_string()))),
                           MessageItem::new_array(                        // hints
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
        c.send_with_reply_and_block(m, self.timeout);
        //let mut r = c.send_with_reply_and_block(m, 2000).unwrap();
        //let reply = r.get_items();
        //println!("{:?}", reply);


    }
}


