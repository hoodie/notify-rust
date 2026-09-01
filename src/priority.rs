use std::fmt;

use zbus::zvariant::Type;

use crate::Urgency;

/// Priority of a notification sent via the desktop portal.
///
/// Maps directly to the `priority` field of the
/// `org.freedesktop.portal.Notification` `AddNotification` call.
#[derive(Eq, Hash, Copy, Clone, Debug, Type, PartialEq)]
#[zvariant(signature = "s")]
pub enum Priority {
    /// Background, unimportant — the notification may be shown silently or not at all.
    Low,
    /// Ordinary notification — the default priority level.
    Normal,
    /// Needs attention soon — elevated but not critical.
    High,
    /// Requires immediate action — the highest urgency level.
    Urgent,
}

impl From<Urgency> for Priority {
    fn from(urgency: Urgency) -> Priority {
        match urgency {
            Urgency::Low => Priority::Low,
            Urgency::Normal => Priority::Normal,
            Urgency::Critical => Priority::Urgent,
        }
    }
}

impl From<&Priority> for &'static str {
    fn from(priority: &Priority) -> &'static str {
        match priority {
            Priority::Low => "low",
            Priority::Normal => "normal",
            Priority::High => "high",
            Priority::Urgent => "urgent",
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(<&'static str>::from(self))
    }
}

impl serde::Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(<&'static str>::from(self))
    }
}
