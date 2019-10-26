//! `NotificationHints` allow you to pass extra information to the server.
//!
//! Many of these are standardized by either:
//!
//! * http://www.galago-project.org/specs/notification/0.9/x344.html
//! * https://developer.gnome.org/notification-spec/#hints
//!
//! Which of these are actually implemented depends strongly on the Notification server you talk to.
//! Usually the `get_capabilities()` gives some clues, but the standards usually mention much more
//! than is actually available.
#![cfg_attr(rustfmt, rustfmt_skip)]

mod constants;

#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) mod urgency;

#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) mod message;

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
mod image;

use self::urgency::NotificationUrgency;

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
pub use self::image::{NotificationImage, image_spec};

/// All currently implemented `NotificationHints` that can be sent.
///
/// as found on https://developer.gnome.org/notification-spec/
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum NotificationHint {
    /// If true, server may interpret action identifiers as named icons and display those.
    ActionIcons(bool),

    /// Check out:
    ///
    /// * http://www.galago-project.org/specs/notification/0.9/x211.html
    /// * https://developer.gnome.org/notification-spec/#categories
    Category(String),

    /// Name of the DesktopEntry representing the calling application. In case of "firefox.desktop"
    /// use "firefox". May be used to retrieve the correct icon.
    DesktopEntry(String),

    /// Image as raw data
    #[cfg(all(feature = "images", unix, not(target_os = "macos")))]
    ImageData(NotificationImage),

    /// Display the image at this path.
    ImagePath(String),

    /// This does not work on all servers, however timeout=0 will do the job
    Resident(bool),

    /// Play the sound at this path.
    SoundFile(String),

    /// A themeable named sound from the freedesktop.org [sound naming specification](http://0pointer.de/public/sound-naming-spec.html) to play when the notification pops up. Similar to icon-name, only for sounds. An example would be "message-new-instant".
    SoundName(String),

    /// Suppress the notification sound.
    SuppressSound(bool),

    /// When set the server will treat the notification as transient and by-pass the server's persistence capability, if it should exist.
    Transient(bool),

    /// Lets the notification point to a certain 'x' position on the screen.
    /// Requires `Y`.
    X(i32),

    /// Lets the notification point to a certain 'y' position on the screen.
    /// Requires `X`.
    Y(i32),

    /// Pass me a NotificationUrgency, either Low, Normal or Critical
    Urgency(NotificationUrgency),

    /// If you want to pass something entirely different.
    Custom(String, String),

    /// A custom numerical (integer) hint
    CustomInt(String, i32),

    /// Only used by this NotificationServer implementation
    Invalid // TODO find a better solution to this
}

impl NotificationHint {
    /// Get the `bool` representation of this hint.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            NotificationHint::ActionIcons(inner)
            | NotificationHint::Resident(inner)
            | NotificationHint::SuppressSound(inner)
            | NotificationHint::Transient(inner) => Some(inner),
            _ => None
        }
    }

    /// Get the `i32` representation of this hint.
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            NotificationHint::X(inner) | NotificationHint::Y(inner) => Some(inner),
            _ => None
        }
    }

    /// Get the `&str` representation of this hint.
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            NotificationHint::DesktopEntry(ref inner) |
            NotificationHint::ImagePath(ref inner)    |
            NotificationHint::SoundFile(ref inner)    |
            NotificationHint::SoundName(ref inner)    => Some(inner),
            _ => None
        }
    }
}

/// convenience converting a name and value into a hint
pub fn hint_from_key_val(name: &str, value: &str) -> Result<NotificationHint, String> {
    use crate::NotificationHint as Hint;
    match (name,value){
        (constants::ACTION_ICONS,val)    => val.parse::<bool>().map(Hint::ActionIcons).map_err(|e|e.to_string()),
        (constants::CATEGORY, val)       => Ok(Hint::Category(val.to_owned())),
        (constants::DESKTOP_ENTRY, val)  => Ok(Hint::DesktopEntry(val.to_owned())),
        (constants::IMAGE_PATH, val)     => Ok(Hint::ImagePath(val.to_owned())),
        (constants::RESIDENT, val)       => val.parse::<bool>().map(Hint::Resident).map_err(|e|e.to_string()),
        (constants::SOUND_FILE, val)     => Ok(Hint::SoundFile(val.to_owned())),
        (constants::SOUND_NAME, val)     => Ok(Hint::SoundName(val.to_owned())),
        (constants::SUPPRESS_SOUND, val) => val.parse::<bool>().map(Hint::SuppressSound).map_err(|e|e.to_string()),
        (constants::TRANSIENT, val)      => val.parse::<bool>().map(Hint::Transient).map_err(|e|e.to_string()),
        (constants::X, val)              => val.parse::<i32>().map(Hint::X).map_err(|e|e.to_string()),
        (constants::Y, val)              => val.parse::<i32>().map(Hint::Y).map_err(|e|e.to_string()),
        _                                => Err(String::from("unknown name"))
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl NotificationHint {}
