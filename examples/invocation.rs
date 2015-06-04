extern crate notify_send;
use notify_send::Notification;
fn main()
{
    // use it this way
    Notification::new()
        .summary("Firefox Crashed")
        .body("Just <b>kidding</b>, this is just the notify_show example.")
        .icon("firefox")
        .show();


    //possible, but don't do this
    Notification {
        appname: "foobar".to_string(),
        summary: "foobar".to_string(),
        body: "foobar".to_string(),
        ..Notification::new()
    }.show();


}

