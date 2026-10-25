use crate::Notification;

/// Options that only exist on the `NSUserNotificationCenter` backend.
///
/// These have no effect with the `preview-macos-un` feature,
/// the trait is still available so enabling that feature doesn't break code that uses it.
///
/// # Example
/// ```no_run
/// use notify_rust::{Notification, NsNotificationExt};
///
/// Notification::new()
///     .summary("Pick a pill")
///     .action("red", "Red Pill")
///     .action("blue", "Blue Pill")
///     .actions_label("Pills")
///     .close_button("Neither")
///     .show()
///     .unwrap();
/// ```
pub trait NsNotificationExt {
    /// Set the label of the close button.
    ///
    /// Clicking it is reported as [`CloseReason::Dismissed`](crate::CloseReason::Dismissed).
    fn close_button(&mut self, label: &str) -> &mut Self;

    /// Set the label of the button that opens the actions dropdown.
    ///
    /// Only used when more than one [action](Notification::action) is added. Defaults to `"Options"`.
    fn actions_label(&mut self, label: &str) -> &mut Self;
}

impl NsNotificationExt for Notification {
    fn close_button(&mut self, label: &str) -> &mut Self {
        self.close_button = Some(label.to_owned());
        self
    }

    fn actions_label(&mut self, label: &str) -> &mut Self {
        self.actions_label = Some(label.to_owned());
        self
    }
}
