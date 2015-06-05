use std::collections::HashSet;

extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;
use notify_rust::NotificationCategory as Category;

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
        .body("this has nothing to do with emails.")
        .icon("thunderbird")
        .appname("thunderbird")
        .hint(Hint::Category(Category::Email))
        .hint(Hint::Transient(false))
        .show();

}

