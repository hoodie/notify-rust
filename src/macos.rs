use crate::{error::*, notification::Notification};

pub use mac_notification_sys::error::{ApplicationError, Error as MacOsError, NotificationError};

use std::{
    fmt,
    ops::{Deref, DerefMut},
    sync::mpsc::{self, Receiver},
    thread,
};

/// Reason why a notification closed.
#[derive(Copy, Clone, Debug)]
pub enum CloseReason {
    /// The notification expired.
    Expired,
    /// The user dismissed the notification.
    Dismissed,
    /// The notification was closed because the user clicked an action.
    CloseAction,
    /// The platform did not expose a more specific reason.
    Other(u32),
}

/// Response to a macOS notification.
#[derive(Clone, Debug)]
pub enum ActionResponse {
    /// Custom action configured by the notification.
    Custom(String),

    /// The notification was closed.
    Closed(CloseReason),
}

/// Helper trait implemented by `Fn()` and `Fn(CloseReason)`.
pub trait CloseHandler<T> {
    /// Calls the handler with the close reason.
    fn call(&self, reason: CloseReason);
}

impl<F> CloseHandler<CloseReason> for F
where
    F: Fn(CloseReason),
{
    fn call(&self, reason: CloseReason) {
        self(reason);
    }
}

impl<F> CloseHandler<()> for F
where
    F: Fn(),
{
    fn call(&self, _: CloseReason) {
        self();
    }
}

/// A handle to a shown notification.
///
/// The handle owns the receiving side of a one-shot channel. The sending side is
/// held by a background thread that waits for macOS to report the user's
/// response, so creating a notification does not block the caller.
pub struct NotificationHandle {
    notification: Notification,
    response: Option<Receiver<ActionResponse>>,
}

impl fmt::Debug for NotificationHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotificationHandle")
            .field("notification", &self.notification)
            .finish_non_exhaustive()
    }
}

impl NotificationHandle {
    #[allow(missing_docs)]
    pub fn new(
        notification: Notification,
        response: Receiver<ActionResponse>,
    ) -> NotificationHandle {
        NotificationHandle {
            notification,
            response: Some(response),
        }
    }

    /// Waits for the user to act on a notification and then calls
    /// `invocation_closure` with the corresponding action name.
    ///
    /// Clicking the notification body maps to `"default"`. Closing the
    /// notification maps to `"__closed"` for compatibility with the XDG API.
    /// macOS reports action button titles, so notify-rust maps a title back to
    /// the identifier passed to [`Notification::action`] when possible.
    pub fn wait_for_action<F>(mut self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        if let Some(response) = self.response.take().and_then(|rx| rx.recv().ok()) {
            match response {
                ActionResponse::Custom(action) => invocation_closure(&action),
                ActionResponse::Closed(_reason) => invocation_closure("__closed"),
            }
        }
    }

    /// Runs a callback on a background thread after macOS reports a response.
    ///
    /// This is the non-blocking counterpart to [`wait_for_action`](Self::wait_for_action).
    pub fn on_response<F>(mut self, invocation_closure: F)
    where
        F: FnOnce(ActionResponse) + Send + 'static,
    {
        if let Some(response) = self.response.take() {
            thread::spawn(move || {
                if let Ok(response) = response.recv() {
                    invocation_closure(response);
                }
            });
        }
    }

    /// Executes a closure after the notification has closed.
    pub fn on_close<A>(mut self, handler: impl CloseHandler<A>) {
        if let Some(ActionResponse::Closed(reason)) =
            self.response.take().and_then(|rx| rx.recv().ok())
        {
            handler.call(reason);
        }
    }
}

impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        &self.notification
    }
}

/// Allow to easily modify notification properties
impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.notification
    }
}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    Ok(NotificationHandle::new(
        notification.clone(),
        spawn_response_listener(notification),
    ))
}

pub(crate) fn schedule_notification(
    notification: &Notification,
    delivery_date: f64,
) -> Result<NotificationHandle> {
    let mut n = mac_notification_sys::Notification::default();
    n.title(notification.summary.as_str())
        .message(&notification.body)
        .maybe_subtitle(notification.subtitle.as_deref())
        .maybe_sound(notification.sound_name.as_deref())
        .delivery_date(delivery_date);

    if let Some(ref image_path) = notification.path_to_image {
        n.content_image(image_path);
    }

    n.send()?;

    // mac-notification-sys exposes responses for immediate notifications only.
    // Keep scheduled notifications on the existing scheduling path; the
    // returned handle has no response to receive.
    let (_sender, receiver) = mpsc::channel();
    Ok(NotificationHandle::new(notification.clone(), receiver))
}

fn spawn_response_listener(notification: &Notification) -> Receiver<ActionResponse> {
    let (sender, receiver) = mpsc::channel();
    let title = notification.summary.clone();
    let subtitle = notification.subtitle.clone();
    let message = notification.body.clone();
    let sound = notification.sound_name.clone();
    let image = notification.path_to_image.clone();
    let actions = notification.actions.clone();

    thread::spawn(move || {
        // `mac-notification-sys` blocks here until macOS reports the user's
        // response. Keeping it on this dedicated thread means showing a
        // notification is non-blocking for the caller; the single result is
        // delivered through the handle's channel.
        let mut options = mac_notification_sys::Notification::default();
        options
            .title(title.as_str())
            .message(message.as_str())
            .maybe_subtitle(subtitle.as_deref())
            .maybe_sound(sound.as_deref());
        if let Some(ref image_path) = image {
            options.content_image(image_path);
        }

        let response = match options.send() {
            Ok(response) => map_response(response, &actions),
            Err(_) => ActionResponse::Closed(CloseReason::Other(0)),
        };

        let _ = sender.send(response);
    });

    receiver
}

/// Map `mac-notification-sys`'s response into notify-rust's action model.
///
/// Clicking the notification body maps to `"default"`; action buttons are
/// mapped back to the identifier passed to [`Notification::action`] when the
/// reported button title matches a configured label.
fn map_response(
    response: mac_notification_sys::NotificationResponse,
    actions: &[String],
) -> ActionResponse {
    use mac_notification_sys::NotificationResponse as Response;

    match response {
        Response::Click => ActionResponse::Custom("default".to_owned()),
        Response::ActionButton(label) | Response::Reply(label) => {
            ActionResponse::Custom(action_identifier_for_label(&label, actions))
        }
        Response::CloseButton(_) => ActionResponse::Closed(CloseReason::Dismissed),
        Response::None => ActionResponse::Closed(CloseReason::Expired),
    }
}

fn action_identifier_for_label(action: &str, actions: &[String]) -> String {
    actions
        .chunks(2)
        .find_map(|chunk| match chunk {
            [identifier, label] if label == action => Some(identifier.clone()),
            _ => None,
        })
        .unwrap_or_else(|| action.to_owned())
}

#[cfg(test)]
mod tests {
    use super::{map_response, ActionResponse, CloseReason};
    use mac_notification_sys::NotificationResponse as Response;

    #[test]
    fn maps_click_to_default_action() {
        match map_response(Response::Click, &[]) {
            ActionResponse::Custom(action) => assert_eq!(action, "default"),
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn maps_button_title_to_action_identifier() {
        let actions = vec!["open".to_owned(), "Open".to_owned()];

        match map_response(Response::ActionButton("Open".to_owned()), &actions) {
            ActionResponse::Custom(action) => assert_eq!(action, "open"),
            other => panic!("unexpected response: {other:?}"),
        }
    }

    #[test]
    fn maps_close_to_dismissed() {
        match map_response(Response::CloseButton("Close".to_owned()), &[]) {
            ActionResponse::Closed(CloseReason::Dismissed) => {}
            other => panic!("unexpected response: {other:?}"),
        }
    }
}
