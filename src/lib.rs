extern crate dbus;
use dbus::{Connection, BusType, Message, MessageItem};

#[test]
fn it_works() {
    send( "cargo" , "notify test", "If you can read this, this lib seems to work." , "dialog-ok");
    //TODO: assert response from dbus for failure, this test currently is not a good test
}

#[test]
fn properly_tested() {
    assert!(false);
}

pub fn send( appname: &str, summary: &str, body:&str, icon: &str )
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
                       MessageItem::Int32(9000),                       // timeout
                   ]);
    let c = Connection::get_private(BusType::Session).unwrap();
    let mut r = c.send_with_reply_and_block(m, 2000).unwrap();
    let reply = r.get_items();
    println!("{:?}", reply);

 }

