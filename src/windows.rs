use winrt_notification::Toast;

use crate::notification::action_pairs;
pub use crate::{error::*, notification::Notification, timeout::Timeout, urgency::Urgency};

use std::ops::{Deref, DerefMut};
use std::sync::mpsc::{self, Receiver};
use std::{path::Path, str::FromStr};

/// A handle to a shown Windows toast notification.
pub struct NotificationHandle {
    notification: Notification,
    action_response: Receiver<String>,
}

impl std::fmt::Debug for NotificationHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotificationHandle")
            .field("notification", &self.notification)
            .finish_non_exhaustive()
    }
}

impl NotificationHandle {
    pub(crate) fn new(notification: Notification, action_response: Receiver<String>) -> Self {
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
        if let Ok(action) = self.action_response.recv() {
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
    let (sender, receiver) = mpsc::channel();
    let activated_sender = sender.clone();
    let dismissed_sender = sender;

    let mut toast = Toast::new(app_id)
        .title(&notification.summary)
        .text1(notification.subtitle.as_ref().map_or("", AsRef::as_ref)) // subtitle
        .text2(&notification.body)
        .sound(sound)
        .duration(duration)
        .on_activated(move |action| {
            let _ = activated_sender.send(action.unwrap_or_else(|| "default".to_owned()));
            Ok(())
        })
        .on_dismissed(move |_reason| {
            let _ = dismissed_sender.send("__closed".to_owned());
            Ok(())
        });

    // Apply scenario only for critical urgency
    if let Some(scenario) = scenario {
        toast = toast.scenario(scenario);
    }
    if let Some(image_path) = &notification.path_to_image {
        toast = toast.image(Path::new(&image_path), "");
    }
    for (identifier, label) in action_pairs(&notification.actions) {
        toast = toast.add_button(label, identifier);
    }

    toast
        .show()
        .map_err(|error| Error::from(ErrorKind::Msg(format!("{error:?}"))))?;

    Ok(NotificationHandle::new(notification.clone(), receiver))
}
