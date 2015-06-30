extern crate notify_rust;
use notify_rust::Notification;
fn main()
{
    let naive = false; // change this to switch between moth implementation styles

    if naive {

        // naive way to update a notification
        let mut notification = Notification::new()
            .summary("Firefox Crashed")
            .body("Just <b>kidding</b>, this is just the notify_show example.")
            .icon("firefox").finalize();
        let id = notification.show();
        std::thread::sleep_ms(1_500);
        notify_rust::close_notification(id);
        notification
            //.appname("foo") // changing appname to keep plasma from merging both notifications
            .body("wait, something has changed")
            .show();

    }

    else {

        // the new and shiny api for updating
        let mut notification = Notification::new()
            .summary("Firefox Crashed")
            .body("Just <b>kidding</b>, this is just the notify_show example.")
            .icon("firefox").finalize();
        notification.show();
        std::thread::sleep_ms(1_500);
        notification
            .appname("foo") // changing appname to keep plasma from merging the new and the old one
            .body("wait, something has changed")
            .update();

    }


}

