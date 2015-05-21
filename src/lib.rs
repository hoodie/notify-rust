extern crate dbus;
use dbus::{Connection, BusType, Message, MessageItem};
use std::env;

#[macro_export]
macro_rules! notify_send {
    () => ( send (&exe_name(), "summary", "body", "dialog-ok", 5););
    ($title:expr) =>
        ( send (&exe_name(), $title, "", "", 5););

    ($title:expr, t $timeout:expr) =>
        ( send (&exe_name(), $title, "", "", $timeout););

    ($title:expr, $message:expr) =>
        ( send (&exe_name(), $title, $message, "", 5););

    ($title:expr, $message:expr, t $timeout:expr) =>
        ( send (&exe_name(), $title, $message, "", $timeout););

    ($title:expr, $message:expr, $icon:expr) =>
        ( send (&exe_name(), $title, $message, $icon, 5););

    ($title:expr, $message:expr, $icon:expr, t $timeout:expr) =>
        ( send (&exe_name(), $title, $message, $icon, $timeout););

}


#[test]
fn it_works() {
    //send( "cargo" , "notify test", "If you can read this, this lib seems to work." , "dialog-ok");
    notify_send!("title1-t", t 5000);
    notify_send!("title1");
    notify_send!("title2", "with message");
    notify_send!("title3", "with message and icon", "dialog-ok");
    notify_send!("title4", "with message, icon and timeout", "dialog-ok", t 3000);
    //TODO: assert response from dbus for failure, this test currently is not a good test
}

#[test]
fn properly_tested() {
    assert!(false);
}

pub fn exe_name() -> String{
    let exe = env::current_exe().unwrap();
    exe.file_name().unwrap().to_str().unwrap().to_string()
}


#[allow(unused_must_use)]
pub fn send( appname: &str, summary: &str, body:&str, icon: &str, timeout: i32 )
{
    let mut m = Message::new_method_call(
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications",
        "Notify"
        ).unwrap();

    m.append_items(&[
                       MessageItem::Str(appname.to_string()),         // appname
                       MessageItem::UInt32(0),                        // notification to update
                       MessageItem::Str(icon.to_string()),            // icon
                       MessageItem::Str(summary.to_string()),         // summary (title)
                       MessageItem::Str(body.to_string()),            // body
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
                       MessageItem::Int32(timeout),                       // timeout
                   ]);
    let c = Connection::get_private(BusType::Session).unwrap();
    c.send_with_reply_and_block(m, timeout);
    //let mut r = c.send_with_reply_and_block(m, 2000).unwrap();
    //let reply = r.get_items();
    //println!("{:?}", reply);

 }

