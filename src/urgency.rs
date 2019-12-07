
/// Levels of Urgency.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Urgency {
    /// The behaviour for `Low` urgency depends on the notification server.
    Low = 0,
    /// The behaviour for `Normal` urgency depends on the notification server.
    Normal = 1,
    /// A critical notification will not time out.
    Critical = 2
}

impl<'a> From<&'a str> for Urgency {
    fn from(string: &'a str) -> Urgency {
        match string.to_lowercase().as_ref() {
            "low"      |
            "lo"       => Urgency::Low,
            "normal"   |
            "medium"   => Urgency::Normal,
            "critical" |
            "high"     |
            "hi"       => Urgency::Critical,
            _ => unimplemented!()
        }
    }
}

impl From<Option<u64>> for Urgency {
    fn from(maybe_int: Option<u64>) -> Urgency {
        match maybe_int {
            Some(0) => Urgency::Low,
            Some(2) => Urgency::Critical,
            _ => Urgency::Normal
        }
    }
}

