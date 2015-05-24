extern crate notify_send;
use notify_send::*;
fn main()
{
    // use it this way
    Notification::new()
        .summary("this is the summary")
        .body("this is the body")
        .icon("firefox")
        .timeout(16)
        .send();


    //possible, but don't do this
    NotifyMessage {
        appname: "foobar".to_string(),
        timeout: 20,
        ..NotifyMessage::new()
    }.send();


}

