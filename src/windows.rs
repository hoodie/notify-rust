use winrt_notification::Toast;

pub use crate::{error::*, notification::Notification, timeout::Timeout, urgency::Urgency};

use std::{
    fmt,
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
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

/// Response to a Windows toast notification.
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

/// A handle to a shown Windows toast notification.
///
/// The current `tauri-winrt-notification` API used by notify-rust can display
/// toasts but does not expose toast activation or dismissal callbacks. The
/// listener methods are present so cross-platform code can compile; on Windows
/// they return without invoking the callback until the backend exposes an
/// activation API this crate can wire through.
pub struct NotificationHandle {
    notification: Notification,
}

impl NotificationHandle {
    #[allow(missing_docs)]
    pub fn new(notification: Notification) -> NotificationHandle {
        NotificationHandle { notification }
    }

    /// Waits for the user to act on a notification.
    ///
    /// Windows toast activation is not exposed by `tauri-winrt-notification`
    /// 0.7, so this currently returns without invoking `invocation_closure`.
    pub fn wait_for_action<F>(self, _invocation_closure: F)
    where
        F: FnOnce(&str),
    {
    }

    /// Runs a callback after Windows reports a toast response.
    ///
    /// Windows toast activation is not exposed by `tauri-winrt-notification`
    /// 0.7, so this currently returns without invoking `invocation_closure`.
    pub fn on_response<F>(self, _invocation_closure: F)
    where
        F: FnOnce(ActionResponse) + Send + 'static,
    {
    }

    /// Executes a closure after the notification has closed.
    ///
    /// Windows toast dismissal is not exposed by `tauri-winrt-notification`
    /// 0.7, so this currently returns without invoking `handler`.
    pub fn on_close<A>(self, _handler: impl CloseHandler<A>) {}
}

impl fmt::Debug for NotificationHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotificationHandle")
            .field("notification", &self.notification)
            .finish()
    }
}

impl Deref for NotificationHandle {
    type Target = Notification;

    fn deref(&self) -> &Notification {
        &self.notification
    }
}

/// Allow to easily modify notification properties.
impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Notification {
        &mut self.notification
    }
}

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    let sound = match &notification.sound_name {
        Some(chosen_sound_name) => winrt_notification::Sound::from_str(chosen_sound_name).ok(),
        None => None,
    };

    let duration = match notification.timeout {
        Timeout::Default => winrt_notification::Duration::Short,
        Timeout::Never => winrt_notification::Duration::Long,
        Timeout::Milliseconds(t) => {
            if t >= 25000 {
                winrt_notification::Duration::Long
            } else {
                winrt_notification::Duration::Short
            }
        }
    };

    // Map urgency to Windows toast scenario
    // Low/Normal -> Default (standard behavior)
    // Critical -> Reminder (stays on screen until dismissed, matching XDG spec)
    let scenario = match notification.urgency {
        Some(Urgency::Critical) => Some(winrt_notification::Scenario::Reminder),
        Some(Urgency::Low) | Some(Urgency::Normal) | None => None, // Default scenario
    };

    let powershell_app_id = &Toast::POWERSHELL_APP_ID.to_string();
    let app_id = &notification.app_id.as_ref().unwrap_or(powershell_app_id);
    let mut toast = Toast::new(app_id)
        .title(&notification.summary)
        .text1(notification.subtitle.as_ref().map_or("", AsRef::as_ref)) // subtitle
        .text2(&notification.body)
        .sound(sound)
        .duration(duration);

    // Apply scenario only for critical urgency
    if let Some(scenario) = scenario {
        toast = toast.scenario(scenario);
    }
    if let Some(image_path) = &notification.path_to_image {
        toast = toast.image(Path::new(&image_path), "");
    }

    toast
        .show()
        .map_err(|error| Error::from(ErrorKind::Msg(format!("{error:?}"))))?;

    Ok(NotificationHandle::new(notification.clone()))
}
