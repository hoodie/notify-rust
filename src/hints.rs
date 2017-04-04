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

#[cfg(all(unix, not(target_os = "macos")))]
use dbus::MessageItem;
use super::NotificationUrgency;
#[cfg(all(unix, not(target_os = "macos")))]
use util::*;

use std::vec::Vec;
use std::cmp::Ordering;

/// "action-icons"
pub const ACTION_ICONS:&'static str   = "action-icons";
/// "category"
pub const CATEGORY:&'static str       = "category";
/// "desktop-entry"
pub const DESKTOP_ENTRY:&'static str  = "desktop-entry";
/// "image-data";
pub const IMAGE_DATA:&'static str     = "image-data";
/// "image_data" as of spec 1.1
pub const IMAGE_DATA_1_1: &'static str = "image_data";
/// "icon_data" as of spec < 1.1
pub const IMAGE_DATA_1_0: &'static str = "icon_data";
/// "image-path"
pub const IMAGE_PATH:&'static str     = "image-path";
/// "resident"
pub const RESIDENT:&'static str       = "resident";
/// "sound-file"
pub const SOUND_FILE:&'static str     = "sound-file";
/// "sound-name"
pub const SOUND_NAME:&'static str     = "sound-name";
/// "suppress-sound"
pub const SUPPRESS_SOUND:&'static str = "suppress-sound";
/// "transient"
pub const TRANSIENT:&'static str      = "transient";
/// "x"
pub const X:&'static str              = "x";
/// "y"
pub const Y:&'static str              = "y";
/// "urgency"
pub const URGENCY:&'static str        = "urgency";

/// Raw image data as represented on dbus
#[derive(PartialEq,Eq,Debug,Clone,Hash)]
pub struct NotificationImage {
    width: i32,
    height: i32,
    rowstride: i32,
    alpha: bool,
    bits_per_sample: i32,
    channels: i32,
    data: Vec<u8>
}

/// Errors that can occour when creating an Image
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ImageError {
    /// The given image is too big. DBus only has 32 bits for width / height 
    TooBig,
    /// The given bytes don't match the width, height and channel count
    WrongDataSize
}

impl NotificationImage {

    /// Creates an image from a raw vector of bytes
    pub fn from_rgb(width: u32, height: u32, data: Vec<u8>) -> Result<Self, ImageError> {
        if width > 0x0fff_ffff {
            return Err(ImageError::TooBig)
        }
        if height > 0x0fff_ffff {
            return Err(ImageError::TooBig)
        }
        let width = width as i32;
        let height = height as i32;
        if data.len() != (width * height * 3) as usize {
            return Err(ImageError::WrongDataSize)
        } else {
            return Ok(Self{
                width: width,
                height: height,
                rowstride: width * 3,
                alpha: false,
                bits_per_sample: 8,
                channels: 3,
                data: data,
            })
        }
    }

}

#[cfg(all(unix, not(target_os = "macos")))]
impl From<NotificationImage> for MessageItem {

    fn from(img : NotificationImage) -> Self {
         MessageItem::Struct(vec![
            MessageItem::Int32(img.width),
            MessageItem::Int32(img.height),
            MessageItem::Int32(img.rowstride),
            MessageItem::Bool(img.alpha),
            MessageItem::Int32(img.bits_per_sample),
            MessageItem::Int32(img.channels),
            MessageItem::Array(img.data.into_iter().map(MessageItem::Byte).collect(),"y".into())
        ])
    }
}
/// All currently implemented `NotificationHints` that can be send.
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum NotificationHint
{ // as found on https://developer.gnome.org/notification-spec/
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

    // ///Not yet implemented
    /// Image as raw data
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
    Custom(String,String),

    /// A custom numerical (integer) hint
    CustomInt(String, i32),

    /// Only used by this NotificationServer implementation
    Invalid // TODO find a better solution to this
}

impl NotificationHint {
    /// Get the `bool` representation of this hint.
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            NotificationHint::ActionIcons(inner)   |
            NotificationHint::Resident(inner)      |
            NotificationHint::SuppressSound(inner) |
            NotificationHint::Transient(inner)     => Some(inner),
            _ => None
        }
    }

    /// Get the `i32` representation of this hint.
    pub fn as_i32(&self) -> Option<i32> {
        match *self {
            NotificationHint::X(inner) |
            NotificationHint::Y(inner) => Some(inner),
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
pub fn hint_from_key_val(name: &str, value: &str) -> Result<NotificationHint, String>{
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
impl NotificationHint {
    /// converts into a MessageItem
    pub fn into_message_item(&self, spec_version: &str) -> MessageItem {
        let hint:(String,MessageItem) = match *self {
            NotificationHint::ActionIcons(value)   => (ACTION_ICONS   .to_owned(), MessageItem::Bool(value)), // bool
            NotificationHint::Category(ref value)      => (CATEGORY       .to_owned(), MessageItem::Str(value.clone())),
            NotificationHint::DesktopEntry(ref value)  => (DESKTOP_ENTRY  .to_owned(), MessageItem::Str(value.clone())),
            NotificationHint::ImageData(ref image)  => {
                let key = match spec_version.cmp("1.1") {
                    Ordering::Less => IMAGE_DATA_1_0.to_owned(),
                    Ordering::Equal => IMAGE_DATA_1_1.to_owned(),
                    Ordering::Greater => IMAGE_DATA.to_owned()
                };
                ( key, image.clone().into() )
            },
            NotificationHint::ImagePath(ref value)     => (IMAGE_PATH     .to_owned(), MessageItem::Str(value.clone())),
            NotificationHint::Resident(value)      => (RESIDENT       .to_owned(), MessageItem::Bool(value)), // bool
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
impl<'a> From<&'a NotificationHint> for MessageItem {
    fn from(hint: &'a NotificationHint) -> Self {
        hint.into_message_item("1.2")
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
                    None => NotificationHint::Custom(unwrap_message_str(&**key), unwrap_message_str(&**value)),
                },
            other => {println!("Invalid {:#?} ", other); NotificationHint::Invalid}
        }
    }
}

#[cfg(all(test, unix, not(target_os = "macos")))]
mod test{
    use super::*;
    use super::NotificationHint as Hint;
    use NotificationUrgency::*;
    use dbus::MessageItem as Item;

    #[test]
    fn hint_to_item() {
        let category = &Hint::Category("testme".to_owned());
        let item:Item= category.into();
        let test_item= Item::DictEntry(
            Box::new(Item::Str("category".into())),
            Box::new(Item::Variant(  Box::new(Item::Str("testme".into()))  ))
            );
        assert_eq!(item, test_item);
    }

    #[test]
    fn urgency() {
        let low = &Hint::Urgency(Low);
        let low_item:Item= low.into();
        let test_item= Item::DictEntry(
            Box::new(Item::Str("urgency".into())),
            Box::new(Item::Variant(  Box::new(Item::Byte(0)))  ));
        assert_eq!(low_item, test_item);
    }

    #[test]
    fn simple_hint_to_item() {
        let old_hint = &NotificationHint::Custom("foo".into(), "bar".into());
        let item:MessageItem = old_hint.into();
        let item_ref = &item;
        let hint:NotificationHint = item_ref.into();
        assert!(old_hint == &hint);
    }

    #[test]
    fn imagedata_hint_to_item() {
        let hint = &NotificationHint::ImageData(NotificationImage::from_rgb(1,1,vec![0,0,0]).unwrap());
        let item:MessageItem = hint.into();
        let test_item = Item::DictEntry(
            Box::new(Item::Str("image-data".into())),
            Box::new(Item::Variant( Box::new(Item::Struct(vec![
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
    fn imagedata_hint_to_item_with_spec() {
        let hint = &NotificationHint::ImageData(NotificationImage::from_rgb(1,1,vec![0,0,0]).unwrap());
        match hint.into_message_item("1.0") {
            Item::DictEntry(key,_) => {
                assert_eq!(*key, Item::Str("icon_data".into()))
            },
            _ => unreachable!()
        }
        match hint.into_message_item("1.1") {
            Item::DictEntry(key,_) => {
                assert_eq!(*key, Item::Str("image_data".into()))
            },
            _ => unreachable!()
        }
        match hint.into_message_item("1.2") {
            Item::DictEntry(key,_) => {
                assert_eq!(*key, Item::Str("image-data".into()))
            },
            _ => unreachable!()
        }

    }
}
