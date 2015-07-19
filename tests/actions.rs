extern crate notify_rust;
use notify_rust::Notification;

#[test]
fn actions_vec() {
    // Actions are sent over as a list of pairs.
    // Each even element in the list (starting at index 0) represents the identifier for the action.
    // Each odd element in the list is the localized string that will be displayed to the user.
    // http://www.galago-project.org/specs/notification/0.9/x408.html#command-notify

    Notification::new()
        .summary("Notification with actions")
        .body("action1=\"Action One\", something_else=\"Something Else\"")
        .icon("dialog-information")
        .timeout(6000) //miliseconds
        .actions(
            vec![
            "action1".to_owned(), "Action One".to_string(),
            "something_else".into(), String::from("Something Else") // so many possibilities for $str -> String :D
            ])
        .show();
}

#[test]
fn action_manual() {
    Notification::new()
        .summary("Another notification with actions")
        .body("action0=\"Press me please\", action1=\"firefox\"")
        .icon("dialog-information")
        .timeout(6000) //miliseconds
        .action("action0", "Press me please")
        .action("action1", "firefox")
        .show();

}

