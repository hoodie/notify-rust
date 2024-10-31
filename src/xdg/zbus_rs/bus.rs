use std::path::PathBuf;

use crate::xdg::NOTIFICATION_DEFAULT_BUS;

fn skip_first_slash(s: &str) -> &str {
    if let Some('/') = s.chars().next() {
        &s[1..]
    } else {
        s
    }
}

type BusNameType = zbus::names::WellKnownName<'static>;

#[derive(Clone, Debug)]
pub struct NotificationBus(BusNameType);

impl Default for NotificationBus {
    #[cfg(feature = "zbus")]
    fn default() -> Self {
        Self(zbus::names::WellKnownName::from_static_str(NOTIFICATION_DEFAULT_BUS).unwrap())
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
        let name =
            zbus::names::WellKnownName::try_from(Self::namespaced_custom(custom_path)?).ok()?;
        Some(Self(name))
    }

    pub fn into_name(self) -> BusNameType {
        self.0
    }
}
