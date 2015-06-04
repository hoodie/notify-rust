extern crate notify_send;
use notify_send::Notification;

//#[test]
//fn get_capabilities()
//{
//    Notification::get_capabilities();
//}

#[test]
fn build_pattern()
{
    let notification = Notification::new().summary("foo").finalize();
    assert_eq!(notification.summary, "foo");

    let mut notification = Notification::new();
    notification.body = "foo".to_string();
    assert_eq!(notification.body, "foo");

    let mut notification = Notification::new();
    notification.icon = "foo".to_string();
    assert_eq!(notification.icon, "foo");

    let mut notification = Notification::new();
    notification.summary = "foo".to_string();
    assert_eq!(notification.summary, "foo");

    let mut notification = Notification::new();
    notification.timeout = 42;
    assert_eq!(notification.timeout, 21*2);

    let mut notification = Notification::new();
    notification.summary = "foo".to_string();
    assert_eq!(notification.summary, "foo");
}

//#[test]
//fn it_works()
//{
//
//    Notification {
//        //appname: "foobar".to_string(),
//        summary: "invocation type 1".to_string(),
//        body: Notification::new().appname,
//        timeout: 20,
//        ..Notification::new()
//    }.send();
//
//    let mut message = Notification::new();
//    message.summary("invocation type 2");
//    message.body("your <b>body</b> is a <u>wonderland</u>");
//    message.send();
//
//    Notification::new()
//        .summary("this is the summary")
//        .summary("invocation type 3")
//        .body("this is the body\nnewline<br/>linebreak")
//        .send();
//
//}
//
//#[test]
//fn loop_test()
//{
//    for i in 0..5 {
//    Notification::new()
//        .summary(&format!("loop {}",i))
//        .body("this is the body\nnewline<br/>linebreak").send();
//    }
//}

//#[test]
//fn properly_tested() {
//    //assert!(false);
//}
