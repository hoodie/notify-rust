//! Cross-platform action and response types.
//!
//! [`Action`] is the type used with [`Notification::action`](crate::Notification::action)
//! to attach buttons or inline-reply inputs to a notification before it is shown.
//! The response variants ([`UserResponse`], [`ActionResponse`]) describe what the
//! user did after the notification appeared.
//!
//! These types describe how a notification was acted upon by the user:
//! either by clicking a configured action, closing the notification, or
//! submitting an inline text reply.
//!
//! They are shared between all backends so that consumer code does not need
//! a `cfg` switch to read responses.

// ── Action builder ──────────────────────────────────────────────────────────

/// The kind of action stored inside an [`Action`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ActionKind {
    /// A plain button.
    Button,
    /// A text-input reply action.
    ///
    /// Only delivered on macOS (`UNTextInputNotificationAction`). On other
    /// platforms this is treated as a plain button.
    Reply {
        /// Label on the submit button (e.g. `"Send"`).
        button_title: String,
        /// Placeholder hint shown inside the empty input field.
        placeholder: String,
    },
}

/// A notification action — a button or inline-reply input shown alongside the notification.
///
/// Build with [`Action::button`] or [`Action::reply`], then pass to
/// [`Notification::action`](crate::Notification::action).
///
/// ## Platform support
///
/// | Variant | XDG | macOS (UN) | Windows |
/// |---------|:---:|:----------:|:-------:|
/// | `button` | ✔︎ | ✔︎ | ✔︎ |
/// | `reply` | falls back to button | ✔︎ `UNTextInputNotificationAction` | ❌ |
/// | `requires_authentication` | no-op | ✔︎ Touch ID / password | no-op |
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub(crate) id: String,
    pub(crate) label: String,
    pub(crate) kind: ActionKind,
    pub(crate) requires_authentication: bool,
}

impl Action {
    /// Create a plain button action.
    ///
    /// Works on all platforms.
    pub fn button(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: ActionKind::Button,
            requires_authentication: false,
        }
    }

    /// Create a text-input reply action.
    ///
    /// On macOS (UN) this becomes a `UNTextInputNotificationAction`; the user
    /// can type a reply directly inside the notification. On other platforms
    /// it falls back to a plain button.
    ///
    /// `label` is the title shown in the notification's Options menu.
    /// The submit button is labelled `"Reply"` and the text field has no
    /// placeholder by default. Use [`Action::reply_with`] for full control.
    pub fn reply(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: ActionKind::Reply {
                button_title: "Reply".to_owned(),
                placeholder: String::new(),
            },
            requires_authentication: false,
        }
    }

    /// Create a text-input reply action with explicit submit-button title and placeholder text.
    pub fn reply_with(
        id: impl Into<String>,
        label: impl Into<String>,
        button_title: impl Into<String>,
        placeholder: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            kind: ActionKind::Reply {
                button_title: button_title.into(),
                placeholder: placeholder.into(),
            },
            requires_authentication: false,
        }
    }

    /// Require authentication (Touch ID / password) before the action fires.
    ///
    /// Currently enforced on macOS only. No-op on other platforms.
    pub fn requires_authentication(mut self) -> Self {
        self.requires_authentication = true;
        self
    }

    /// The action's identifier, matched against [`UserResponse::Action`].
    pub fn id(&self) -> &str {
        &self.id
    }

    /// The action's display label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns `true` if this is a reply (text-input) action.
    pub fn is_reply(&self) -> bool {
        matches!(self.kind, ActionKind::Reply { .. })
    }
}

// ── Response types ───────────────────────────────────────────────────────────

/// Reason a notification was closed without an action being invoked.
///
/// ### Platform notes
///
/// **XDG (Linux/BSD):** maps directly to the `NotificationClosed` D-Bus signal
/// reasons defined in [Table 8 of the spec](https://specifications.freedesktop.org/notification-spec/latest/protocol.html).
///
/// **macOS:** the underlying system does not distinguish between close reasons,
/// so all closes are reported as [`CloseReason::Dismissed`].
///
/// **Windows (`Windows.UI.Notifications`):** maps from [`ToastDismissalReason`]:
/// `UserCanceled` → [`Dismissed`](CloseReason::Dismissed),
/// `TimedOut` → [`Expired`](CloseReason::Expired),
/// `ApplicationHidden` → [`CloseAction`](CloseReason::CloseAction).
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum CloseReason {
    /// The notification expired (timed out).
    Expired,
    /// The notification was dismissed by the user.
    Dismissed,
    /// The notification was closed programmatically (e.g. a `CloseNotification`
    /// D-Bus call on XDG, or `ToastNotifier::Hide` on Windows).
    CloseAction,
    /// An unrecognized or reserved reason was reported by the platform.
    ///
    /// The raw code (if any) is logged at `DEBUG` level by the backend.
    /// This variant is produced on all platforms to handle forward-compatibility;
    /// the `#[non_exhaustive]` attribute means callers must always include a
    /// wildcard arm when matching.
    Other,
}

impl From<u32> for CloseReason {
    fn from(raw_reason: u32) -> Self {
        match raw_reason {
            1 => CloseReason::Expired,
            2 => CloseReason::Dismissed,
            3 => CloseReason::CloseAction,
            _other => CloseReason::Other,
        }
    }
}

/// The response to a notification — every possible outcome of showing one.
///
/// This is what [`NotificationHandle::response`](crate::NotificationHandle::response)
/// resolves to.
///
/// ### Platform notes
///
/// | Variant | XDG | macOS (UN) | Windows |
/// |---------|-----|------------|---------|
/// | `Action` | ✔︎ via `ActionInvoked` signal | ✔︎ | ✔︎ via `Activated` event |
/// | `Reply` | ❌ not in spec | ✔︎ `UNTextInputNotificationAction` | ✔︎ `ToastTextBox` |
/// | `Closed` | ✔︎ full `CloseReason` | ✔︎ only `Dismissed` | ✔︎ `UserCanceled`/`TimedOut`/`ApplicationHidden` |
///
/// On XDG the `"default"` action key is the conventional identifier for a
/// click on the notification body itself, though servers are not required to
/// use it.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum UserResponse {
    /// The user clicked the notification body or a labelled action button.
    ///
    /// The key is the action identifier registered with
    /// [`Notification::action`](crate::Notification::action).
    /// The conventional key `"default"` means the user clicked the
    /// notification body itself.
    Action(String),

    /// The user submitted an inline text reply.
    ///
    /// Only produced on macOS (`UNTextInputNotificationAction`) and
    /// Windows (`ToastTextBox` input).
    Reply(String),

    /// The notification was closed or dismissed without an action being taken.
    Closed(CloseReason),
}

impl UserResponse {
    /// Returns `true` if this is an [`Action`](UserResponse::Action) with the
    /// key `"default"`, which conventionally means the notification body was
    /// clicked.
    pub fn is_default_action(&self) -> bool {
        matches!(self, UserResponse::Action(key) if key == "default")
    }
}

/// The response from the user after a notification was shown.
///
/// Match on this to handle every possible outcome:
///
/// ```no_run
/// # use notify_rust::{ActionResponse, CloseReason};
/// # let response = ActionResponse::Closed(CloseReason::Dismissed);
/// match response {
///     ActionResponse::Action(key) if key == "default" => println!("body clicked"),
///     ActionResponse::Action(key) => println!("button '{key}' clicked"),
///     ActionResponse::Reply(text) => println!("user replied: {text}"),
///     ActionResponse::Closed(reason) => println!("closed: {reason:?}"),
/// }
/// ```
///
/// ### Platform notes
///
/// | Variant | XDG | macOS (UN) | Windows |
/// |---------|-----|------------|---------|
/// | `Action` | ✔︎ via `ActionInvoked` signal | ✔︎ | ✔︎ via `Activated` event |
/// | `Reply` | ❌ not in spec | ✔︎ `UNTextInputNotificationAction` | ✔︎ `ToastTextBox` input |
/// | `Closed` | ✔︎ full `CloseReason` | ✔︎ only `Dismissed` | ✔︎ `UserCanceled`/`TimedOut`/`ApplicationHidden` |
///
/// On XDG the `"default"` action key is the conventional identifier for a
/// click on the notification body itself, though servers are not required to
/// use it.
///
/// On macOS (legacy `NSUserNotificationCenter` path) `wait_for_action` is not
/// available.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ActionResponse {
    /// The user clicked the notification body or a labelled action button.
    ///
    /// The key is the action identifier that was registered with
    /// [`Notification::action`](crate::Notification::action).
    /// The conventional key `"default"` means the user clicked the
    /// notification body itself.
    Action(String),

    /// The user submitted an inline text reply.
    ///
    /// Only produced on macOS (via `UNTextInputNotificationAction`) and
    /// Windows (via a `ToastTextBox` input element).
    Reply(String),

    /// The notification was closed without any action being taken.
    Closed(CloseReason),
}

impl ActionResponse {
    /// Returns `true` if this is an [`Action`](ActionResponse::Action) with
    /// the key `"default"`, which conventionally means the notification body
    /// was clicked.
    pub fn is_default_action(&self) -> bool {
        matches!(self, ActionResponse::Action(key) if key == "default")
    }
}

impl From<String> for ActionResponse {
    fn from(key: String) -> Self {
        Self::Action(key)
    }
}

impl From<&str> for ActionResponse {
    fn from(key: &str) -> Self {
        Self::Action(key.to_owned())
    }
}

/// Helper trait implemented by closures used with `wait_for_action`.
///
/// You rarely need to implement this manually — any `FnOnce(&ActionResponse)`
/// closure will do.
pub trait ActionResponseHandler {
    /// Invoke the handler with the given response.
    fn call(self, response: &ActionResponse);
}

impl<F> ActionResponseHandler for F
where
    F: FnOnce(&ActionResponse),
{
    fn call(self, response: &ActionResponse) {
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
