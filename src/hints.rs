//! `NotificationHints` allow to pass extra information to the server.
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

use super::NotificationUrgency;

#[cfg(all(unix, not(target_os = "macos")))] use std::collections::{HashMap, HashSet};
#[cfg(all(unix, not(target_os = "macos")))] use dbus::{MessageItem, arg::RefArg};

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use dbus::MessageItemArray;

use miniver::Version;

#[cfg(all(unix, not(target_os = "macos")))]
use util::*;

use std::cmp::Ordering;

/// "action-icons"
pub const ACTION_ICONS: &str    = "action-icons";

/// "category"
pub const CATEGORY: &str        = "category";

/// "desktop-entry"
pub const DESKTOP_ENTRY: &str   = "desktop-entry";

/// "image-data" if spec_version > 1.1;
pub const IMAGE_DATA: &str      = "image-data";

/// "image_data" if spec_version == 1.1
pub const IMAGE_DATA_1_1: &str = "image_data";

/// "image-data" if spec_version < 1.1;
pub const IMAGE_DATA_1_0: &str = "icon_data";

/// "image-path"
pub const IMAGE_PATH: &str      = "image-path";

/// "resident"
pub const RESIDENT: &str        = "resident";

/// "sound-file"
pub const SOUND_FILE: &str      = "sound-file";

/// "sound-name"
pub const SOUND_NAME: &str      = "sound-name";

/// "suppress-sound"
pub const SUPPRESS_SOUND: &str  = "suppress-sound";

/// "transient"
pub const TRANSIENT: &str       = "transient";

/// "x"
pub const X: &str               = "x";

/// "y"
pub const Y: &str               = "y";

/// "urgency"
pub const URGENCY: &str         = "urgency";

/// Raw image data as represented on dbus
#[derive(PartialEq, Eq, Debug, Clone, Hash)]
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
pub struct NotificationImage {
    width:           i32,
    height:          i32,
    rowstride:       i32,
    alpha:           bool,
    bits_per_sample: i32,
    channels:        i32,
    data:            Vec<u8>
}

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
impl NotificationImage {
    /// Creates an image from a raw vector of bytes
    pub fn from_rgb(width: i32, height: i32, data: Vec<u8>) -> Result<Self, ImageError> {
        const MAX_SIZE: i32 = 0x0fff_ffff;
        if width > MAX_SIZE || height > MAX_SIZE {
            return Err(ImageError::TooBig);
        }

        let channels = 3i32;
        let bits_per_sample = 8;

        if data.len() != (width * height * channels) as usize {
            Err(ImageError::WrongDataSize)
        } else {
            Ok(Self {
                width,
                height,
                bits_per_sample,
                channels,
                data,
                rowstride: width * channels,
                alpha: false,
            })
        }
    }
}

/// Errors that can occour when creating an Image
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
pub enum ImageError {
    /// The given image is too big. DBus only has 32 bits for width / height
    TooBig,
    /// The given bytes don't match the width, height and channel count
    WrongDataSize
}

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
impl From<NotificationImage> for MessageItem {
    fn from(img: NotificationImage) -> Self {
        let bytes = img.data.into_iter().map(MessageItem::Byte).collect();

        MessageItem::Struct(vec![MessageItem::Int32(img.width),
                                 MessageItem::Int32(img.height),
                                 MessageItem::Int32(img.rowstride),
                                 MessageItem::Bool(img.alpha),
                                 MessageItem::Int32(img.bits_per_sample),
                                 MessageItem::Int32(img.channels),
                                 MessageItem::Array(MessageItemArray::new(bytes, "ay".into()).unwrap()  )
                                ])
    }
}

/// All currently implemented `NotificationHints` that can be send.
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

    /// 	A themeable named sound from the freedesktop.org [sound naming specification](http://0pointer.de/public/sound-naming-spec.html) to play when the notification pops up. Similar to icon-name, only for sounds. An example would be "message-new-instant".
    SoundName(String),

    /// Suppress the notification sound.
    SuppressSound(bool),

    /// When set the server will treat the notification as transient and by-pass the server's persistence capability, if it should exist. When set the server will treat the notification as transient and by-pass the server's persistence capability, if it should exist.
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

/// convinience converting a name and value into a hint
pub fn hint_from_key_val(name: &str, value: &str) -> Result<NotificationHint, String> {
    use NotificationHint as Hint;
    match (name,value){
        (ACTION_ICONS,val)    => val.parse::<bool>().map(Hint::ActionIcons).map_err(|e|e.to_string()),
        (CATEGORY, val)       => Ok(Hint::Category(val.to_owned())),
        (DESKTOP_ENTRY, val)  => Ok(Hint::DesktopEntry(val.to_owned())),
        (IMAGE_PATH, val)     => Ok(Hint::ImagePath(val.to_owned())),
        (RESIDENT, val)       => val.parse::<bool>().map(Hint::Resident).map_err(|e|e.to_string()),
        (SOUND_FILE, val)     => Ok(Hint::SoundFile(val.to_owned())),
        (SOUND_NAME, val)     => Ok(Hint::SoundName(val.to_owned())),
        (SUPPRESS_SOUND, val) => val.parse::<bool>().map(Hint::SuppressSound).map_err(|e|e.to_string()),
        (TRANSIENT, val)      => val.parse::<bool>().map(Hint::Transient).map_err(|e|e.to_string()),
        (X, val)              => val.parse::<i32>().map(Hint::X).map_err(|e|e.to_string()),
        (Y, val)              => val.parse::<i32>().map(Hint::Y).map_err(|e|e.to_string()),
        _                     => Err(String::from("unknown name"))
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl NotificationHint {}

/// matching image data key for each spec version
pub fn image_spec(version: Version) -> String {
    match version.cmp(&Version::new(1, 1)) {
        Ordering::Less => IMAGE_DATA_1_0.to_owned(),
        Ordering::Equal => IMAGE_DATA_1_1.to_owned(),
        Ordering::Greater => IMAGE_DATA.to_owned()
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl<'a> From<&'a NotificationHint> for MessageItem {
    fn from(hint: &'a NotificationHint) -> Self {
        let hint:(String,MessageItem) = match *hint {
            NotificationHint::ActionIcons(value)       => (ACTION_ICONS   .to_owned(), MessageItem::Bool(value)), // bool
            NotificationHint::Category(ref value)      => (CATEGORY       .to_owned(), MessageItem::Str(value.clone())),
            NotificationHint::DesktopEntry(ref value)  => (DESKTOP_ENTRY  .to_owned(), MessageItem::Str(value.clone())),
            #[cfg(all(feature = "images", unix, not(target_os ="macos")))]
            NotificationHint::ImageData(ref image)     => (image_spec(*::SPEC_VERSION), image.clone().into()),
            NotificationHint::ImagePath(ref value)     => (IMAGE_PATH     .to_owned(), MessageItem::Str(value.clone())),
            NotificationHint::Resident(value)          => (RESIDENT       .to_owned(), MessageItem::Bool(value)), // bool
            NotificationHint::SoundFile(ref value)     => (SOUND_FILE     .to_owned(), MessageItem::Str(value.clone())),
            NotificationHint::SoundName(ref value)     => (SOUND_NAME     .to_owned(), MessageItem::Str(value.clone())),
            NotificationHint::SuppressSound(value)     => (SUPPRESS_SOUND .to_owned(), MessageItem::Bool(value)),
            NotificationHint::Transient(value)         => (TRANSIENT      .to_owned(), MessageItem::Bool(value)),
            NotificationHint::X(value)                 => (X              .to_owned(), MessageItem::Int32(value)),
            NotificationHint::Y(value)                 => (Y              .to_owned(), MessageItem::Int32(value)),
            NotificationHint::Urgency(value)           => (URGENCY        .to_owned(), MessageItem::Byte(value as u8)),
            NotificationHint::Custom(ref key, ref val) => (key            .to_owned(), MessageItem::Str(val.to_owned ())),
            NotificationHint::CustomInt(ref key, val)  => (key            .to_owned(), MessageItem::Int32(val)),
            NotificationHint::Invalid                  => ("invalid"      .to_owned(), MessageItem::Str("Invalid".to_owned()))
        };

        MessageItem::DictEntry(
            Box::new(hint.0.into()),
            Box::new(MessageItem::Variant( Box::new(hint.1) ))
            )
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl<'a> From<&'a MessageItem> for NotificationHint {
    fn from (item: &MessageItem) -> NotificationHint {
        match item{
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == CATEGORY       => NotificationHint::Category(unwrap_message_str(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == ACTION_ICONS   => NotificationHint::ActionIcons(unwrap_message_bool(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == DESKTOP_ENTRY  => NotificationHint::DesktopEntry(unwrap_message_str(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == IMAGE_PATH     => NotificationHint::ImagePath(unwrap_message_str(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == RESIDENT       => NotificationHint::Resident(unwrap_message_bool(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == SOUND_FILE     => NotificationHint::SoundFile(unwrap_message_str(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == SOUND_NAME     => NotificationHint::SoundName(unwrap_message_str(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == SUPPRESS_SOUND => NotificationHint::SuppressSound(unwrap_message_bool(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == TRANSIENT      => NotificationHint::Transient(unwrap_message_bool(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == X              => NotificationHint::X(unwrap_message_int(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == Y              => NotificationHint::Y(unwrap_message_int(&**value)),
            &MessageItem::DictEntry(ref key, ref value) if unwrap_message_str(&**key) == URGENCY        => NotificationHint::Urgency(
                match unwrap_message_int(&**value){
                    0 => NotificationUrgency::Low,
                    2 => NotificationUrgency::Critical,
                    _ => NotificationUrgency::Normal
                }),
            &MessageItem::DictEntry(ref key, ref value) => match try_unwrap_message_int(value) {
                    Some(num) => NotificationHint::CustomInt(unwrap_message_str(&**key), num),
                    None => NotificationHint::Custom(unwrap_message_str(&**key), unwrap_message_str(&**value))
            }
            other => {
                println!("Invalid {:#?} ", other);
                NotificationHint::Invalid
            }
        }
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl<'a, A:RefArg> From<(&'a String, &'a A)> for NotificationHint {
    fn from(pair: (&String, &A)) -> Self {

        let (key, variant) = pair;
        match (key.as_ref(), variant.as_u64(), variant.as_i64(), variant.as_str().map(String::from)) {

            (ACTION_ICONS,   Some(1),  _,       _          ) => NotificationHint::ActionIcons(true),
            (ACTION_ICONS,   _,        _,       _          ) => NotificationHint::ActionIcons(false),
            (URGENCY,        level,    _,       _          ) => NotificationHint::Urgency(level.into()),
            (CATEGORY,       _,        _,       Some(name) ) => NotificationHint::Category(name),

            (DESKTOP_ENTRY,  _,        _,       Some(entry)) => NotificationHint::DesktopEntry(entry),
            (IMAGE_PATH,     _,        _,       Some(path) ) => NotificationHint::ImagePath(path),
            (RESIDENT,       Some(1),  _,       _          ) => NotificationHint::Resident(true),
            (RESIDENT,       _,        _,       _          ) => NotificationHint::Resident(false),

            (SOUND_FILE,     _,        _,       Some(path) ) => NotificationHint::SoundFile(path),
            (SOUND_NAME,     _,        _,       Some(name) ) => NotificationHint::SoundName(name),
            (SUPPRESS_SOUND, Some(1),  _,       _          ) => NotificationHint::SuppressSound(true),
            (SUPPRESS_SOUND, _,        _,       _          ) => NotificationHint::SuppressSound(false),
            (TRANSIENT,      Some(1),  _,       _          ) => NotificationHint::Transient(true),
            (TRANSIENT,      _,        _,       _          ) => NotificationHint::Transient(false),
            (X,              _,        Some(x), _          ) => NotificationHint::X(x as i32),
            (Y,              _,        Some(y), _          ) => NotificationHint::Y(y as i32),

            other => {
                eprintln!("Invalid NotificationHint {:#?} ", other);
                NotificationHint::Invalid
            }
        }
    }
}

#[allow(missing_docs)]
#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) fn hints_from_variants<A: RefArg>(hints: &HashMap<String, A>) -> HashSet<NotificationHint> {
    hints.iter().map(Into::into).collect()
}

#[cfg(all(test, unix, not(target_os = "macos")))]
mod test {
    use dbus::MessageItem as Item;

    use super::*;
    use super::NotificationHint as Hint;
    use super::NotificationUrgency::*;

    #[test]
    fn hint_to_item() {
        let category  = &Hint::Category("testme".to_owned());
        let item:Item = category.into();
        let test_item = Item::DictEntry(
            Box::new(Item::Str("category".into())),
            Box::new(Item::Variant(  Box::new(Item::Str("testme".into()))  ))
            );
        assert_eq!(item, test_item);
    }

    #[test]
    fn urgency() {
        let low = &Hint::Urgency(Low);
        let low_item:Item = low.into();
        let test_item = Item::DictEntry(
            Box::new(Item::Str("urgency".into())),
            Box::new(Item::Variant(  Box::new(Item::Byte(0)))  ));
        assert_eq!(low_item, test_item);
    }

    #[test]
    fn simple_hint_to_item() {
        let old_hint = &NotificationHint::Custom("foo".into(), "bar".into());
        let item: MessageItem = old_hint.into();
        let item_ref = &item;
        let hint: NotificationHint = item_ref.into();
        assert!(old_hint == &hint);
    }

    #[test]
    #[cfg(all(feature = "images", unix, not(target_os = "macos")))]
    fn imagedata_hint_to_item() {
        let hint = &NotificationHint::ImageData(NotificationImage::from_rgb(1, 1, vec![0, 0, 0]).unwrap());
        let item: MessageItem = hint.into();
        let test_item = Item::DictEntry(
            Box::new(Item::Str(image_spec(*::SPEC_VERSION))),
            Box::new(Item::Variant(Box::new(Item::Struct(vec![
                Item::Int32(1),
                Item::Int32(1),
                Item::Int32(3),
                Item::Bool(false),
                Item::Int32(8),
                Item::Int32(3),
                Item::Array(vec![
                    Item::Byte(0),
                    Item::Byte(0),
                    Item::Byte(0),
                ],"y".into())
            ]))))
        );
        assert_eq!(item, test_item);
    }

    #[test]
    #[cfg(all(feature = "images", unix, not(target_os = "macos")))]
    fn imagedata_hint_to_item_with_spec() {
        let key = image_spec(Version::new(1, 0));
        assert_eq!(key, String::from("icon_data"));

        let key = image_spec(Version::new(1, 1));
        assert_eq!(key, String::from("image_data"));

        let key = image_spec(Version::new(1, 2));
        assert_eq!(key, String::from("image-data"));
    }
}
