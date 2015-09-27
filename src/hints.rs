use dbus::{MessageItem};
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

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub enum NotificationHint
{ // as found on https://developer.gnome.org/notification-spec/
    ActionIcons(bool),
    Category(String),
    DesktopEntry(String),
    //ImageData(iiibiiay),
    ImagePath(String),
    //IconData(iiibiiay),
    /// This does not work on all servers, however timeout=0 will do the job
    Resident(bool),
    SoundFile(String),
    SoundName(String),
    SuppressSound(bool),
    Transient(bool),
    X(i32),
    Y(i32),
    /// Pass me a NotificationUrgency, either Low, Normal or Critical
    Urgency(NotificationUrgency),
    Custom(String,String),
    Invalid // TODO find a better solution to this
}

impl NotificationHint
{
    pub fn as_bool(&self) -> Option<bool>
    {
        match self
        {
            &NotificationHint::ActionIcons(inner)   => Some(inner),
            &NotificationHint::Resident(inner)      => Some(inner),
            &NotificationHint::SuppressSound(inner) => Some(inner),
            &NotificationHint::Transient(inner)     => Some(inner),
            _ => None
        }
    }
    pub fn as_i32(&self) -> Option<i32>
    {
        match self
        {
            &NotificationHint::X(inner) => Some(inner),
            &NotificationHint::Y(inner) => Some(inner),
            _ => None
        }
    }

    pub fn as_str(&self) -> Option<&str>
    {
        match self
        {
            &NotificationHint::DesktopEntry(ref inner) => Some(&inner),
            &NotificationHint::ImagePath(ref inner) => Some(&inner),
            &NotificationHint::SoundFile(ref inner) => Some(&inner),
            &NotificationHint::SoundName(ref inner) => Some(&inner),
            _ => None
        }

    }
}

impl<'a> From<&'a NotificationHint> for MessageItem
{
    fn from(hint: &NotificationHint) -> MessageItem
    {
        let hint:(String,MessageItem) = match hint {
            &NotificationHint::ActionIcons(value)   => (ACTION_ICONS   .to_owned(), MessageItem::Bool(value)), // bool
            &NotificationHint::Category(ref value)      => (CATEGORY       .to_owned(), MessageItem::Str(value.clone())),
            &NotificationHint::DesktopEntry(ref value)  => (DESKTOP_ENTRY  .to_owned(), MessageItem::Str(value.clone())),
          //&NotificationHint::ImageData(iiibiiay)      => (IMAGE_DATA     .to_owned(), MessageItem::Str(format!("{:?}",  value)),
            &NotificationHint::ImagePath(ref value)     => (IMAGE_PATH     .to_owned(), MessageItem::Str(value.clone())),
          //&NotificationHint::IconData(iiibiiay)       => (ICON_DATA      .to_owned(), MessageItem::Str(format!("{:?}",  value)),
            &NotificationHint::Resident(value)      => (RESIDENT       .to_owned(), MessageItem::Bool(value)), // bool
            &NotificationHint::SoundFile(ref value)     => (SOUND_FILE     .to_owned(), MessageItem::Str(value.clone())),
            &NotificationHint::SoundName(ref value)     => (SOUND_NAME     .to_owned(), MessageItem::Str(value.clone())),
            &NotificationHint::SuppressSound(value)     => (SUPPRESS_SOUND .to_owned(), MessageItem::Bool(value)),
            &NotificationHint::Transient(value)         => (TRANSIENT      .to_owned(), MessageItem::Bool(value)),
            &NotificationHint::X(value)                 => (X              .to_owned(), MessageItem::Int32(value)),
            &NotificationHint::Y(value)                 => (Y              .to_owned(), MessageItem::Int32(value)),
            &NotificationHint::Urgency(value)           => (URGENCY        .to_owned(), MessageItem::Byte(value as u8)),
            &NotificationHint::Custom(ref key, ref val) => (key            .to_owned(), MessageItem::Str(val.to_owned ())),
            &NotificationHint::Invalid                  => ("invalid"      .to_owned(), MessageItem::Str("Invalid".to_owned()))
        };

        MessageItem::DictEntry(
            Box::new(hint.0.into()),
            Box::new(MessageItem::Variant( Box::new(hint.1) ))
            )
    }
}

//impl<'a> FromMessageItem<'a> for NotificationHint {
impl<'a> From<&'a MessageItem> for NotificationHint
{
    fn from (item: &MessageItem) -> NotificationHint
    {
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
            &MessageItem::DictEntry(ref key, ref value) => NotificationHint::Custom(unwrap_message_str(&**key), unwrap_message_str(&**value)),
            foo @ _ => {println!("Invalid {:#?} ", foo); NotificationHint::Invalid}
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
    fn hint_to_item()
    {
        let category = &Hint::Category("testme".to_owned());
        let item:Item= category.into();
        let test_item= Item::DictEntry(
            Box::new(Item::Str("category".into())),
            Box::new(Item::Variant(  Box::new(Item::Str("testme".into()))  ))
            );
        assert_eq!(item, test_item);
    }

    #[test]
    fn urgency()
    {
        let low = &Hint::Urgency(Low);
        let low_item:Item= low.into();
        let test_item= Item::DictEntry(
            Box::new(Item::Str("urgency".into())),
            Box::new(Item::Variant(  Box::new(Item::Byte(0)))  ));
        assert_eq!(low_item, test_item);
    }

    #[test]
    fn simple_hint_to_item()
    {
        let old_hint = &NotificationHint::Custom("foo".into(), "bar".into());
        let item:MessageItem = old_hint.into();
        let item_ref = &item;
        let hint:NotificationHint = item_ref.into();
        assert!(old_hint == &hint);
    }
}
