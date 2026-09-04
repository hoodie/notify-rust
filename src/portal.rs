//! Standalone portal notification builder.
//!
//! This module provides a [`Notification`] type that communicates exclusively through the
//! [XDG desktop portal](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Notification.html)
//! (`org.freedesktop.portal.Notification`).  It is the right choice for sandboxed
//! environments (Flatpak, Snap) where direct D-Bus access to
//! `org.freedesktop.Notifications` is not available.
//!
//! # Quick start
//!
//! ```no_run
//! # #[cfg(all(unix, not(target_os = "macos"), feature = "async", feature = "zbus"))]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use notify_rust::portal::{Notification, Priority};
//!
//! let handle = Notification::new("Download complete")
//!     .body("Your file is ready.")
//!     .priority(Priority::Normal)
//!     .icon_named("document-save")
//!     .show()
//!     .await?;
//!
//! // replace the notification in-place, or dismiss it
//! // handle.update();
//! handle.close();
//! # Ok(())
//! # }
//! ```
//!
//! # Stable IDs
//!
//! If you need the notification to survive across process restarts (e.g. a persistent
//! download-progress notification), supply an explicit ID with [`.id()`](Notification::id):
//!
//! ```no_run
//! # #[cfg(all(unix, not(target_os = "macos"), feature = "async", feature = "zbus"))]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use notify_rust::portal::Notification;
//!
//! let handle = Notification::new("Downloading…")
//!     .id("com.example.myapp.download")
//!     .body("50% complete")
//!     .show()
//!     .await?;
//!
//! // Later — re-show with the same ID to replace in-place:
//! Notification::new("Downloading…")
//!     .id("com.example.myapp.download")
//!     .body("100% complete")
//!     .show()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Conversion from a classic [`crate::Notification`]
//!
//! If you already have a classic notification, call [`.portal()`](crate::Notification::portal)
//! on it to obtain a `portal::Notification` pre-populated with a best-effort translation of
//! the classic fields:
//!
//! ```no_run
//! # #[cfg(all(unix, not(target_os = "macos"), feature = "async", feature = "zbus"))]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! use notify_rust::Notification as ClassicNotification;
//!
//! ClassicNotification::new()
//!     .summary("hello")
//!     .body("world")
//!     .portal()   // consumes the classic notification, returns portal::Notification
//!     .show()
//!     .await?;
//! # Ok(())
//! # }
//! ```

// This module is only meaningful on Linux/BSD with the zbus feature enabled.
// All public items are still declared at the module level for rustdoc and IDE
// tooling, but gated so they only compile when the portal is available.

#[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
use crate::{error::Result, xdg::zbus_rs::portal as portal_impl};

// Re-export the stable public portal types so users can write
// `use notify_rust::portal::{Button, Icon, Priority}`.
#[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
pub use crate::priority::Priority;
#[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
pub use crate::xdg::PortalNotificationHandle as NotificationHandle;
#[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
pub use crate::xdg::{PortalButton as Button, PortalIcon as Icon};

// ---------------------------------------------------------------------------
// portal::Notification
// ---------------------------------------------------------------------------

/// A notification builder for the XDG desktop portal
/// (`org.freedesktop.portal.Notification`).
///
/// This is a standalone type — distinct from [`crate::Notification`] — that only exposes
/// the concepts the portal spec defines.  It is the counterpart to the classic
/// `Notification` for code that exclusively targets the portal path.
///
/// The builder is obtained in two ways:
///
/// 1. **Directly**: [`Notification::new(title)`](Notification::new)
/// 2. **Via conversion**: [`classic.portal()`](crate::Notification::portal) — converts a
///    classic [`crate::Notification`] using the
///    [concept map](crate::portal#conversion-from-a-classic-notification).
///
/// # Platform support
///
/// Requires Linux or BSD with the `zbus` feature enabled.  The type is only available in
/// those configurations.
#[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
#[derive(Debug)]
pub struct Notification {
    /// Notification title (required).
    pub(crate) title: String,

    /// Optional body text.
    pub(crate) body: Option<String>,

    /// Optional explicit notification ID.
    ///
    /// If `None`, a process-unique monotonic ID is generated on [`show()`](Self::show).
    pub(crate) id: Option<String>,

    /// Notification priority.
    pub(crate) priority: Option<Priority>,

    /// Optional icon.
    pub(crate) icon: Option<Icon>,

    /// Action buttons.
    pub(crate) buttons: Vec<Button>,

    /// The default action key, invoked when the notification body is clicked.
    pub(crate) default_action: Option<String>,
}

#[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
impl Notification {
    // -----------------------------------------------------------------------
    // Constructor
    // -----------------------------------------------------------------------

    /// Create a new portal notification with the given title.
    ///
    /// The title is the only required field; all other fields are optional and
    /// can be set with the builder methods below.
    ///
    /// # Example
    /// ```no_run
    /// # use notify_rust::portal::Notification;
    /// let n = Notification::new("Hello from the portal");
    /// ```
    pub fn new(title: impl Into<String>) -> Self {
        Notification {
            title: title.into(),
            body: None,
            id: None,
            priority: None,
            icon: None,
            buttons: Vec::new(),
            default_action: None,
        }
    }

    // -----------------------------------------------------------------------
    // Builder methods
    // -----------------------------------------------------------------------

    /// Set the notification body text.
    ///
    /// The body is shown below the title and may contain plain text.
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Set an explicit notification ID.
    ///
    /// The portal uses caller-supplied string IDs.  If the same ID is used in a
    /// subsequent [`show()`](Self::show) call the portal replaces the existing
    /// notification in-place.  This makes it easy to build "progress" notifications:
    ///
    /// ```no_run
    /// # use notify_rust::portal::Notification;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// Notification::new("Uploading…")
    ///     .id("myapp.upload")
    ///     .body("25%")
    ///     .show().await?;
    ///
    /// Notification::new("Uploading…")
    ///     .id("myapp.upload")
    ///     .body("100% — done!")
    ///     .show().await?;
    /// # Ok(()) }
    /// ```
    ///
    /// If no ID is set, a unique one is generated automatically.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the notification priority.
    ///
    /// Defaults to [`Priority::Normal`] when not set.
    ///
    /// | Value                    | Meaning                     |
    /// |--------------------------|-----------------------------|
    /// | [`Priority::Low`]        | Background, unimportant     |
    /// | [`Priority::Normal`]     | Ordinary notification       |
    /// | [`Priority::High`]       | Needs attention soon        |
    /// | [`Priority::Urgent`]     | Requires immediate action   |
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }

    /// Set a themed icon by name.
    ///
    /// Themed icons are looked up in the current icon theme and bypass portal icon
    /// validation entirely — they are always safe to use.
    ///
    /// ```no_run
    /// # use notify_rust::portal::Notification;
    /// Notification::new("Alert").icon_named("dialog-warning");
    /// ```
    pub fn icon_named(mut self, name: impl Into<String>) -> Self {
        self.icon = Some(Icon::themed(vec![name.into()]));
        self
    }

    /// Set the icon from a file on disk.
    ///
    /// The file is copied into a sealed `memfd` and passed to the portal as a file
    /// descriptor.  If the file cannot be opened or does not satisfy the portal's
    /// validation constraints, the icon is silently dropped.
    ///
    /// # Portal icon validation constraints
    ///
    /// The portal validates every `file-descriptor` icon before forwarding it to the
    /// notification backend.  Icons that fail validation are dropped silently — the
    /// notification appears without an icon and no error is returned.
    ///
    /// | Constraint       | Limit                       |
    /// |------------------|-----------------------------|
    /// | Formats          | `png`, `jpeg`, `svg`        |
    /// | Must be square   | `width == height`           |
    /// | Max raster size  | 512 × 512 px                |
    /// | Max SVG size     | 4096 × 4096 px              |
    /// | Max file size    | 4 MB                        |
    ///
    /// Prefer [`icon_named`](Self::icon_named) when a suitable icon theme entry exists.
    pub fn icon_path(mut self, path: impl AsRef<str>) -> Self {
        self.icon = Icon::open(path.as_ref());
        self
    }

    /// Set the icon directly.
    ///
    /// Accepts any [`Icon`] value, allowing both themed and file-descriptor icons.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Add an action button.
    ///
    /// Buttons are shown alongside the notification.  Each button must have a unique
    /// action key and a human-readable label.
    ///
    /// # Example
    /// ```no_run
    /// # use notify_rust::portal::{Notification, Button};
    /// Notification::new("Download complete")
    ///     .button(Button::new("open", "Open"))
    ///     .button(Button::new("dismiss", "Dismiss"));
    /// ```
    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }

    /// Set the default action key.
    ///
    /// The default action is invoked when the user clicks the notification body
    /// (outside of any specific button).  This corresponds to the portal
    /// `default-action` vardict field.
    ///
    /// If your actions vec contains an entry with key `"default"`, use this method to
    /// promote it explicitly.  When converting from a classic [`crate::Notification`]
    /// via [`.portal()`](crate::Notification::portal) the `"default"` action is
    /// promoted automatically.
    pub fn default_action(mut self, action: impl Into<String>) -> Self {
        self.default_action = Some(action.into());
        self
    }

    // -----------------------------------------------------------------------
    // show
    // -----------------------------------------------------------------------

    /// Send this notification via the XDG desktop portal.
    ///
    /// If no explicit ID was set via [`.id()`](Self::id), a process-unique monotonic ID
    /// is generated automatically.  The ID is stored in the returned
    /// [`NotificationHandle`] and used by [`NotificationHandle::update`] (replace
    /// in-place) and [`NotificationHandle::close`] (dismiss).
    ///
    /// # Errors
    ///
    /// Returns an error if the D-Bus session connection fails or the portal rejects the
    /// call (e.g. because the `org.freedesktop.portal.Notification` interface is not
    /// available on this system).
    ///
    /// # GNOME requirement: app ID and `.desktop` file
    ///
    /// The GNOME portal backend (`xdg-desktop-portal-gnome`) forwards notifications
    /// through GNOME Shell's `org.gtk.Notifications` API. Before displaying anything,
    /// GNOME Shell performs two checks (source: `GtkNotificationDaemonAppSource`):
    ///
    /// 1. The app ID must be a valid `GLib` application ID — reverse-DNS form with at
    ///    least two dot-separated alphanumeric components (e.g. `"org.example.MyApp"`).
    ///
    /// 2. A `.desktop` file named `<app-id>.desktop` must exist somewhere GIO can find
    ///    it: `$XDG_DATA_HOME/applications` (`~/.local/share/applications/`) **or** any
    ///    directory in `$XDG_DATA_DIRS`. `~/.local/share/applications/` is sufficient —
    ///    no system path is required.
    ///
    /// If either check fails the notification is silently dropped from the caller's
    /// perspective, and the portal daemon logs:
    ///
    /// ```text
    /// Error from gnome-shell: GDBus.Error:org.gtk.Notifications.Error.InvalidApp:
    ///     The app by ID "" could not be found
    /// ```
    ///
    /// The app ID is derived by `xdg-desktop-portal` from the **systemd user unit
    /// name** of the D-Bus sender's process (read from its cgroup). Processes launched
    /// directly from a terminal have no matching user service unit, so the app ID falls
    /// back to `""`, which immediately fails check 1.
    ///
    /// ## Making it work on GNOME
    ///
    /// You need exactly two things:
    ///
    /// **Step 1** — install a `.desktop` file. `~/.local/share/applications/` is fine:
    ///
    /// ```text
    /// # ~/.local/share/applications/org.example.myapp.desktop
    /// [Desktop Entry]
    /// Name=My App
    /// Type=Application
    /// Exec=/path/to/my-binary
    /// NoDisplay=true
    /// ```
    ///
    /// Then refresh the cache: `update-desktop-database ~/.local/share/applications/`
    ///
    /// The stem of the filename (`org.example.myapp`) is the app ID. Prefer IDs with
    /// no hyphens; hyphens are allowed but get systemd-escaped in the unit name (see
    /// step 2).
    ///
    /// **Step 2** — launch the binary as a systemd user service with a matching name.
    /// `xdg-desktop-portal` extracts the app ID from the unit name using:
    ///
    /// ```text
    /// app[-<launcher>]-<ApplicationID>[@<instance>].service
    /// ```
    ///
    /// So for app ID `org.example.myapp`:
    ///
    /// ```text
    /// systemd-run --user --service-type=oneshot \
    ///     --unit=app-org.example.myapp.service \
    ///     /path/to/my-binary
    /// ```
    ///
    /// **Note on hyphens**: systemd hex-escapes hyphens in unit names
    /// (`org.notify-rust` → `app-org.notify\x2drust.service`). Use pure dot-separated
    /// IDs to avoid the escaping.
    ///
    /// **Note on working directory**: `systemd-run` defaults to `$HOME`, not the
    /// current directory. Pass `--working-directory=…` if your binary uses relative
    /// paths.
    ///
    /// This restriction is specific to the GNOME backend. Other portal backends (KDE,
    /// mako, dunst, …) do not enforce an app-ID lookup and show notifications
    /// unconditionally, even when launched from a terminal.
    #[cfg(all(feature = "async", feature = "zbus"))]
    pub async fn show(self) -> Result<NotificationHandle> {
        portal_impl::connect_and_send_portal_notification(self).await
    }
}

// ---------------------------------------------------------------------------
// Conversion from classic Notification
// ---------------------------------------------------------------------------

/// Extension method — adds `.portal()` to the classic [`crate::Notification`] builder.
///
/// Call this on a classic notification to obtain a [`portal::Notification`] pre-populated
/// with a best-effort translation of the classic fields.  Methods that have no portal
/// equivalent (`.hint()`, `.timeout()`, etc.) are silently dropped at the conversion
/// boundary.
///
/// The returned [`Notification`] supports all portal-specific builder methods such as
/// [`.button()`](Notification::button) and [`.priority()`](Notification::priority).
///
/// # Example
///
/// ```no_run
/// # #[cfg(all(unix, not(target_os = "macos"), feature = "async", feature = "zbus"))]
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use notify_rust::Notification as Classic;
///
/// Classic::new()
///     .summary("hello")
///     .body("world")
///     .icon("dialog-information")
///     .portal()
///     .show()
///     .await?;
/// # Ok(())
/// # }
/// ```
#[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
pub trait IntoPortalNotification {
    /// Convert `self` into a [`portal::Notification`](Notification), translating all
    /// supported fields and silently dropping any that have no portal equivalent.
    fn portal(self) -> Notification;
}

#[cfg(all(unix, not(target_os = "macos"), feature = "zbus"))]
impl IntoPortalNotification for crate::notification::Notification {
    fn portal(self) -> Notification {
        use crate::Hint;

        // --- priority ---
        let priority = self.hints.iter().find_map(|h| {
            if let Hint::Urgency(u) = h {
                Some(Priority::from(*u))
            } else {
                None
            }
        });

        // --- icon ---
        let icon = {
            let path_from_hint = self.get_hints().find_map(|h| match h {
                Hint::ImagePath(ref p) => Some(p.clone()),
                _ => None,
            });

            if let Some(path) = path_from_hint {
                log::debug!("portal: icon from image path '{}'", path);
                Icon::open(&path)
            } else if let Some(name) = self.icon.as_deref() {
                log::debug!("portal: icon themed '{}'", name);
                Some(Icon::themed(vec![name.to_owned()]))
            } else {
                None
            }
        };

        // --- actions → default-action + buttons ---
        //
        // Classic notifications store actions as a flat alternating vec:
        //   ["action-id-1", "Label 1", "action-id-2", "Label 2", ...]
        //
        // The portal separates the "default" action from ordinary buttons.
        let (default_action, buttons) = {
            let pairs: Vec<(&str, &str)> = self
                .actions
                .as_chunks::<2>().0.iter()
                .map(|c| (c[0].as_str(), c[1].as_str()))
                .collect();

            if pairs.is_empty() {
                (None, Vec::new())
            } else {
                let mut default: Option<String> = None;
                let mut btns: Vec<Button> = Vec::new();

                for (id, label) in pairs {
                    if id == "default" {
                        // Store the action *key*, not the human-readable label.
                        default = Some(id.to_owned());
                    } else {
                        btns.push(Button::new(id, label));
                    }
                }

                (default, btns)
            }
        };

        Notification {
            title: self.summary.clone(),
            body: if self.body.is_empty() {
                None
            } else {
                Some(self.body.clone())
            },
            id: None,
            priority,
            icon,
            buttons,
            default_action,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(all(test, unix, not(target_os = "macos"), feature = "zbus"))]
mod tests {
    use super::*;
    use crate::{urgency::Urgency, Hint};

    // Helper: build a classic notification and convert it.
    fn classic_to_portal(
        setup: impl FnOnce(&mut crate::notification::Notification),
    ) -> Notification {
        let mut n = crate::notification::Notification::new();
        setup(&mut n);
        n.portal()
    }

    // -----------------------------------------------------------------------
    // Basic field mapping
    // -----------------------------------------------------------------------

    #[test]
    fn title_maps_from_summary() {
        let p = classic_to_portal(|n| {
            n.summary("hello");
        });
        assert_eq!(p.title, "hello");
    }

    #[test]
    fn body_maps_correctly() {
        let p = classic_to_portal(|n| {
            n.summary("t");
            n.body("world");
        });
        assert_eq!(p.body.as_deref(), Some("world"));
    }

    #[test]
    fn empty_body_becomes_none() {
        let p = classic_to_portal(|n| {
            n.summary("t");
        });
        assert!(p.body.is_none());
    }

    // -----------------------------------------------------------------------
    // Priority / Urgency conversion
    // -----------------------------------------------------------------------

    #[test]
    fn urgency_low_maps_to_priority_low() {
        let p = classic_to_portal(|n| {
            n.hint(Hint::Urgency(Urgency::Low));
        });
        assert!(matches!(p.priority, Some(Priority::Low)));
    }

    #[test]
    fn urgency_normal_maps_to_priority_normal() {
        let p = classic_to_portal(|n| {
            n.hint(Hint::Urgency(Urgency::Normal));
        });
        assert!(matches!(p.priority, Some(Priority::Normal)));
    }

    #[test]
    fn urgency_critical_maps_to_priority_urgent_not_high() {
        let p = classic_to_portal(|n| {
            n.hint(Hint::Urgency(Urgency::Critical));
        });
        // Critical → Urgent (not High — High has no classic equivalent)
        assert!(matches!(p.priority, Some(Priority::Urgent)));
    }

    #[test]
    fn no_urgency_hint_leaves_priority_none() {
        let p = classic_to_portal(|n| {
            n.summary("t");
        });
        // No urgency hint → None (caller decides, show() defaults to Normal)
        assert!(p.priority.is_none());
    }

    // -----------------------------------------------------------------------
    // Icon conversion
    // -----------------------------------------------------------------------

    #[test]
    fn themed_icon_from_icon_named() {
        let p = classic_to_portal(|n| {
            n.icon_named("dialog-warning");
        });
        assert!(matches!(p.icon, Some(Icon::Themed(_))));
        if let Some(Icon::Themed(names)) = p.icon {
            assert_eq!(names, vec!["dialog-warning".to_owned()]);
        }
    }

    #[test]
    fn themed_icon_from_icon() {
        let p = classic_to_portal(|n| {
            n.icon("bell");
        });
        assert!(matches!(p.icon, Some(Icon::Themed(_))));
    }

    #[test]
    fn no_icon_set_is_none() {
        let p = classic_to_portal(|n| {
            n.summary("t");
        });
        assert!(p.icon.is_none());
    }

    // -----------------------------------------------------------------------
    // Action / button conversion
    // -----------------------------------------------------------------------

    #[test]
    fn empty_actions_produces_no_buttons_and_no_default() {
        let p = classic_to_portal(|n| {
            n.summary("t");
        });
        assert!(p.buttons.is_empty());
        assert!(p.default_action.is_none());
    }

    #[test]
    fn default_action_stores_the_key_not_the_label() {
        // The portal spec requires default-action to be the action *key* ("default"),
        // not the human-readable label ("Open").
        let p = classic_to_portal(|n| {
            n.action("default", "Open");
        });
        assert_eq!(p.default_action.as_deref(), Some("default"));
        assert!(p.buttons.is_empty());
    }

    #[test]
    fn non_default_actions_become_buttons() {
        let p = classic_to_portal(|n| {
            n.action("ok", "OK").action("cancel", "Cancel");
        });
        assert!(p.default_action.is_none());
        assert_eq!(p.buttons.len(), 2);
    }

    #[test]
    fn mixed_actions_split_correctly() {
        // ["default", "Open", "ok", "OK"] →
        //   default-action = "default", buttons = [Button { action: "ok", label: "OK" }]
        let p = classic_to_portal(|n| {
            n.action("default", "Open").action("ok", "OK");
        });
        assert_eq!(p.default_action.as_deref(), Some("default"));
        assert_eq!(p.buttons.len(), 1);
    }

    // -----------------------------------------------------------------------
    // portal::Notification standalone builder
    // -----------------------------------------------------------------------

    #[test]
    fn standalone_title_required() {
        let n = Notification::new("Hello");
        assert_eq!(n.title, "Hello");
    }

    #[test]
    fn standalone_id_overrides_auto_id() {
        let n = Notification::new("t").id("my-stable-id");
        assert_eq!(n.id.as_deref(), Some("my-stable-id"));
    }

    #[test]
    fn standalone_no_id_is_none() {
        let n = Notification::new("t");
        assert!(n.id.is_none());
    }

    #[test]
    fn standalone_priority_set() {
        let n = Notification::new("t").priority(Priority::Urgent);
        assert!(matches!(n.priority, Some(Priority::Urgent)));
    }

    #[test]
    fn standalone_button_appended() {
        let n = Notification::new("t")
            .button(Button::new("open", "Open"))
            .button(Button::new("cancel", "Cancel"));
        assert_eq!(n.buttons.len(), 2);
    }

    #[test]
    fn standalone_default_action_set() {
        let n = Notification::new("t").default_action("open");
        assert_eq!(n.default_action.as_deref(), Some("open"));
    }

    // -----------------------------------------------------------------------
    // Priority serialization (via Display)
    // -----------------------------------------------------------------------

    #[test]
    fn priority_display_strings() {
        assert_eq!(Priority::Low.to_string(), "low");
        assert_eq!(Priority::Normal.to_string(), "normal");
        assert_eq!(Priority::High.to_string(), "high");
        assert_eq!(Priority::Urgent.to_string(), "urgent");
    }

    // -----------------------------------------------------------------------
    // NotificationId
    // -----------------------------------------------------------------------

    #[test]
    fn notification_id_portal_roundtrip() {
        use crate::xdg::NotificationId;
        let id = NotificationId::Portal("abc".to_owned());
        assert_eq!(id.as_portal(), Some("abc"));
        assert_eq!(id.as_global(), None);
    }

    #[test]
    fn notification_id_global_roundtrip() {
        use crate::xdg::NotificationId;
        let id = NotificationId::Global(42);
        assert_eq!(id.as_global(), Some(42));
        assert_eq!(id.as_portal(), None);
    }
}
