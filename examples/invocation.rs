#![allow(unused_must_use)]
extern crate notify_rust;
use notify_rust::Notification;
fn main()
{
    Notification::new()
        .summary("Firefox Crashed")
        .body("Just <b>kidding</b>, this is just the notify_show example.")
        .icon("firefox")
        .show();

}
