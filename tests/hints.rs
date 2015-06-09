extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;

#[test]
fn urgency()
{
    // use it this way
    for urgency in 0..3{
        Notification::new()
            .summary(&format!("Urgency {}", urgency))
            .body("This notification uses hints")
            .icon("firefox")
            .hint(Hint::Urgency(urgency))
            .show();
    }
    }

#[test]
fn category()
{
    Notification::new()
        .summary("Category:email")
        .body("This has nothing to do with emails.")
        .icon("thunderbird")
        .appname("thunderbird")
        .hint(Hint::Category("email".to_string()))
        .show();
}

#[test]
fn persistent()
{
    Notification::new()
        .summary("Incomming Call: Your Mom!")
        .body("This should not go away untill you acknoledge it.")
        .icon("call-start")
        .hint(Hint::Resident(true))
        .show();

}

