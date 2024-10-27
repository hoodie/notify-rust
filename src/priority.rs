use zbus::zvariant::Type;

use crate::Urgency;

/// Used in desktop portals.
#[derive(Eq, Hash, Copy, Clone, Debug, Type, PartialEq)]
#[zvariant(signature = "s")]
pub enum Priority {
    Low,
    Normal,
    High,
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
impl Into<&str> for &Priority {
    fn into(self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Normal => "normal",
            Priority::High => "high",
            Priority::Urgent => "urgent",
        }
    }
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        let prio: &str = self.into();
        prio.to_string()
    }
}

impl serde::Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde::Serialize::serialize(&self.to_string(), serializer)
    }
}
