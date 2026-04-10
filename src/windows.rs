#![allow(unsafe_code)]

use std::{
    ops::{Deref, DerefMut},
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
};

use win32_notif::{
    notification::{
        audio::{Audio, Src},
        visual::{Image, Placement, Text},
        Scenario, ToastDuration,
    },
    NotificationBuilder, NotificationDataSet, NotificationDismissedEventHandler, ToastsNotifier,
};
use windows::UI::Notifications::ToastNotification as RawToast;

pub use crate::{error::*, notification::Notification, timeout::Timeout, urgency::Urgency};

/// Monotonically increasing counter used to give every notification a unique tag,
/// preventing `SQLITE_CONSTRAINT_UNIQUE` errors when multiple toasts are shown in
/// rapid succession with the same group.
static NOTIF_COUNTER: AtomicU64 = AtomicU64::new(0);

// ── CloseReason ──────────────────────────────────────────────────────────────

/// Why a Windows toast notification was dismissed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseReason {
    /// The toast expired (timed out automatically).
    Expired,
    /// The user explicitly dismissed the toast (swiped away or clicked X).
    Dismissed,
    /// The application or OS hid the toast programmatically.
    CloseAction,
    /// Unknown or unrecognised reason.
    Other,
}

/// Allows [`NotificationHandle::on_close`] to accept both
/// `|reason: CloseReason| { … }` and `|| { … }` closures.
pub trait CloseHandler<A> {
    /// Invoke the handler with the dismiss reason.
    fn call(self, reason: CloseReason);
}

impl<F: FnOnce(CloseReason)> CloseHandler<CloseReason> for F {
    fn call(self, reason: CloseReason) {
        self(reason);
    }
}

impl<F: FnOnce()> CloseHandler<()> for F {
    fn call(self, _reason: CloseReason) {
        self();
    }
}

/// A handle to a Windows toast notification that has been shown.
///
/// Implements [`Deref`] / [`DerefMut`] targeting the underlying [`Notification`],
/// so you can update fields and call [`update`](NotificationHandle::update) to
/// replace the visible toast in-place.
pub struct NotificationHandle {
    /// The notify-rust `Notification` that was shown; mutated by `update()`.
    notification: Notification,
    /// The WinRT notifier; kept alive so `close()` and `update()` work.
    notifier: ToastsNotifier,
    /// Cloned COM reference to the shown toast; required to call `Hide()`.
    raw_toast: RawToast,
    /// Unique tag used to identify this toast in the Windows notification store.
    tag: String,
    /// Cloned into each dismissed handler registration so `update()` can
    /// re-register the handler without losing the receive end.
    close_tx: mpsc::Sender<CloseReason>,
    /// Receives exactly one value when the toast is dismissed or expires.
    close_rx: mpsc::Receiver<CloseReason>,
    /// Snapshot of `notification.path_to_image` as it was when the toast was
    /// last structurally built.  Used by `update()` to decide whether a cheap
    /// data-binding patch is sufficient or a full rebuild is needed.
    last_shown_image: Option<String>,
}

impl NotificationHandle {
    /// Programmatically dismiss the visible toast popup and remove it from the Action Center.
    pub fn close(self) {
        // Hide the currently visible popup.
        let _ = unsafe { self.notifier.as_raw().Hide(&self.raw_toast) };
        // Also remove any lingering entry from the Action Center history.
        if let Ok(mgr) = self.notifier.manager() {
            let _ = mgr.remove_notification_with_tag(&self.tag);
        }
    }

    /// Block the current thread until the notification is dismissed, then call `handler`.
    ///
    /// Accepts either `|reason: CloseReason| { … }` or `|| { … }`.
    pub fn on_close<A>(self, handler: impl CloseHandler<A>) {
        let reason = self.close_rx.recv().unwrap_or(CloseReason::Other);
        handler.call(reason);
    }

    /// Replace the visible toast with the current state of the notification.
    ///
    /// **Fast path — data-binding update (no flicker):** when only text fields
    /// (`summary`, `subtitle`, `body`) have changed, the live toast is patched
    /// in-place via [`NotificationDataSet`].  The popup is never hidden, so
    /// there is no visual flicker.
    ///
    /// **Slow path — structural rebuild:** if the image has changed since the
    /// last show, or if the data-binding update fails (e.g. the toast was
    /// already dismissed), the old toast is hidden and a fresh one is shown.
    ///
    /// Mutate the handle first (it [`Deref`]s to [`Notification`]), then call
    /// this method:
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// let mut handle = Notification::new().summary("old").show().unwrap();
    /// handle.body("new body");
    /// handle.update();
    /// ```
    pub fn update(&mut self) {
        // ── Detect whether a structural rebuild is required ───────────────────
        // Images are not data-bindable in WinRT toast XML, so any change to
        // path_to_image requires a full hide + rebuild cycle.
        let image_changed = self.notification.path_to_image != self.last_shown_image;

        if !image_changed {
            // ── Fast path: data-binding update ────────────────────────────────
            // Text slots were created with `create_binded` keys "summary",
            // "subtitle", and "body" in `build_and_show`.  We simply push new
            // values for those keys; the Windows notification runtime patches
            // the live toast without removing it from the screen.
            if let Ok(data) = NotificationDataSet::new() {
                let _ = data.insert("summary", &self.notification.summary);

                let subtitle = self
                    .notification
                    .subtitle
                    .as_deref()
                    .filter(|s| !s.is_empty())
                    .unwrap_or("");
                let _ = data.insert("subtitle", subtitle);

                let _ = data.insert("body", &self.notification.body);

                if self.notifier.update(&data, &self.tag, &self.tag).is_ok() {
                    // Patched successfully — nothing else to do.
                    return;
                }
            }
            // Data-binding update failed (toast may have already been dismissed).
            // Fall through to the structural rebuild below.
        }

        // ── Slow path: hide the old toast and show a rebuilt one ──────────────
        let _ = unsafe { self.notifier.as_raw().Hide(&self.raw_toast) };

        if let Ok(new_toast) = build_and_show(
            &self.notification,
            &self.notifier,
            &self.tag,
            Some(self.close_tx.clone()),
        ) {
            self.raw_toast = new_toast;
            self.last_shown_image = self.notification.path_to_image.clone();
        }
    }
}

impl Deref for NotificationHandle {
    type Target = Notification;
    fn deref(&self) -> &Self::Target {
        &self.notification
    }
}

impl DerefMut for NotificationHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.notification
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Map a winrt-style sound-event name (or any freeform string) to a `win32_notif`
/// [`Src`] variant.  Returns `None` to indicate that a *silent* audio element
/// should be used instead.
fn map_sound_name(name: &str) -> Option<Src> {
    let lower = name.to_ascii_lowercase();

    if lower.contains("silent") {
        return None;
    }

    // Ordered longest-suffix-first so e.g. "alarm10" is matched before "alarm1".
    const LOOKUP: &[(&str, fn() -> Src)] = &[
        ("looping.alarm10", || Src::Alarm10),
        ("looping.alarm9", || Src::Alarm9),
        ("looping.alarm8", || Src::Alarm8),
        ("looping.alarm7", || Src::Alarm7),
        ("looping.alarm6", || Src::Alarm6),
        ("looping.alarm5", || Src::Alarm5),
        ("looping.alarm4", || Src::Alarm4),
        ("looping.alarm3", || Src::Alarm3),
        ("looping.alarm2", || Src::Alarm2),
        ("looping.alarm", || Src::Alarm),
        ("looping.call10", || Src::Call10),
        ("looping.call9", || Src::Call9),
        ("looping.call8", || Src::Call8),
        ("looping.call7", || Src::Call7),
        ("looping.call6", || Src::Call6),
        ("looping.call5", || Src::Call5),
        ("looping.call4", || Src::Call4),
        ("looping.call3", || Src::Call3),
        ("looping.call2", || Src::Call2),
        ("looping.call", || Src::Call),
        ("notification.im", || Src::IM),
        ("notification.mail", || Src::Mail),
        ("notification.reminder", || Src::Reminder),
        ("notification.sms", || Src::Sms),
    ];

    Some(
        LOOKUP
            .iter()
            .find(|(key, _)| lower.contains(key))
            .map_or(Src::Default, |(_, make)| make()),
    )
}

/// Convert a filesystem path to the `file:///` URI scheme expected by win32_notif.
/// HTTP/HTTPS URLs and already-formed `file:///` URIs are passed through unchanged.
fn path_to_file_uri(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") || path.starts_with("file:///") {
        return path.to_owned();
    }

    // Normalise Windows back-slashes to forward slashes.
    let forward = path.replace('\\', "/");

    // Absolute paths like "C:/…" carry a drive letter → three slashes.
    // Paths beginning with "/" (UNC-style) only need two.
    if forward.starts_with('/') {
        format!("file://{forward}")
    } else {
        format!("file:///{forward}")
    }
}

/// Populate a [`NotificationBuilder`] from a notify-rust [`Notification`],
/// show the toast, and return a cloned `RawToast` COM reference.
///
/// ## Data binding
///
/// The three text slots are always present and use **data-binding keys**:
///
/// | slot | binding key  | content           |
/// |------|-------------|-------------------|
/// | 0    | `"summary"` | title / summary   |
/// | 1    | `"subtitle"`| subtitle (may be empty) |
/// | 2    | `"body"`    | body text (may be empty) |
///
/// This means [`NotificationHandle::update`] can push new values for those
/// keys via [`NotificationDataSet`] without hiding and reshowing the toast.
/// Windows silently ignores text slots whose bound value is an empty string,
/// so the slots are safe to include unconditionally.
fn build_and_show(
    notification: &Notification,
    notifier: &ToastsNotifier,
    tag: &str,
    dismissed_tx: Option<mpsc::Sender<CloseReason>>,
) -> Result<RawToast> {
    // --- Duration ----------------------------------------------------------------
    let duration = match notification.timeout {
        Timeout::Default => ToastDuration::Short,
        Timeout::Never => ToastDuration::Long,
        Timeout::Milliseconds(ms) => {
            if ms >= 25_000 {
                ToastDuration::Long
            } else {
                ToastDuration::Short
            }
        }
    };

    // --- Scenario (urgency mapping) ----------------------------------------------
    // Critical urgency → Reminder scenario keeps the toast on screen until the
    // user explicitly dismisses it, matching the XDG "critical" intent.
    let scenario = match notification.urgency {
        Some(Urgency::Critical) => Scenario::Reminder,
        _ => Scenario::Default,
    };

    // --- Builder -----------------------------------------------------------------
    let mut builder = NotificationBuilder::new()
        .with_duration(duration)
        .with_scenario(scenario);

    // ── Text slots (data-bound) ───────────────────────────────────────────────
    //
    // `create_binded` requires binding keys that are purely alphabetic.
    // "summary", "subtitle", and "body" all satisfy that constraint.

    // Slot 0 — title / summary.
    builder = builder
        .visual(Text::create_binded(0, "summary"))
        .value("summary", &notification.summary);

    // Slot 1 — subtitle (empty string → not rendered by Windows).
    let subtitle_val = notification
        .subtitle
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("");
    builder = builder
        .visual(Text::create_binded(1, "subtitle"))
        .value("subtitle", subtitle_val);

    // Slot 2 — body (empty string → not rendered by Windows).
    builder = builder
        .visual(Text::create_binded(2, "body"))
        .value("body", &notification.body);

    // ── App-logo / hero image ─────────────────────────────────────────────────
    if let Some(image_path) = &notification.path_to_image {
        let uri = path_to_file_uri(image_path);
        builder = builder.visual(Image::create(0, &uri).with_placement(Placement::AppLogoOverride));
    }

    // ── Sound ─────────────────────────────────────────────────────────────────
    match &notification.sound_name {
        None => {}
        Some(name) => match map_sound_name(name) {
            None => {
                builder = builder.audio(Audio::new(Src::Default, false, true));
            }
            Some(src) => {
                builder = builder.audio(Audio::new(src, false, false));
            }
        },
    }

    // ── Dismissed handler ─────────────────────────────────────────────────────
    // The closure must be `Fn` (not `FnOnce`) per the win32_notif API; cloning
    // the sender each invocation is a no-op after the first send.
    if let Some(tx) = dismissed_tx {
        builder = builder.on_dismissed(NotificationDismissedEventHandler::new(move |_, reason| {
            use win32_notif::handler::ToastDismissedReason;
            let close_reason = match reason {
                Some(ToastDismissedReason::TimedOut) => CloseReason::Expired,
                Some(ToastDismissedReason::UserCanceled) => CloseReason::Dismissed,
                Some(ToastDismissedReason::ApplicationHidden) => CloseReason::CloseAction,
                _ => CloseReason::Other,
            };
            // Ignore send errors: the receiver may have already been dropped
            // (e.g. the caller never called on_close).
            let _ = tx.send(close_reason);
            Ok(())
        }));
    }

    let notif = builder
        .build(0, notifier, tag, tag)
        .map_err(|e| Error::from(ErrorKind::Msg(format!("{e:?}"))))?;

    // Clone the COM reference before show() so we can call Hide() later.
    // SAFETY: as_raw() returns a valid pointer to the inner ToastNotification
    // for the lifetime of `notif`; clone() bumps the COM refcount so the
    // returned value is valid independently.
    let raw_toast = unsafe { notif.as_raw() }.clone();

    notif
        .show()
        .map_err(|e| Error::from(ErrorKind::Msg(format!("{e:?}"))))?;

    Ok(raw_toast)
}

// ── Public entry point ────────────────────────────────────────────────────────

/// "Microsoft.Windows.Explorer" is always registered on Windows and provides a
/// reliable AUMID for unpackaged applications that haven't registered their own.
const FALLBACK_APP_ID: &str = "Microsoft.Windows.Explorer";

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    let app_id = notification.app_id.as_deref().unwrap_or(FALLBACK_APP_ID);

    let notifier = ToastsNotifier::new(Some(app_id))
        .map_err(|e| Error::from(ErrorKind::Msg(format!("{e:?}"))))?;

    // Give every notification a unique tag so that rapid successive calls don't
    // collide in the Windows notification database (SQLITE_CONSTRAINT_UNIQUE).
    let tag = NOTIF_COUNTER.fetch_add(1, Ordering::Relaxed).to_string();

    let (close_tx, close_rx) = mpsc::channel();

    // Snapshot the image path so `update()` can later detect structural changes.
    let last_shown_image = notification.path_to_image.clone();

    let raw_toast = build_and_show(notification, &notifier, &tag, Some(close_tx.clone()))?;

    Ok(NotificationHandle {
        notification: notification.clone(),
        notifier,
        raw_toast,
        tag,
        close_tx,
        close_rx,
        last_shown_image,
    })
}
