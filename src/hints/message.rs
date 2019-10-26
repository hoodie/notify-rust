//! `NotificationHints` allow you to pass extra information to the server.
//!
//! Many of these are standardized by either:
//!
//! [galago-project spec](http://www.galago-project.org/specs/notification/0.9/x344.html) or
//! [gnome notification-spec](https://developer.gnome.org/notification-spec/#hints)
//!
//! Which of these are actually implemented depends strongly on the Notification server you talk to.
//! Usually the `get_capabilities()` gives some clues, but the standards usually mention much more
//! than is actually available.
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(dead_code, unused_imports)]


use super::{NotificationUrgency, NotificationHint, constants::*};

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use super::image::*;

use std::collections::{HashMap, HashSet};
use dbus::arg::{messageitem::MessageItem, RefArg};

/// All currently implemented `NotificationHints` that can be sent.
///
/// as found on https://developer.gnome.org/notification-spec/
#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub(crate) struct NotificationHintMessage(NotificationHint);

impl NotificationHintMessage {
    pub fn wrap_hint(hint: NotificationHint) -> (MessageItem, MessageItem) {
        Self::from(hint).into()
    }
}

impl From<NotificationHint> for NotificationHintMessage {
    fn from(hint: NotificationHint) -> Self {
        NotificationHintMessage(hint)
    }
}

impl std::ops::Deref for NotificationHintMessage {
    type Target = NotificationHint;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, A: RefArg> From<(&'a String, &'a A)> for NotificationHintMessage {
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
                eprintln!("Invalid NotificationHint{:#?} ", other);
                NotificationHint::Invalid
            }
        }.into()
    }
}

#[deprecated(note = "Prefer the DBus Arg and RefArg APIs")]
impl From<NotificationHintMessage> for (MessageItem, MessageItem) {
    fn from(hint: NotificationHintMessage) -> Self {

        let (key, value): (String, MessageItem) = match hint.0 {
            NotificationHint::ActionIcons(value)       => (ACTION_ICONS   .to_owned(), MessageItem::Bool(value)), // bool
            NotificationHint::Category(ref value)      => (CATEGORY       .to_owned(), MessageItem::Str(value.clone())),
            NotificationHint::DesktopEntry(ref value)  => (DESKTOP_ENTRY  .to_owned(), MessageItem::Str(value.clone())),
            #[cfg(all(feature = "images", unix, not(target_os ="macos")))]
            NotificationHint::ImageData(ref image)     => (image_spec(*crate::SPEC_VERSION), image.clone().into()),
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

        (MessageItem::Str(key), MessageItem::Variant(Box::new(value)))
    }
}


// TODO: deprecated, Prefer the DBus Arg and RefArg APIs
impl From<(&MessageItem, &MessageItem)> for NotificationHintMessage {
    fn from ((key, mut value): (&MessageItem, &MessageItem)) -> Self {
        use NotificationHint as Hint;

        // If this is a variant, consider the thing inside it
        // If it's a nested variant, keep drilling down until we get a real value
        while let MessageItem::Variant(inner) = value {
            value = &*inner;
        }

        let is_stringy = value.inner::<&str>().is_ok();

        match key.inner::<&str>() {
            Ok(CATEGORY)        => value.inner::<&str>().map(String::from).map(Hint::Category),
            Ok(ACTION_ICONS)    => value.inner().map(Hint::ActionIcons),
            Ok(DESKTOP_ENTRY)   => value.inner::<&str>().map(String::from).map(Hint::DesktopEntry),
            Ok(IMAGE_PATH)      => value.inner::<&str>().map(String::from).map(Hint::ImagePath),
            Ok(RESIDENT)        => value.inner().map(Hint::Resident),
            Ok(SOUND_FILE)      => value.inner::<&str>().map(String::from).map(Hint::SoundFile),
            Ok(SOUND_NAME)      => value.inner::<&str>().map(String::from).map(Hint::SoundName),
            Ok(SUPPRESS_SOUND)  => value.inner().map(Hint::SuppressSound),
            Ok(TRANSIENT)       => value.inner().map(Hint::Transient),
            Ok(X)               => value.inner().map(Hint::X),
            Ok(Y)               => value.inner().map(Hint::Y),
            Ok(URGENCY)         => value.inner().map(|i| match i {
                0  => NotificationUrgency::Low,
                2  => NotificationUrgency::Critical,
                _  => NotificationUrgency::Normal
            }).map(Hint::Urgency),
            Ok(k) if is_stringy => value.inner::<&str>().map(|v| Hint::Custom(k.to_string(), v.to_string())),
            Ok(k)               => value.inner().map(|v| Hint::CustomInt(k.to_string(), v)),
            _ => Err(()),
        }.unwrap_or(Hint::Invalid)
        .into()
    }
}


#[allow(missing_docs)]
pub(crate) fn hints_from_variants<A: RefArg>(hints: &HashMap<String, A>) -> HashSet<NotificationHintMessage> {
    hints.iter().map(Into::into).collect()
}
