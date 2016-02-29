//! NotificationHints allow to pass extra information to the server.
//!
//! Many of these are standardized by either:
//!
//! * http://www.galago-project.org/specs/notification/0.9/x344.html
//! * https://developer.gnome.org/notification-spec/#hints
//!
//! Which of these are actually implemented depends strongly on the Notification server you talk to.
//! Usually the `get_capabilities()` gives some clues, but the standards usually mention much more
//! than is actually available.


use dbus::MessageItem;
use super::NotificationUrgency;
use util::*;

const ACTION_ICONS:&'static str   = "action-icons";
const CATEGORY:&'static str       = "category";
const DESKTOP_ENTRY:&'static str  = "desktop-entry";
//const IMAGE_DATA:&'static str   = "image-data";
const IMAGE_PATH:&'static str     = "image-path";
//const ICON_DATA:&'static str    = "icon_data";
const RESIDENT:&'static str       = "resident";
const SOUND_FILE:&'static str     = "sound-file";
const SOUND_NAME:&'static str     = "sound-name";
const SUPPRESS_SOUND:&'static str = "suppress-sound";
const TRANSIENT:&'static str      = "transient";
const X:&'static str              = "x";
const Y:&'static str              = "y";
const URGENCY:&'static str        = "urgency";

/// All currently implemented NotificationHints that can be send.
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
    //ImageData(iiibiiay),
    //IconData(iiibiiay),

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
            NotificationHint::SoundName(ref inner)    => Some(&inner),
            _ => None
        }

    }
}

impl<'a> From<&'a NotificationHint> for MessageItem {
    fn from(hint: &NotificationHint) -> MessageItem {
        let hint:(String,MessageItem) = match *hint {
            NotificationHint::ActionIcons(value)   => (ACTION_ICONS   .to_owned(), MessageItem::Bool(value)), // bool
            NotificationHint::Category(ref value)      => (CATEGORY       .to_owned(), MessageItem::Str(value.clone())),
            NotificationHint::DesktopEntry(ref value)  => (DESKTOP_ENTRY  .to_owned(), MessageItem::Str(value.clone())),
          //NotificationHint::ImageData(iiibiiay)      => (IMAGE_DATA     .to_owned(), MessageItem::Str(format!("{:?}",  value)),
            NotificationHint::ImagePath(ref value)     => (IMAGE_PATH     .to_owned(), MessageItem::Str(value.clone())),
          //NotificationHint::IconData(iiibiiay)       => (ICON_DATA      .to_owned(), MessageItem::Str(format!("{:?}",  value)),
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

//impl<'a> FromMessageItem<'a> for NotificationHint {
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
            foo => {println!("Invalid {:#?} ", foo); NotificationHint::Invalid}
        }
    }

}

#[cfg(test)]
mod test{
    use super::*;
    use super::NotificationHint as Hint;
    use NotificationUrgency::*;
    use dbus::*;
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
}
