use crate::{
    error::*,
    notification::{action_pairs, delivery_date_is_in_past, Notification},
};

use mac_notification_sys::{MainButton, NotificationResponse};

pub use mac_notification_sys::error::{ApplicationError, Error as MacOsError, NotificationError};

use std::ops::{Deref, DerefMut};
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

/// A handle to a shown notification.
///
/// This keeps a connection alive to ensure actions work on certain desktops.
pub struct NotificationHandle {
    notification: Notification,
    action_response: Receiver<Option<String>>,
}

impl std::fmt::Debug for NotificationHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotificationHandle")
            .field("notification", &self.notification)
            .finish_non_exhaustive()
    }
}

impl NotificationHandle {
    #[allow(missing_docs)]
    pub fn new(notification: Notification, action_response: Receiver<Option<String>>) -> Self {
        Self {
            notification,
            action_response,
        }
    }

    /// Waits for the user to act on a notification and then calls
    /// `invocation_closure` with the name of the corresponding action.
    ///
    /// Clicking the notification body returns `"default"`. Closing the notification returns
    /// `"__closed"` for compatibility with the XDG backend.
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        if let Ok(Some(action)) = self.action_response.recv() {
            invocation_closure(&action);
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
    let notification = notification.clone();
    let handle_notification = notification.clone();
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let response = send_waiting_notification(&notification, None);
        let _ = sender.send(
            response
                .ok()
                .and_then(|response| response_to_action(&response, &notification.actions)),
        );
    });

    Ok(NotificationHandle::new(handle_notification, receiver))
}

pub(crate) fn schedule_notification(
    notification: &Notification,
    delivery_date: f64,
) -> Result<NotificationHandle> {
    validate_delivery_date(delivery_date)?;

    let notification = notification.clone();
    let handle_notification = notification.clone();
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let response = send_waiting_notification(&notification, Some(delivery_date));
        let _ = sender.send(
            response
                .ok()
                .and_then(|response| response_to_action(&response, &notification.actions)),
        );
    });

    Ok(NotificationHandle::new(handle_notification, receiver))
}

fn send_waiting_notification(
    notification: &Notification,
    delivery_date: Option<f64>,
) -> std::result::Result<NotificationResponse, MacOsError> {
    let mut n = mac_notification_sys::Notification::default();
    n.title(notification.summary.as_str())
        .message(&notification.body)
        .maybe_subtitle(notification.subtitle.as_deref())
        .maybe_sound(notification.sound_name.as_deref())
        .wait_for_click(true);

    if let Some(delivery_date) = delivery_date {
        n.delivery_date(delivery_date);
    }

    let action_labels = action_pairs(&notification.actions)
        .map(|(_identifier, label)| label)
        .collect::<Vec<_>>();
    match action_labels.as_slice() {
        [] => {}
        [label] => {
            n.main_button(MainButton::SingleAction(label));
        }
        labels => {
            n.main_button(MainButton::DropdownActions("Actions", labels));
        }
    }

    if let Some(ref image_path) = notification.path_to_image {
        n.content_image(image_path);
    }

    n.send()
}

fn validate_delivery_date(delivery_date: f64) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs_f64())
        .unwrap_or(0.0);

    if delivery_date_is_in_past(delivery_date, now) {
        return Err(NotificationError::ScheduleInThePast.into());
    }

    Ok(())
}

fn response_to_action(response: &NotificationResponse, actions: &[String]) -> Option<String> {
    match response {
        NotificationResponse::Click => Some("default".to_owned()),
        NotificationResponse::ActionButton(label) => action_pairs(actions)
            .find_map(|(identifier, action_label)| {
                (action_label == label).then(|| identifier.to_owned())
            })
            .or_else(|| Some(label.clone())),
        NotificationResponse::CloseButton(_) => Some("__closed".to_owned()),
        NotificationResponse::Reply(reply) => Some(reply.clone()),
        NotificationResponse::None => None,
    }
}
