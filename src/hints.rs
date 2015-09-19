use dbus::MessageItem;
use super::NotificationUrgency;

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
    Invalid
}

impl<'a> From<&'a NotificationHint> for MessageItem {
    fn from(hint: &NotificationHint) -> MessageItem
    {
        let hint:(String,String) = match hint {
            &NotificationHint::ActionIcons(ref value)  => ("action-icons".to_owned(),    format!("{}",  value)), // bool
            &NotificationHint::Category(ref value)     => ("category".to_owned(),        value.clone()),
            &NotificationHint::DesktopEntry(ref value) => ("desktop-entry".to_owned(),   value.clone()),
          //&NotificationHint::ImageData(iiibiiay)     => ("image-data".to_owned(),      format!("{:?}",  value)),
            &NotificationHint::ImagePath(ref value)    => ("image-path".to_owned(),      value.clone()),
          //&NotificationHint::IconData(iiibiiay)      => ("icon_data".to_owned(),       format!("{:?}",  value)),
            &NotificationHint::Resident(ref value)     => ("resident".to_owned(),        format!("{}",  value)), // bool
            &NotificationHint::SoundFile(ref value)    => ("sound-file".to_owned(),      value.clone()),
            &NotificationHint::SoundName(ref value)    => ("sound-name".to_owned(),      value.clone()),
            &NotificationHint::SuppressSound(value)    => ("suppress-sound".to_owned(),  format!("{}",  value)),
            &NotificationHint::Transient(value)        => ("transient".to_owned(),       format!("{}",  value)),
            &NotificationHint::X(value)                => ("x".to_owned(),               format!("{}",  value)),
            &NotificationHint::Y(value)                => ("y".to_owned(),               format!("{}",  value)),
            &NotificationHint::Urgency(value)          => ("urgency".to_owned(),         format!("{}",  value as u32)),
            _                                          => ("Foo".to_owned(),"bar".to_owned())
        };

        MessageItem::DictEntry(
            Box::new(hint.0.into()),
            Box::new(MessageItem::Variant( Box::new(hint.1.into()) ))
            )
    }
}

    fn unwrap_message_str(item: &MessageItem) -> String {
        match item{
            &MessageItem::Str(ref value) => value.to_owned(),
            &MessageItem::Variant(ref value) => match item{
                &MessageItem::Str(ref value) => value.to_owned(),
                _ => "".to_owned()
            },
            _ => "".to_owned()
        }
    }

impl<'a> From<&'a MessageItem> for NotificationHint {

    fn from (item: &MessageItem) -> NotificationHint
    {
        match item{
            &MessageItem::DictEntry(box ref key, box ref value) => NotificationHint::Custom(unwrap_message_str(&key), unwrap_message_str(&value)),
            _ => NotificationHint::Invalid
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
        let hint = &NotificationHint::Custom("foo".into(), "bar".into());
        let item:MessageItem = hint.into();
        //let hint : NotificationHint = item.into();
        println!("{:?}", hint);
        println!("{:?}", item);
    }
}
