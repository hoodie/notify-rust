extern crate notify_rust;
use notify_rust::Notification;
fn main()
{
    // Actions are sent over as a list of pairs.
    // Each even element in the list (starting at index 0) represents the identifier for the action.
    // Each odd element in the list is the localized string that will be displayed to the user.
    // http://www.galago-project.org/specs/notification/0.9/x408.html#command-notify

    Notification::new()
        .summary("Firefox Crashed")
        .body("Just <b>kidding</b>, this is just the notify_rust example.")
        .icon("firefox")
        .timeout(6000) //miliseconds
        .actions(
            vec![
            "action1".to_string(), //
            "Action One".to_string(),
            "something_else".to_string(),
            "Something Else".to_string()
            ])
        .show();


}

