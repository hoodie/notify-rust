extern crate notify_send;
use notify_send::*;
fn main()
{
    Notification::new()
        .summary("this is the summary")
        .body("this is the body")
        .send();


    NotifyMessage {
        appname: "foobar".to_string(),
        timeout: 20,
        ..NotifyMessage::new()
    }.send();


}

