extern crate notify_rust;
use notify_rust::Notification;

#[test]
fn closing()
{
    Notification::new()
        .summary("You see me")
        .body("you don't see me!")
        .show()
        .close();
}

#[test]
fn burst()
{
    for msg in [
        "These should each",
        "come in their own pop up.",
        "If they don't than",
        "I will have to complain about it."
    ].iter(){
        Notification::new()
            .summary("burst")
            .appname(&format!("{}", msg))
            .body(&format!("{}", msg))
            .icon("media-floppy")
            .show();
    }

    for msg in [
        "These may be grouped",
        "together by the server.",
        "that is because the all have the same",
        "appname."
    ].iter(){
        Notification::new()
            .summary("merged burst")
            .body(&format!("{}", msg))
            .icon("applications-toys")
            .show();
    }
}

#[test]
fn get_capabilities()
{
    let capabilities:Vec<String> = notify_rust::get_capabilities();
    for capability in capabilities{
        Notification::new().summary("capability").body(&format!("{}", capability)).show();
    }
}

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

#[test]
fn init()
{

    Notification {
        //appname: "foobar".to_string(),
        summary: "invocation type 1".to_string(),
        body: Notification::new().appname,
        timeout: 20,
        ..Notification::new()
    }.show();

    let mut message = Notification::new();
    message.summary("invocation type 2");
    message.body("your <b>body</b> is a <u>wonderland</u>");
    message.show();

    Notification::new()
        .summary("this is the summary")
        .summary("invocation type 3")
        .body("this is the body\nnewline<br/>linebreak")
        .show();

}
