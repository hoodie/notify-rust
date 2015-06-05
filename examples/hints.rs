use std::collections::HashSet;

extern crate notify_rust;
use notify_rust::Notification;
use notify_rust::NotificationHint as Hint;

fn main()
{
    // use it this way
    Notification::new()
        .summary("This notification uses hints")
        .icon("firefox")
        .hint(Hint::Urgency(0))
        .hint(Hint::X(0))
        .show();

}

