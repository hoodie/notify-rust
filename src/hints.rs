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
    Urgency(NotificationUrgency),
    Custom(String,String),
    Invalid // TODO remove me
}

impl<'a> From<&'a NotificationHint> for MessageItem
{
    fn from(hint: &NotificationHint) -> MessageItem
    {
        let hint:(String,String) = match hint {
            &NotificationHint::ActionIcons(ref value)   => (ACTION_ICONS   .to_owned(), format!("{}",  value)), // bool
            &NotificationHint::Category(ref value)      => (CATEGORY       .to_owned(), value.clone()),
            &NotificationHint::DesktopEntry(ref value)  => (DESKTOP_ENTRY  .to_owned(), value.clone()),
          //&NotificationHint::ImageData(iiibiiay)      => (IMAGE_DATA     .to_owned(), format!("{:?}",  value)),
            &NotificationHint::ImagePath(ref value)     => (IMAGE_PATH     .to_owned(), value.clone()),
          //&NotificationHint::IconData(iiibiiay)       => (ICON_DATA      .to_owned(), format!("{:?}",  value)),
            &NotificationHint::Resident(ref value)      => (RESIDENT       .to_owned(), format!("{}",  value)), // bool
            &NotificationHint::SoundFile(ref value)     => (SOUND_FILE     .to_owned(), value.clone()),
            &NotificationHint::SoundName(ref value)     => (SOUND_NAME     .to_owned(), value.clone()),
            &NotificationHint::SuppressSound(value)     => (SUPPRESS_SOUND .to_owned(), format!("{}",  value)),
            &NotificationHint::Transient(value)         => (TRANSIENT      .to_owned(), format!("{}",  value)),
            &NotificationHint::X(value)                 => (X              .to_owned(), format!("{}",  value)),
            &NotificationHint::Y(value)                 => (Y              .to_owned(), format!("{}",  value)),
            &NotificationHint::Urgency(value)           => (URGENCY        .to_owned(), format!("{}",  value as u32)),
            &NotificationHint::Custom(ref key, ref val) => (key            .to_owned(), val.to_owned ()),
            &NotificationHint::Invalid                  => ("invalid"      .to_owned(), "Invalid".to_owned ())
        };

        MessageItem::DictEntry(
            Box::new(hint.0.into()),
            Box::new(MessageItem::Variant( Box::new(hint.1.into()) ))
            )
    }
}

//impl<'a> FromMessageItem<'a> for NotificationHint {
impl<'a> From<&'a MessageItem> for NotificationHint {

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
                    2 => NotificationUrgency::High,
                    _ => NotificationUrgency::Medium
                }),
            &MessageItem::DictEntry(ref key, ref value) => NotificationHint::Custom(unwrap_message_str(&**key), unwrap_message_str(&**value)),
            foo @ _ => {println!("Invalid {:#?} ", foo); NotificationHint::Invalid}
        }
    }

}

#[cfg(test)]
mod test{
    use super::*;
    use dbus::*;

    #[test]
    fn hint_to_item()
    {
        let old_hint = &NotificationHint::Custom("foo".into(), "bar".into());
        let item:MessageItem = old_hint.into();
        let item_ref = &item;
        let hint:NotificationHint = item_ref.into();
        assert!(old_hint == &hint);
        println!("old_hint: {:?}", old_hint);
        println!("hint: {:?}", hint);
        println!("item: {:?}", item);
    }
}
