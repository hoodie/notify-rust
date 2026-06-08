use std::{
    ops::{Deref, DerefMut},
    path::Path,
    str::FromStr,
    sync::mpsc::{self, Receiver, Sender},
};

use winrt_notification::{Toast, ToastDismissalReason};

pub use crate::{error::*, notification::Notification, timeout::Timeout, urgency::Urgency};

#[derive(Copy, Clone, Debug)]
pub enum CloseReason {
    Expired,
    Dismissed,
    CloseAction,
    Other,
}

enum ToastEvent {
    Activated(Option<String>),
    Dismissed(CloseReason),
}

pub struct NotificationHandle {
    events: Receiver<ToastEvent>,
    notification: Notification,
}

impl NotificationHandle {
    pub fn wait_for_action<F>(self, invocation_closure: F)
    where
        F: FnOnce(&str),
    {
        match self.events.recv() {
            Ok(ToastEvent::Activated(Some(action))) => invocation_closure(action.as_str()),
            Ok(ToastEvent::Activated(None)) => invocation_closure("default"),
            Ok(ToastEvent::Dismissed(_)) | Err(_) => invocation_closure("__closed"),
        }
    }

    pub fn on_close<F>(self, handler: F)
    where
        F: FnOnce(CloseReason),
    {
        while let Ok(event) = self.events.recv() {
            if let ToastEvent::Dismissed(reason) = event {
                handler(reason);
                break;
            }
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

impl From<Option<ToastDismissalReason>> for CloseReason {
    fn from(reason: Option<ToastDismissalReason>) -> Self {
        match reason {
            Some(ToastDismissalReason::TimedOut) => CloseReason::Expired,
            Some(ToastDismissalReason::UserCanceled) => CloseReason::Dismissed,
            Some(ToastDismissalReason::ApplicationHidden) => CloseReason::CloseAction,
            None => CloseReason::Other,
        }
    }
}

fn build_toast(
    notification: &Notification,
    event_tx: Option<Sender<ToastEvent>>,
) -> Result<Toast> {
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

    let scenario = match notification.urgency {
        Some(Urgency::Critical) => winrt_notification::Scenario::Reminder,
        Some(Urgency::Low) | Some(Urgency::Normal) | None => winrt_notification::Scenario::Default,
    };

    let app_id = notification.app_id.as_deref().unwrap_or(Toast::POWERSHELL_APP_ID);
    let mut toast = Toast::new(app_id)
        .title(&notification.summary)
        .text1(notification.subtitle.as_deref().unwrap_or(""))
        .text2(&notification.body)
        .sound(sound)
        .duration(duration)
        .scenario(scenario);

    if let Some(image_path) = &notification.path_to_image {
        toast = toast.image(Path::new(image_path), "");
    }

    for action in notification.actions.chunks_exact(2) {
        toast = toast.add_button(&action[1], &action[0]);
    }

    if let Some(action_tx) = event_tx {
        let dismissed_tx = action_tx.clone();
        toast = toast
            .on_activated(move |action| {
                let _ = action_tx.send(ToastEvent::Activated(action));
                Ok(())
            })
            .on_dismissed(move |reason| {
                let _ = dismissed_tx.send(ToastEvent::Dismissed(reason.into()));
                Ok(())
            });
    }

    Ok(toast)
}

pub(crate) fn show_notification(notification: &Notification) -> Result<()> {
    build_toast(notification, None)?
        .show()
        .map_err(|error| Error::from(ErrorKind::Msg(format!("{error:?}"))))
}

pub(crate) fn show_notification_handle(notification: &Notification) -> Result<NotificationHandle> {
    let (tx, rx) = mpsc::channel();
    let toast = build_toast(notification, Some(tx))?;

    toast
        .show()
        .map_err(|error| Error::from(ErrorKind::Msg(format!("{error:?}"))))?;

    Ok(NotificationHandle {
        events: rx,
        notification: notification.clone(),
    })
}

#[cfg(test)]
mod tests {
    use super::CloseReason;
    use winrt_notification::ToastDismissalReason;

    #[test]
    fn maps_missing_dismiss_reason_to_other() {
        assert!(matches!(
            CloseReason::from(None::<ToastDismissalReason>),
            CloseReason::Other
        ));
    }
}
