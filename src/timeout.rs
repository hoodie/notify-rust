/// Describes the timeout of a notification
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Timeout {
    /// Expires according to server default.
    ///
    /// Whatever that might be...
    Default,

    /// Do not expire, user will have to close this manually.
    Never,

    /// Expire after n milliseconds.
    Milliseconds(u32),
}

impl Timeout {
    pub(crate) fn into_i32(self) -> i32 {
        self.into()
    }
}

impl Default for Timeout {
    fn default() -> Self {
        Timeout::Default
    }
}

#[test]
fn timeout_from_i32() {
    assert_eq!(Timeout::from(234), Timeout::Milliseconds(234));
    assert_eq!(Timeout::from(-234), Timeout::Default);
    assert_eq!(Timeout::from(0), Timeout::Never)
}

impl From<i32> for Timeout {
    fn from(int: i32) -> Timeout {
        use std::cmp::Ordering::*;
        match int.cmp(&0) {
            Greater => Timeout::Milliseconds(int as u32),
            Less => Timeout::Default,
            Equal => Timeout::Never,
        }
    }
}

impl Into<i32> for Timeout {
    fn into(self) -> i32 {
        match self {
            Timeout::Default => -1,
            Timeout::Never => 0,
            Timeout::Milliseconds(ms) => ms as i32,
        }
    }
}

pub struct TimeoutMessage(Timeout);

impl From<Timeout> for TimeoutMessage {
    fn from(hint: Timeout) -> Self {
        TimeoutMessage(hint)
    }
}

impl std::ops::Deref for TimeoutMessage {
    type Target = Timeout;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
use dbus::arg::messageitem::MessageItem;

#[cfg(all(unix, not(target_os = "macos")))]
impl std::convert::TryFrom<&MessageItem> for TimeoutMessage {
    type Error = ();

    fn try_from(mi: &MessageItem) -> Result<TimeoutMessage, ()> {
        mi.inner::<i32>().map(|i| TimeoutMessage(i.into()))
    }
}
