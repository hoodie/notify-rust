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
    #[cfg(all(feature = "zbus", not(feature = "dbus")))]
    #[cfg(all(unix, not(target_os = "macos")))]
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

impl From<Timeout> for i32 {
    fn from(timeout: Timeout) -> Self {
        match timeout {
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

#[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
use dbus::arg::messageitem::MessageItem;

#[cfg(all(feature = "dbus", unix, not(target_os = "macos")))]
impl std::convert::TryFrom<&MessageItem> for TimeoutMessage {
    type Error = ();

    fn try_from(mi: &MessageItem) -> Result<TimeoutMessage, ()> {
        mi.inner::<i32>().map(|i| TimeoutMessage(i.into()))
    }
}
