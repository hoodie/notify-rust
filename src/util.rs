use dbus::MessageItem;

// TODO get rid of the util functions

pub fn unwrap_message_int(item: &MessageItem) -> i32 {
    try_unwrap_message_int(item).unwrap_or(0)
}

pub fn try_unwrap_message_int(item: &MessageItem) -> Option<i32> {
    unwrap_message_str(item).parse::<i32>().ok()
}

pub fn unwrap_message_bool(item: &MessageItem) -> bool {
    unwrap_message_str(item).parse::<bool>().unwrap_or(false)
}

pub fn unwrap_message_str(item: &MessageItem) -> String {
    match *item {
        MessageItem::Str(ref value) => value.to_owned(),
        MessageItem::Variant(ref value) => match **value {
            MessageItem::Str(ref value) => value.to_owned(),
            _ => "".to_owned()
        },
        _ => "".to_owned()
    }
}
