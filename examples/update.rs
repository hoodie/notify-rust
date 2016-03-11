extern crate notify_rust;
use notify_rust::Notification;
use std::time::Duration;

fn update_via_handle()
{
    let mut notification_handle = Notification::new()
        .summary("WARNING")
        .body("something terrible just happened!")
        .icon("dialog-warning")
        .show()
        .unwrap();

    std::thread::sleep(Duration::from_millis(1_500));

    notification_handle
        .appname("foo") // changing appname to keep plasma from merging the new and the old one
        .icon("dialog-ok")
        .body("Just <b>kidding</b>, this is just the notification update example.");

    notification_handle.update();

}

fn update_via_manually_stored_id()
{

    let handle = Notification::new()
        .summary("WARNING")
        .body("something terrible just happened!")
        .icon("dialog-warning")
        .show()
        .unwrap();

    let id = handle.id();
    std::thread::sleep(Duration::from_millis(1_500));

    Notification::new()
        .appname("foo") // changing appname to keep plasma from merging the new and the old one
        .icon("dialog-ok")
        .body("Just <b>kidding</b>, this is just the notification update example.")
        .id(id)
        .show()
        .unwrap();
    


}

fn main()
{
    update_via_handle();

    //update_via_manually_stored_id();
}
