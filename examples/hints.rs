extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;

fn main()
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

    Notification::new()
        .summary("Category:email")
        .body("This has nothing to do with emails.\nIt should not go away untill you acknoledge it.")
        .icon("thunderbird")
        .appname("thunderbird")
        .hint(Hint::Category("email".to_string()))
        .hint(Hint::Resident(true))
        .show();

}

