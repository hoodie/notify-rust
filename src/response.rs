//! Cross-platform notification response types.
//!
//! These types describe the outcome of a shown notification — whether the
//! user interacted with it or it was closed by the platform.
//! They are shared between all backends so consumer code does not need
//! a `cfg` switch to read responses.

/// Reason a notification was closed without an action being invoked.
///
/// ### Platform notes
///
/// **XDG (Linux/BSD):** maps directly to `NotificationClosed` D-Bus signal reasons.
///
/// **macOS:** the system does not distinguish close reasons, so all closes are
/// reported as [`CloseReason::Dismissed`].
///
/// **Windows:** `UserCanceled` → [`Dismissed`](CloseReason::Dismissed),
/// `TimedOut` → [`Expired`](CloseReason::Expired),
/// `ApplicationHidden` → [`CloseAction`](CloseReason::CloseAction).
// #[non_exhaustive] // TODO: mark in 5.0
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CloseReason {
    /// The notification expired (timed out).
    Expired,

    /// The notification was dismissed by the user.
    Dismissed,

    /// The notification was closed programmatically.
    CloseAction,

    /// An unrecognised or reserved reason was reported by the platform.
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

/// The outcome of a shown notification.
///
/// Returned by `NotificationHandle::response` / `response_blocking` and passed to
/// [`wait_for_response`](crate::NotificationHandle::wait_for_response).
///
/// Match on this to handle every possible outcome:
///
/// ```no_run
/// # use notify_rust::{NotificationResponse, CloseReason};
/// # let response = NotificationResponse::Closed(CloseReason::Dismissed);
/// match response {
///     NotificationResponse::Action(ref key) if key == "default" => println!("body clicked"),
///     NotificationResponse::Action(ref key) => println!("button '{key}' clicked"),
///     NotificationResponse::Reply(ref text) => println!("user replied: {text}"),
///     NotificationResponse::Closed(reason) => println!("closed: {reason:?}"),
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum NotificationResponse {
    /// The user clicked the notification body or a labelled action button.
    ///
    /// The conventional key `"default"` means the notification body was clicked.
    Action(String),

    /// The user submitted an inline text reply.
    ///
    /// Only produced by the `preview-macos-un` backend (macOS `UNUserNotificationCenter`
    /// with an inline reply action). On all other backends this variant is never emitted.
    Reply(String),

    /// The notification was closed without any action being taken.
    Closed(CloseReason),
}

impl NotificationResponse {
    /// Returns `true` if this is an [`Action`](NotificationResponse::Action) with the
    /// key `"default"`, which conventionally means the notification body was clicked.
    pub fn is_default_action(&self) -> bool {
        matches!(self, NotificationResponse::Action(key) if key == "default")
    }
}

impl From<String> for NotificationResponse {
    fn from(key: String) -> Self {
        Self::Action(key)
    }
}

impl From<&str> for NotificationResponse {
    fn from(key: &str) -> Self {
        Self::Action(key.to_owned())
    }
}

/// Response to an action — backward-compatible facade.
///
/// This type is preserved for source compatibility with existing match arms and type signatures.
/// Prefer [`NotificationResponse`] for new code — it owns its data and covers more cases.
///
/// **Deprecated since 4.18.0** — use [`NotificationResponse`] instead.
#[derive(Clone, Debug)]
pub enum ActionResponse<'a> {
    /// The user clicked a named action button (or the notification body, key `"default"`).
    Custom(&'a str),
    /// The notification was closed without any action being taken.
    Closed(CloseReason),
}

impl<'a> From<&'a str> for ActionResponse<'a> {
    fn from(raw: &'a str) -> Self {
        Self::Custom(raw)
    }
}

/// Helper trait implemented by closures used with [`NotificationHandle::wait_for_response`](crate::NotificationHandle::wait_for_response).
///
/// Any `FnOnce(&NotificationResponse)` closure automatically implements this.
pub trait ResponseHandler {
    /// Invoke the handler with the given response.
    fn call(self, response: &NotificationResponse);
}

impl<F> ResponseHandler for F
where
    F: FnOnce(&NotificationResponse),
{
    fn call(self, response: &NotificationResponse) {
        (self)(response);
    }
}

/// Callback for the close signal of a notification.
///
/// Implemented for both `Fn(CloseReason)` and `Fn()`, so there is rarely a
/// good reason to implement this manually.
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
    fn call(&self, _reason: CloseReason) {
        self();
    }
}
