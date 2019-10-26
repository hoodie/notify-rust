
/// Levels of Urgency.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum NotificationUrgency {
    /// The behaviour for `Low` urgency depends on the notification server.
    Low = 0,
    /// The behaviour for `Normal` urgency depends on the notification server.
    Normal = 1,
    /// A critical notification will not time out.
    Critical = 2
}

impl<'a> From<&'a str> for NotificationUrgency {
    fn from(string: &'a str) -> NotificationUrgency {
        match string.to_lowercase().as_ref() {
            "low"      |
            "lo"       => NotificationUrgency::Low,
            "normal"   |
            "medium"   => NotificationUrgency::Normal,
            "critical" |
            "high"     |
            "hi"       => NotificationUrgency::Critical,
            _ => unimplemented!()
        }
    }
}

impl From<Option<u64>> for NotificationUrgency {
    fn from(maybe_int: Option<u64>) -> NotificationUrgency {
        match maybe_int {
            Some(0) => NotificationUrgency::Low,
            Some(2) => NotificationUrgency::Critical,
            _ => NotificationUrgency::Normal
        }
    }
}

