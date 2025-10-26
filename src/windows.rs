use winrt_notification::Toast;

pub use crate::{error::*, notification::Notification, timeout::Timeout, urgency::Urgency};

use std::{path::Path, str::FromStr};

pub(crate) fn show_notification(notification: &Notification) -> Result<()> {
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
        .map_err(|error| Error::from(ErrorKind::Msg(format!("{error:?}"))))
}
