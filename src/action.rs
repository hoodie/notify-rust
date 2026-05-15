//! Cross-platform action response types.
//!
//! These types describe how a notification was acted upon by the user:
//! either by clicking a configured action, or by closing the notification.
//! They are shared between the XDG (Linux/BSD) and macOS backends so that
//! consumer code does not need a `cfg` switch to read responses.

/// Reason a notification was closed.
///
/// On XDG (Linux/BSD) this maps to the `NotificationClosed` D-Bus signal as
/// listed under [Table 8. `NotificationClosed` Parameters](https://specifications.freedesktop.org/notification-spec/latest/ar01s09.html#idm46350804042704).
///
/// On macOS the underlying notification system does not distinguish between
/// close reasons, so all closes are reported as [`CloseReason::Dismissed`].
#[derive(Copy, Clone, Debug)]
pub enum CloseReason {
    /// The notification expired.
    Expired,
    /// The notification was dismissed by the user.
    Dismissed,
    /// The notification was closed by a call to `CloseNotification`.
    CloseAction,
    /// Undefined or reserved reason.
    Other(u32),
}

impl From<u32> for CloseReason {
    fn from(raw_reason: u32) -> Self {
        match raw_reason {
            1 => CloseReason::Expired,
            2 => CloseReason::Dismissed,
            3 => CloseReason::CloseAction,
            other => CloseReason::Other(other),
        }
    }
}

/// Response to a user action on a notification.
#[derive(Clone, Debug)]
pub enum ActionResponse<'a> {
    /// Custom action configured by the notification.
    Custom(&'a str),
    /// The notification was closed.
    Closed(CloseReason),
}

impl<'a> From<&'a str> for ActionResponse<'a> {
    fn from(raw: &'a str) -> Self {
        Self::Custom(raw)
    }
}

/// Helper trait implemented by closures used with `wait_for_action`.
pub trait ActionResponseHandler {
    /// Invoke the handler with the given response.
    fn call(self, response: &ActionResponse);
}

impl<F> ActionResponseHandler for F
where
    F: FnOnce(&ActionResponse),
{
    fn call(self, res: &ActionResponse) {
        (self)(res);
    }
}

/// Callback for the `Close` signal of a notification.
///
/// This is implemented by `Fn()` and `Fn(CloseReason)`, so there is rarely a
/// good reason to implement this trait manually.
pub trait CloseHandler<T> {
    /// Called with the [`CloseReason`].
    fn call(&self, reason: CloseReason);
}

impl<F> CloseHandler<CloseReason> for F
where
    F: Fn(CloseReason),
{
    fn call(&self, reason: CloseReason) {
        self(reason);
    }
}

impl<F> CloseHandler<()> for F
where
    F: Fn(),
{
    fn call(&self, _: CloseReason) {
        self();
    }
}
