/// Levels of Urgency.
///
/// # Specification
/// > Developers must use their own judgement when deciding the urgency of a notification. Typically, if the majority of programs are using the same level for a specific type of urgency, other applications should follow them.
/// >
/// > For low and normal urgencies, server implementations may display the notifications how they choose. They should, however, have a sane expiration timeout dependent on the urgency level.
/// >
/// > **Critical notifications should not automatically expire**, as they are things that the user will most likely want to know about. They should only be closed when the user dismisses them, for example, by clicking on the notification. 
/// 
/// <cite> — see [Galago](http://www.galago-project.org/specs/notification/0.9/x320.html) or [Gnome](https://developer.gnome.org/notification-spec/#urgency-levels) specification.</cite>
/// 
/// # Example
/// ```no_run
/// # use notify_rust::*;
/// Notification::new()
///     .summary("oh no")
///     .icon("dialog-warning")
///     .urgency(Urgency::Critical)
///     .show()?;
/// ```
///
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

