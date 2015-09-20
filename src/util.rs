use std::borrow::Cow;
use dbus::{Message,MessageItem};

//TODO get rid of the util functions

pub fn unwrap_message_int(item: &MessageItem) -> i32{
    unwrap_message_str(item).parse::<i32>().unwrap_or(0)
}

pub fn unwrap_message_bool(item: &MessageItem) -> bool{
    unwrap_message_str(item).parse::<bool>().unwrap_or(false)
}

pub fn unwrap_message_str(item: &MessageItem) -> String {
    match item{
        &MessageItem::Str(ref value) => value.to_owned(),
        &MessageItem::Variant(ref value) =>
            match **value{
                MessageItem::Str(ref value) => value.to_owned(),
                _ => "".to_owned()
            },
            _ => "".to_owned()
    }
}

pub fn unwrap_message_string(item: Option<&MessageItem>) -> String {
    match item{
        Some(&MessageItem::Str(ref value)) => value.clone(),
        Some(&MessageItem::Array(ref items, Cow::Borrowed("{sv}"))) => format!("DICT   {:?}", items),
        Some(&MessageItem::Array(ref items, Cow::Borrowed("s"))) => format!("ARRAY  {:?}", items),
        Some(&MessageItem::Array(ref items, ref sig )) => format!("{sig:?} {items:?}", items=items, sig=sig),
        _ => "".to_owned()
    }
}

