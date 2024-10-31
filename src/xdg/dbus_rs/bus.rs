use std::path::PathBuf;

use crate::xdg::NOTIFICATION_DEFAULT_BUS;

fn skip_first_slash(s: &str) -> &str {
    if let Some('/') = s.chars().next() {
        &s[1..]
    } else {
        s
    }
}

type BusNameType = dbus::strings::BusName<'static>;

#[derive(Clone, Debug)]
pub struct NotificationBus(BusNameType);

impl Default for NotificationBus {
    fn default() -> Self {
        Self(dbus::strings::BusName::from_slice(NOTIFICATION_DEFAULT_BUS).unwrap())
    }
}

impl NotificationBus {
    fn namespaced_custom(custom_path: &str) -> Option<String> {
        // abusing path for semantic join
        skip_first_slash(
            PathBuf::from("/de/hoodie/Notification")
                .join(custom_path)
                .to_str()?,
        )
        .replace('/', ".")
        .into()
    }

    pub fn custom(custom_path: &str) -> Option<Self> {
        let name = dbus::strings::BusName::new(Self::namespaced_custom(custom_path)?).ok()?;
        Some(Self(name))
    }

    pub fn into_name(self) -> BusNameType {
        self.0
    }
}
