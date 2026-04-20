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
    ManageNotification, NotificationBuilder, NotificationDataSet,
    NotificationDismissedEventHandler, NotificationPriority, ToastsNotifier,
};
use windows::UI::Notifications::ToastNotification as RawToast;

pub use crate::{error::*, notification::Notification, timeout::Timeout, urgency::Urgency};

/// Unique tag counter — prevents `SQLITE_CONSTRAINT_UNIQUE` on rapid successive shows.
static NOTIF_COUNTER: AtomicU64 = AtomicU64::new(0);

// ── CloseReason ──────────────────────────────────────────────────────────────

/// Why a Windows toast notification was dismissed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CloseReason {
    /// The toast timed out.
    Expired,
    /// The user swiped away or clicked X.
    Dismissed,
    /// Hidden programmatically by the app or OS.
    CloseAction,
    /// Unknown reason.
    Other,
}

/// Lets [`NotificationHandle::on_close`] accept `|reason: CloseReason| {}` and `|| {}`.
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

/// Handle to a shown Windows toast. Derefs to [`Notification`] for in-place mutation.
pub struct NotificationHandle {
    notification: Notification,
    /// Kept alive so `close()` and `update()` work.
    notifier: ToastsNotifier,
    /// COM ref to the live toast; needed for `Hide()`.
    raw_toast: RawToast,
    /// Identifies this toast in the Windows notification store.
    tag: String,
    /// Shared with dismissed handlers; cloned on each `update()` rebuild.
    close_tx: mpsc::Sender<CloseReason>,
    /// Yields the dismiss reason once.
    close_rx: mpsc::Receiver<CloseReason>,
    /// Image paths as of the last structural build.
    /// Any change forces a hide + rebuild in `update()` (images aren't data-bindable).
    last_shown_image: Option<String>,
    last_shown_hero_image: Option<String>,
}

impl NotificationHandle {
    /// Hide the toast popup and remove it from the Action Center.
    pub fn close(self) {
        let _ = unsafe { self.notifier.as_raw().Hide(&self.raw_toast) };
        if let Ok(mgr) = self.notifier.manager() {
            let _ = mgr.remove_notification_with_tag(&self.tag);
        }
    }

    /// Block until the notification is dismissed, then call `handler`.
    pub fn on_close<A>(self, handler: impl CloseHandler<A>) {
        let reason = self.close_rx.recv().unwrap_or(CloseReason::Other);
        handler.call(reason);
    }

    /// Replace the visible toast with the current notification state.
    ///
    /// **Fast path:** if only text fields changed, patches the live toast via
    /// [`NotificationDataSet`] with no flicker.
    ///
    /// **Slow path:** if an image changed, or the data-binding update fails,
    /// hides the old toast and shows a fresh one.
    ///
    /// ```no_run
    /// # use notify_rust::Notification;
    /// let mut handle = Notification::new().summary("old").show().unwrap();
    /// handle.body("new body");
    /// handle.update();
    /// ```
    pub fn update(&mut self) {
        let image_changed = self.notification.path_to_image != self.last_shown_image
            || self.notification.hero_image != self.last_shown_hero_image;

        if !image_changed {
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
                    return;
                }
            }
        }

        let _ = unsafe { self.notifier.as_raw().Hide(&self.raw_toast) };

        if let Ok(new_toast) = build_and_show(
            &self.notification,
            &self.notifier,
            &self.tag,
            Some(self.close_tx.clone()),
        ) {
            self.raw_toast = new_toast;
            self.last_shown_image = self.notification.path_to_image.clone();
            self.last_shown_hero_image = self.notification.hero_image.clone();
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

type SoundEntry = (&'static str, fn() -> Src);

/// Maps a `WinRT` sound-event name to a [`Src`] variant.
/// Returns `None` to request a silent audio element instead.
fn map_sound_name(name: &str) -> Option<Src> {
    let lower = name.to_ascii_lowercase();

    if lower.contains("silent") {
        return None;
    }

    // Longest-match first so "alarm10" beats "alarm1".
    const LOOKUP: &[SoundEntry] = &[
        ("looping.alarm10", || Src::Alarm10),
        ("looping.alarm9",  || Src::Alarm9),
        ("looping.alarm8",  || Src::Alarm8),
        ("looping.alarm7",  || Src::Alarm7),
        ("looping.alarm6",  || Src::Alarm6),
        ("looping.alarm5",  || Src::Alarm5),
        ("looping.alarm4",  || Src::Alarm4),
        ("looping.alarm3",  || Src::Alarm3),
        ("looping.alarm2",  || Src::Alarm2),
        ("looping.alarm",   || Src::Alarm),
        ("looping.call10",  || Src::Call10),
        ("looping.call9",   || Src::Call9),
        ("looping.call8",   || Src::Call8),
        ("looping.call7",   || Src::Call7),
        ("looping.call6",   || Src::Call6),
        ("looping.call5",   || Src::Call5),
        ("looping.call4",   || Src::Call4),
        ("looping.call3",   || Src::Call3),
        ("looping.call2",   || Src::Call2),
        ("looping.call",    || Src::Call),
        ("notification.im",       || Src::IM),
        ("notification.mail",     || Src::Mail),
        ("notification.reminder", || Src::Reminder),
        ("notification.sms",      || Src::Sms),
    ];

    Some(
        LOOKUP
            .iter()
            .find(|(key, _)| lower.contains(key))
            .map_or(Src::Default, |(_, make)| make()),
    )
}

/// Returns an error if `path` is a local filesystem path that does not exist.
/// HTTP(S) URLs and `file:///` URIs are passed through without checking.
fn check_local_image(path: &str, field: &str) -> Result<()> {
    if path.starts_with("http://")
        || path.starts_with("https://")
        || path.starts_with("file:///")
    {
        return Ok(());
    }
    if !std::path::Path::new(path).exists() {
        return Err(Error::from(ErrorKind::Msg(format!(
            "{field}: local image file not found: \"{path}\""
        ))));
    }
    Ok(())
}

/// Converts a filesystem path to a `file:///` URI.
///
/// Relative paths are resolved via [`std::fs::canonicalize`]. The `\\?\` prefix
/// that canonicalize adds on Windows is stripped, and characters invalid in URIs
/// (space, `#`, `?`, `%`) are percent-encoded. HTTP(S) and `file:///` inputs pass
/// through unchanged.
fn path_to_file_uri(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") || path.starts_with("file:///") {
        return path.to_owned();
    }

    let absolute = std::fs::canonicalize(path)
        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default().join(path));

    let raw = absolute.to_string_lossy();

    // Strip the extended-length prefix \\?\ (regular) or \\?\UNC\ (network).
    let stripped = raw
        .strip_prefix(r"\\?\UNC\")
        .map_or_else(
            || raw.strip_prefix(r"\\?\").unwrap_or(&raw).to_owned(),
            |s| format!("//{s}"),
        );

    let forward = stripped.replace('\\', "/");

    // `%` first to avoid double-encoding.
    let encoded = forward
        .replace('%', "%25")
        .replace(' ', "%20")
        .replace('#', "%23")
        .replace('?', "%3F");

    if encoded.starts_with('/') {
        format!("file://{encoded}")
    } else {
        format!("file:///{encoded}")
    }
}

/// Builds a toast from `notification`, shows it, and returns a cloned COM reference.
///
/// Text slots 0–2 (`"summary"`, `"subtitle"`, `"body"`) use data-binding keys so
/// [`NotificationHandle::update`] can patch them in-place without a rebuild.
fn build_and_show(
    notification: &Notification,
    notifier: &ToastsNotifier,
    tag: &str,
    dismissed_tx: Option<mpsc::Sender<CloseReason>>,
) -> Result<RawToast> {
    let duration = match notification.timeout {
        Timeout::Default => ToastDuration::Short,
        Timeout::Never => ToastDuration::Long,
        Timeout::Milliseconds(ms) => {
            if ms >= 25_000 { ToastDuration::Long } else { ToastDuration::Short }
        }
    };

    // Critical → Urgent bypasses Focus Assist / DND on Windows 11 22H2+.
    // On older Windows the unknown value degrades to Default (auto-dismisses);
    // pair with Timeout::Never for guaranteed persistence on older systems.
    let scenario = match notification.urgency {
        Some(Urgency::Critical) => Scenario::Urgent,
        _ => Scenario::Default,
    };

    let mut builder = NotificationBuilder::new()
        .with_duration(duration)
        .with_scenario(scenario);

    builder = builder
        .visual(Text::create_binded(0, "summary"))
        .value("summary", &notification.summary);

    let subtitle_val = notification
        .subtitle
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("");
    builder = builder
        .visual(Text::create_binded(1, "subtitle"))
        .value("subtitle", subtitle_val);

    builder = builder
        .visual(Text::create_binded(2, "body"))
        .value("body", &notification.body);

    if let Some(image_path) = &notification.path_to_image {
        check_local_image(image_path, "image_path")?;
        let uri = path_to_file_uri(image_path);
        builder = builder.visual(Image::create(0, &uri).with_placement(Placement::AppLogoOverride));
    }

    if let Some(hero_path) = &notification.hero_image {
        check_local_image(hero_path, "hero_image")?;
        let uri = path_to_file_uri(hero_path);
        builder = builder.visual(Image::create(1, &uri).with_placement(Placement::Hero));
    }

    match &notification.sound_name {
        None => {}
        Some(name) => match map_sound_name(name) {
            None      => { builder = builder.audio(Audio::new(Src::Default, false, true)); }
            Some(src) => { builder = builder.audio(Audio::new(src, false, false)); }
        },
    }

    // The closure must be `Fn`; cloning the sender on each call is a no-op after the first send.
    if let Some(tx) = dismissed_tx {
        builder = builder.on_dismissed(NotificationDismissedEventHandler::new(move |_, reason| {
            use win32_notif::handler::ToastDismissedReason;
            let close_reason = match reason {
                Some(ToastDismissedReason::TimedOut)           => CloseReason::Expired,
                Some(ToastDismissedReason::UserCanceled)       => CloseReason::Dismissed,
                Some(ToastDismissedReason::ApplicationHidden)  => CloseReason::CloseAction,
                _                                              => CloseReason::Other,
            };
            let _ = tx.send(close_reason);
            Ok(())
        }));
    }

    let notif = builder
        .build(0, notifier, tag, tag)
        .map_err(|e| Error::from(ErrorKind::Msg(format!("{e:?}"))))?;

    if notification.urgency == Some(Urgency::Critical) {
        let _ = notif.set_priority(NotificationPriority::High);
    }

    // SAFETY: `as_raw()` is valid for the lifetime of `notif`; `clone()` bumps
    // the COM refcount so the returned value is independently owned.
    let raw_toast = unsafe { notif.as_raw() }.clone();

    notif
        .show()
        .map_err(|e| Error::from(ErrorKind::Msg(format!("{e:?}"))))?;

    Ok(raw_toast)
}

// ── Public entry point ────────────────────────────────────────────────────────

/// Fallback AUMID — always registered, works for unpackaged apps without their own ID.
const FALLBACK_APP_ID: &str = "Microsoft.Windows.Explorer";

pub(crate) fn show_notification(notification: &Notification) -> Result<NotificationHandle> {
    let app_id = notification.app_id.as_deref().unwrap_or(FALLBACK_APP_ID);

    let notifier = ToastsNotifier::new(Some(app_id))
        .map_err(|e| Error::from(ErrorKind::Msg(format!("{e:?}"))))?;

    let tag = NOTIF_COUNTER.fetch_add(1, Ordering::Relaxed).to_string();

    let (close_tx, close_rx) = mpsc::channel();

    let last_shown_image = notification.path_to_image.clone();
    let last_shown_hero_image = notification.hero_image.clone();

    let raw_toast = build_and_show(notification, &notifier, &tag, Some(close_tx.clone()))?;

    Ok(NotificationHandle {
        notification: notification.clone(),
        notifier,
        raw_toast,
        tag,
        close_tx,
        close_rx,
        last_shown_image,
        last_shown_hero_image,
    })
}