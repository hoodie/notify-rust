use crate::{error::*, notification::Notification, ActionResponse, CloseHandler, CloseReason};

pub use mac_notification_sys::error::{ApplicationError, Error as MacOsError, NotificationError};
use mac_notification_sys::{MainButton, NotificationResponse};

use std::ops::{Deref, DerefMut};

/// A handle to a notification.
///
/// On macOS this also keeps the source [`Notification`] around so that
/// [`NotificationHandle::wait_for_action`] / [`NotificationHandle::on_close`]
/// can later drive the synchronous response flow exposed by
/// `mac_notification_sys`.
///
/// When the notification has any [`Notification::action`] configured, the
/// actual delivery is **deferred** until `wait_for_action` (or `on_close`) is
/// invoked: the underlying `mac_notification_sys::Notification::send` call is
/// blocking and only returns once the user interacted with the notification,
/// so it has to be issued at the point where the response is observable.
///
/// Notifications with no actions retain the original fire-and-forget semantics
/// — `show()` already delivered them and the resulting handle is essentially
/// inert.
#[derive(Debug)]
pub struct NotificationHandle {
    notification: Notification,
    /// `true` while the underlying notification has not been sent yet —
    /// i.e. an interactive notification that is awaiting `wait_for_action` /
    /// `on_close`. `false` once the notification has been delivered (either
    /// fire-and-forget or after a `wait_for_action` has run).
    pending: bool,
}

impl NotificationHandle {
    #[allow(missing_docs)]
    pub fn new(notification: Notification) -> NotificationHandle {
        NotificationHandle {
            notification,
            pending: false,
        }
    }

    fn pending(notification: Notification) -> NotificationHandle {
        NotificationHandle {
            notification,
            pending: true,
        }
    }

    /// Wait for the user to act on the notification and call the closure with
    /// the identifier of the activated action.
    ///
    /// The identifier is the first argument originally passed to
    /// [`Notification::action`]. If the notification was closed without any
    /// action being clicked, the closure is invoked with the magic
    /// `"__closed"` string for parity with the XDG backend (this sentinel is
    /// scheduled to be replaced by an [`ActionResponse::Closed`] variant in
    /// 5.0).
    ///
    /// On macOS this is implemented on top of
    /// `mac_notification_sys::Notification::wait_for_click`, which delivers
    /// the notification synchronously and blocks until the user interacts
    /// with it. The first time this method (or `on_close`) is called on a
    /// pending interactive handle, that synchronous send is issued.
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        if !self.pending {
            // Fire-and-forget notifications are already delivered and gone.
            // There is nothing to wait on, so emit the close sentinel for
            // compatibility with the XDG semantics.
            invocation_closure("__closed");
            return;
        }

        let response = self.show_blocking();
        let identifier: String = match response {
            Ok(NotificationResponse::ActionButton(label)) => {
                match self.identifier_for_label(&label) {
                    Some(id) => id.to_owned(),
                    None => label,
                }
            }
            Ok(NotificationResponse::Click) => self
                .first_identifier()
                .map_or_else(|| "__closed".to_owned(), str::to_owned),
            Ok(NotificationResponse::Reply(text)) => text,
            Ok(NotificationResponse::CloseButton(_)) | Ok(NotificationResponse::None) => {
                "__closed".to_owned()
            }
            Err(_) => "__closed".to_owned(),
        };
        invocation_closure(&identifier);
    }

    /// Execute a closure after the notification has closed.
    ///
    /// On macOS the close reason is always reported as
    /// [`CloseReason::Dismissed`], because the underlying notification system
    /// does not distinguish between expiry, user dismissal and programmatic
    /// close.
    ///
    /// Mirrors the behaviour of `wait_for_action` regarding pending
    /// notifications: an interactive (action-bearing) notification will be
    /// delivered now and this call will block until the user closes it; a
    /// fire-and-forget notification will return immediately because it has
    /// already been delivered.
    pub fn on_close<A>(self, handler: impl CloseHandler<A>) {
        if self.pending {
            let _ = self.show_blocking();
        }
        handler.call(CloseReason::Dismissed);
    }

    /// Drive the underlying `mac_notification_sys` send, blocking until the
    /// user interacts with the notification (or it is dismissed).
    fn show_blocking(&self) -> Result<NotificationResponse> {
        let labels: Vec<&str> = self
            .notification
            .actions
            .chunks(2)
            .filter_map(|c| c.get(1).map(String::as_str))
            .collect();

        let mut n = mac_notification_sys::Notification::default();
        n.title(self.notification.summary.as_str())
            .message(&self.notification.body)
            .maybe_subtitle(self.notification.subtitle.as_deref())
            .maybe_sound(self.notification.sound_name.as_deref())
            .wait_for_click(true);

        if let Some(ref image_path) = self.notification.path_to_image {
            n.content_image(image_path);
        }

        match labels.len() {
            0 => {}
            1 => {
                n.main_button(MainButton::SingleAction(labels[0]));
            }
            _ => {
                n.main_button(MainButton::DropdownActions(labels[0], &labels[1..]));
            }
        }

        n.send().map_err(Into::into)
    }

    fn identifier_for_label(&self, label: &str) -> Option<&str> {
        self.notification
            .actions
            .chunks(2)
            .find_map(|chunk| match (chunk.first(), chunk.get(1)) {
                (Some(id), Some(lbl)) if lbl == label => Some(id.as_str()),
                _ => None,
            })
    }

    fn first_identifier(&self) -> Option<&str> {
        self.notification.actions.first().map(String::as_str)
    }
}

/// Mirror of the XDG handle, used so that `ActionResponse` is part of the
/// public macOS API surface even though it is not constructed directly here.
#[allow(dead_code)]
fn _action_response_is_used(_response: &ActionResponse) {}

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
    if notification.actions.is_empty() {
        let mut n = mac_notification_sys::Notification::default();
        n.title(notification.summary.as_str())
            .message(&notification.body)
            .maybe_subtitle(notification.subtitle.as_deref())
            .maybe_sound(notification.sound_name.as_deref());

        if let Some(ref image_path) = notification.path_to_image {
            n.content_image(image_path);
        }

        n.send()?;

        Ok(NotificationHandle::new(notification.clone()))
    } else {
        Ok(NotificationHandle::pending(notification.clone()))
    }
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

    Ok(NotificationHandle::new(notification.clone()))
}
