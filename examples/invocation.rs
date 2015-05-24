extern crate notify_send;
use notify_send::Notification;
fn main()
{
    // use it this way
    Notification::new()
        .summary("this is the summary")
        .body("this is the body")
        .icon("firefox")
        .timeout(6000) //miliseconds
        .send();


    //possible, but don't do this
    Notification {
        appname: "foobar".to_string(),
        timeout: 2000,
        ..Notification::new()
    }.send();


}

