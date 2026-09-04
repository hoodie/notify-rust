# XDG Desktop Portal — Notification API: Engineering Learnings

> Discoveries made while implementing `notify-rust` portal support.
> These are things that are **not obvious from the spec alone** and were
> found by reading `xdg-desktop-portal` source code and observing runtime
> behaviour.

---

## 1. The frontend interface does not take `app_id`

The portal spec defines two separate D-Bus interfaces:

| Interface                                  | Used by                                      |
| ------------------------------------------ | -------------------------------------------- |
| `org.freedesktop.portal.Notification`      | Applications (us)                            |
| `org.freedesktop.impl.portal.Notification` | Notification backend (e.g. GNOME Shell, KDE) |

Only the _backend_ interface carries `app_id` in its method signatures:

```
// Backend (impl) interface — NOT what we call
AddNotification(app_id s, id s, notification a{sv})
```

The _frontend_ interface that applications call has no `app_id` at all:

```
// Frontend interface — what we call
AddNotification(id s, notification a{sv})
RemoveNotification(id s)
```

The portal daemon derives the caller's identity from the D-Bus sender name
and resolves it to an `app_id` internally before forwarding to the backend.
Passing `app_id` as an argument to the frontend causes a runtime
`org.freedesktop.DBus.Error.InvalidArgs` error.

**Source:** `notification.c` → `notification_handle_add_notification`,
`notification_handle_remove_notification`.

---

## 2. Notification IDs are scoped by `(app_id, id)`, not by connection

The portal tracks active notifications in a hash table keyed by a `Pair`:

```c
typedef struct {
  char *app_id;
  char *id;
} Pair;

GHashTable *active; /* Pair → char *sender */
```

The `id` field is the caller-supplied string from `AddNotification`.
The `app_id` is resolved by the portal from the calling process — **not**
from anything the caller passes.

For **sandboxed apps** (Flatpak, Snap) the `app_id` is the application's
declared identifier (e.g. `"org.example.MyApp"`).

For **unsandboxed (host) apps** the portal tries to derive an `app_id` from
the calling process's systemd unit name (via `sd_pid_get_user_unit`). If the
process was not launched through a systemd app unit — which is the common
case for anything run from a terminal — **`app_id` falls back to `""`**
(empty string).

This means all unsandboxed terminal processes share the same `app_id = ""`
namespace for notification IDs.

**Source:** `xdp-app-info-host.c` → `xdp_app_info_host_new`,
`get_app_from_pid`.

---

## 3. The portal evicts notifications when the sender disconnects

```c
static void
on_peer_disconnect (XdpContext *context, const char *sender, gpointer user_data)
{
  // Removes every notification in `active` whose app_id matches the
  // app_id of the disconnecting sender.
  ...
  while (g_hash_table_iter_next (&iter, (gpointer *)&p, NULL))
    {
      if (g_strcmp0 (p->app_id, sender_app_id) == 0)
        g_hash_table_iter_remove (&iter);
    }
}
```

When a D-Bus connection closes, the portal removes every entry from its
`active` table whose `app_id` matches the disconnecting sender's `app_id`.

For **sandboxed apps** this is per-application (reasonable isolation).

For **unsandboxed apps** where `app_id = ""` this means: when _any_
unsandboxed sender disconnects, **every** notification that was sent by any
unsandboxed process (in the same portal session) is evicted from the active
table.

In practice, because a terminal-launched process is the only one sending
notifications during a test session, this means: when _our_ connection
closes, our notification is forgotten.

**Source:** `notification.c` → `on_peer_disconnect`.

---

## 4. The "same ID = update" guarantee only works on the same connection

The spec says:

> "If the application reuses the same ID without withdrawing, the
> notification is updated with the new one."

This is true — but only if the second `AddNotification` arrives **over the
same D-Bus connection** (or at least a connection with the same `app_id`).

If the pattern is:

```
Connection A: AddNotification(id="x", ...)   → active: ("", "x") → sender A
Connection A drops → on_peer_disconnect removes ("", "x")
Connection B: AddNotification(id="x", ...)   → active: ("", "x") → sender B  ← NEW entry, not a replace
```

then the portal creates a _new_ notification popup rather than replacing the
first, because the active table entry was evicted when connection A closed.

For the update guarantee to hold the correct pattern is:

```
Connection A: AddNotification(id="x", body="0%")  → active: ("", "x") → A
Connection A: AddNotification(id="x", body="50%") → active: ("", "x") → A  ← in-place replace ✓
Connection A: AddNotification(id="x", body="100%")→ active: ("", "x") → A  ← in-place replace ✓
Connection A: RemoveNotification(id="x")
Connection A drops → nothing to evict
```

**Practical implication for notify-rust:** all update calls must reuse the
`zbus::Connection` that was used to send the original notification. This is
why `PortalNotificationHandle::update_with()` passes `&self.connection` rather
than opening a new session connection, and why calling
`Notification::new(...).id(STABLE_ID).show()` in a loop does **not** produce
in-place updates.

---

## 5. `handle.update()` vs `handle.update_with(new_notification)`

The existing `handle.update()` on `PortalNotificationHandle` re-sends the
**originally stored content** over the existing connection. It is useful when
you want to bump the notification to the top without changing its text.

To update the **content** (body, title, priority, etc.) use
`handle.update_with(new_notification)`, which:

1. Accepts a fresh `portal::Notification` describing the new state.
2. Sends `AddNotification` with `handle.id` and `handle.connection`.
3. The notification's `.id()` value (if any) is ignored — the handle's ID
   always takes precedence.

---

## 6. Icon validation is done by the portal, not the backend

File-descriptor icons are validated **before** being forwarded to the
notification backend:

```c
if (xdp_validate_icon (sealed_icon, XDP_ICON_TYPE_NOTIFICATION, NULL, NULL))
  { /* forward to backend */ }
/* else: silently drop — no error returned to caller */
```

Constraints enforced by the `notification` ruleset (from `validate-icon.c`):

| Constraint      | Limit                |
| --------------- | -------------------- |
| Formats         | `png`, `jpeg`, `svg` |
| Must be square  | `width == height`    |
| Max raster size | 512 × 512 px         |
| Max SVG size    | 4096 × 4096 px       |
| Max file size   | 4 MB                 |

Icons that fail validation are **silently dropped**. The notification is
shown without an icon and no error is returned to the caller. There is no
way to distinguish "icon accepted" from "icon rejected" from the application
side.

Themed icons (`("themed", ["name"])`) bypass validation entirely.

**Practical implication:** always test with images that satisfy all
constraints. The `examples/octodex-256.png` (256 × 256 px PNG) was chosen
specifically to pass validation. The original `octodex.jpg` / `octodex.png`
(896 × 896 px) fails with "Image too large" and is silently dropped.

Local validation tool (on systems that have it):

```sh
/usr/lib/xdg-desktop-portal-validate-icon \
    --path=examples/octodex-256.png \
    --ruleset=notification
```

---

## 7. The `file-descriptor` icon format requires a sealed `memfd`

The portal validates file-descriptor icons by calling
`xdp_sealed_fd_new_from_handle`, which requires the fd to be a **sealable**
anonymous memory file created with `memfd_create(MFD_ALLOW_SEALING)`.

Regular file descriptors opened with `open()` on a filesystem path are
**not** accepted — the portal will return:

```
Invalid file descriptor: The file descriptor needs to be sealable
```

The correct procedure:

1. `memfd_create("name", MFD_ALLOW_SEALING)` — create the anonymous fd.
2. `ftruncate(fd, size)` — size it to match the source file.
3. Copy file contents in (e.g. `std::io::copy`).
4. `fcntl(fd, F_ADD_SEALS, F_SEAL_SHRINK | F_SEAL_GROW)` — seal against
   size changes. The portal calls `lseek(fd, 0, SEEK_SET)` itself before
   reading, so no seek back to the start is required from the caller.

In Rust this is implemented via the `memfd` crate. See
`src/xdg/zbus_rs/portal.rs` → `Icon::copy_file_to_sealed_memfd`.

---

## 8. `NotificationClosed` does not exist on the frontend interface

The `NotificationClosed(id u32, reason u32)` signal is part of the **classic**
`org.freedesktop.Notifications` interface. It does **not** exist on
`org.freedesktop.portal.Notification`.

The portal spec only defines one signal on the frontend interface:

```
ActionInvoked(id s, action s, parameter av)
```

This signal is emitted for **non-exported** actions (those whose name does not
start with `app.`). A plain dismissal by the user — clicking the × button or
swiping the notification away — produces **no signal** on the frontend
interface.

**Practical implication:** `PortalNotificationHandle::on_close` cannot be
implemented correctly for portal notifications. It will block forever waiting
for an `ActionInvoked` signal that will never arrive for a plain dismissal.
Document this limitation clearly rather than silently hanging.

<<<<<<< HEAD
The `NotificationClosed` signal _does_ exist on the **backend implementation**
=======
> **Update:** the *other* half of this — what happens once an action click *does* fire —
> was also buggy: `on_close`/`wait_for_action` used to consume the handle (`self`), so
> there was no way to call `handle.close()` afterwards even though the portal never
> auto-removes a notification on action click (see §14). Fixed by changing both to take
> `&self`, matching the classic handle. See `tasks.md` Phase 4 for details.

The `NotificationClosed` signal *does* exist on the **backend implementation**
>>>>>>> a1b9ff5 (WIP)
interface (`org.freedesktop.impl.portal.Notification`), which only the portal
daemon's backend plugins can access — not applications.

---

## 9. `ActionInvoked` parameter layout (portal v2)

The `parameter av` array in the frontend `ActionInvoked` signal carries:

```
parameter[0]  — action target value (optional, present if the button had a `target`)
parameter[1]  — platform data vardict a{sv}, always contains:
                  "activation-token" s  — XDG Activation token for window focusing
parameter[2]  — user response (optional, only for inline-reply actions)
```

The activation token can be used with the XDG Activation protocol to raise
the application window when the user clicks an action button. It is currently
ignored in notify-rust but is available for future use.

**Source:** portal spec,
`org.freedesktop.portal.Notification::ActionInvoked` description.

---

## 10. Backend version detection affects wire format

The portal's `notification.c` checks `impl_version` (the version advertised
by the backend, e.g. GNOME Shell's notification backend) when deciding what
wire format to use:

- `impl_version < 2`: icons are converted from `file-descriptor` to `bytes`
  before forwarding, because older backends do not understand fd icons.
- `impl_version > 1`: `markup-body`, `sound`, `display-hint`, `category`,
  and `button.purpose` are accepted; on older backends they are silently
  filtered.

As an application caller we do not control or see `impl_version`. The portal
handles the translation transparently. However, this means features like
`markup-body` and `display-hint` may not reach the actual notification
backend on older desktop environments even if we send them correctly to the
portal.

---

## 11. Systemd unit detection for `app_id` on the host

For unsandboxed (host) processes, `xdg-desktop-portal` determines the `app_id`
by calling `sd_pid_get_user_unit` on the D-Bus sender's PID and then parsing
the unit name with this regex pair (from `xdp-app-info-host.c`,
`_xdp_app_info_host_parse_app_id_from_unit_name`):

```
# scope / slice
app[-<launcher>]-<ApplicationID>-<RANDOM>.scope
app[-<launcher>]-<ApplicationID>-<RANDOM>.slice

# service
app[-<launcher>]-<ApplicationID>[@<RANDOM>].service
```

If the process has no matching user service unit — which is always the case for
processes launched from a terminal (`cargo run`, a plain shell, etc.) — the
`app_id` falls back to `""`.

Hyphens in the `ApplicationID` segment are hex-escaped by systemd as `\x2d`
in the unit name (e.g. `org.notify-rust` → `app-org.notify\x2drust.service`).
Prefer pure dot-separated IDs to avoid the escaping.

**Impact varies by backend** — see §13 for the GNOME-specific consequence of
`app_id = ""`, which is severe (silent notification drop). On KDE and other
backends the app_id is used only for scoping the `(app_id, id)` active-table
key, so `""` is consistent across calls from the same session and does not
prevent notifications from appearing.

---

## 13. GNOME Shell silently drops notifications when `app_id` is invalid or has no `.desktop` file

**TL;DR**: On GNOME, any portal notification whose `app_id` is `""` (the
default for terminal-launched processes — see §11) is **silently dropped**.
No error is returned to the caller. This is the single biggest practical
obstacle to developing the portal path on GNOME.

### What GNOME Shell does with the app ID

`xdg-desktop-portal-gnome`'s `notification.c` receives the `AddNotification`
call from the portal frontend and immediately forwards it verbatim to GNOME
Shell via `org.gtk.Notifications.AddNotification(app_id, id, notification)`.

GNOME Shell's `GtkNotificationDaemon.AddNotificationAsync`
(`js/ui/notificationDaemon.js`) calls `_ensureAppSource(appId)`, which
constructs a `GtkNotificationDaemonAppSource`. That constructor performs two
checks and throws `InvalidAppError` if either fails:

```js
// gnome-shell 49.x — notificationDaemon.js
constructor(appId, dbusImpl) {
    if (!Gio.Application.id_is_valid(appId))   // check 1
        throw new InvalidAppError();

    const app = Shell.AppSystem.get_default()
        .lookup_app(`${appId}.desktop`);        // check 2
    if (!app)
        throw new InvalidAppError();
    ...
}
```

`AddNotificationAsync` catches `InvalidAppError` and returns a D-Bus error to
`xdg-desktop-portal-gnome`, which logs it and swallows it. The caller's
`AddNotification` call already returned `()` (success) by this point, so from
the library's perspective the call succeeded.

The journal shows:

```
xdg-desktop-portal-gnome: Error from gnome-shell:
  GDBus.Error:org.gtk.Notifications.Error.InvalidApp:
  The app by ID "" could not be found
```

### Check 1 — valid GLib application ID

`Gio.Application.id_is_valid` requires:

- At least two dot-separated components
- Each component is alphanumeric (hyphens allowed within components)
- No leading, trailing, or consecutive dots

`""` fails immediately. Any non-empty reverse-DNS string like `org.notify.rust`
passes.

### Check 2 — installed `.desktop` file

`Shell.AppSystem.lookup_app` calls `shell_app_cache_get_info`, which is backed
by GIO's `g_desktop_app_info_new(desktop_id)`. GIO searches:

- `$XDG_DATA_HOME/applications` — i.e. `~/.local/share/applications/`
- Every directory in `$XDG_DATA_DIRS` — typically `/usr/local/share`,
  `/usr/share`, Flatpak export paths, etc.

**`~/.local/share/applications/` is sufficient** — no system path is needed.
The file must be named `<app-id>.desktop` and contain a valid `[Desktop Entry]`
block. A minimal stub works:

```ini
[Desktop Entry]
Name=My App
Type=Application
Exec=false
NoDisplay=true
```

After creating the file, run `update-desktop-database ~/.local/share/applications/`
to refresh the cache.

### Making it work end-to-end on GNOME

Both conditions must be satisfied simultaneously:

**Step 1** — install the `.desktop` file as above.

**Step 2** — launch the process as a systemd user service unit whose name
follows the `app-<id>.service` pattern so the portal can extract a non-empty
`app_id` from the cgroup (see §11):

```sh
systemd-run --user --service-type=oneshot \
    --working-directory=/path/to/project \
    --unit=app-org.notify.rust.service \
    ./target/debug/examples/desktop-portal
```

Note that `systemd-run` defaults its working directory to `$HOME`, not the
shell's `$PWD`, so `--working-directory` is required if the binary uses
relative paths.

### No clean workaround from the library side

The `XDG_DESKTOP_PORTAL_TEST_APP_ID` environment variable exists in the portal
source (`maybe_create_test_app_info` in `xdp-app-info.c`) and bypasses the
systemd unit lookup. However, it must be set in the **portal daemon's**
environment, not the caller's — meaning it requires restarting
`xdg-desktop-portal` with a custom environment, which is more invasive than
the two-step setup above.

The portal does not expose any mechanism for an unsandboxed caller to assert
its own `app_id`. The identity is always derived server-side from the process's
cgroup.

### Practical consequence for development

The portal path is easiest to develop and test on **KDE** (no setup required)
or inside a **Flatpak sandbox** (the intended deployment environment). On GNOME
the two-step setup works but requires modifying the local system, which is
undesirable for a clean dev environment.

Using the classic `org.freedesktop.Notifications` path (`show()` instead of
`show_via_portal()`) works on GNOME without any setup and is the right default
for unsandboxed applications.

**Sources verified against:**

- `xdg-desktop-portal` 1.20.3 — `src/xdp-app-info-host.c`
- `xdg-desktop-portal-gnome` 49.0 — `src/notification.c`
- `gnome-shell` 49.5 — `js/ui/notificationDaemon.js`

---

## 12. RETRACTED — KDE Plasma in-place update claim was unverified and appears incorrect

> **Correction (2026-09-01):** This section originally claimed that KDE Plasma's portal
> backend *always* creates a new popup instead of replacing an existing notification, and
> cited `xdg-desktop-portal-kde/src/notification.cpp` — including a supposed source snippet
> of `NotificationPortal::AddNotification` — as evidence.
>
> Both claims turned out to be wrong:
>
> 1. **Live re-test on KDE Plasma 6.7.4-2 (Wayland, `xdg-desktop-portal` 1.22.1-2 +
>    `xdg-desktop-portal-kde` 6.7.4-2)** showed the *opposite* of what was documented here:
>    - `examples/portal-update-plain.rs` (bare `handle.update()`, three consecutive calls) —
>      the notification **refreshed in place**, not three separate popups.
>    - `examples/portal-reuse.rs` (two `AddNotification` calls with the same explicit `.id()`,
>      each on its own connection) — the second call **replaced** the first, not stacked.
> 2. **The cited source file does not exist.** A full listing of `src/` in the
>    `xdg-desktop-portal-kde` repo (`master` branch, checked via the GitLab API) contains no
>    `notification.cpp` or similarly named file implementing `org.freedesktop.impl.portal.Notification`.
>    The only notification-related file is `notificationinhibition.cpp`, which implements Do
>    Not Disturb inhibition, not `AddNotification`. The C++ snippet quoted below as "what KDE
>    actually does" cannot be verified against any file in the current upstream repository.
>
> **Conclusion:** this section's central claim was not backed by a real investigation and
> should be treated as unreliable. It's possible an older KDE Plasma version genuinely had
> this bug and it was later fixed (upstream KDE notifications went through some plumbing
> changes around Plasma 6), or the original claim was simply fabricated. Either way, **do
> not rely on the content below** — it is kept only for historical context. The corrected,
> verified behavior is: bare `update()` and same-ID reuse both correctly replace the visible
> notification in place on KDE Plasma 6.7.4. All rustdoc and example doc comments referencing
> this alleged limitation have been corrected to reflect the verified behavior. If you hit a
> KDE version where this *does* stack popups, please replace this section with an actually
> verified repro (KDE version, exact commands, screenshots/observations) rather than
> restoring the text below.

<details>
<summary>Original (unverified, likely incorrect) claim — kept for historical context only</summary>

**TL;DR**: On KDE Plasma, calling `AddNotification` with the same ID a second time (even over the same D-Bus connection) will **always** produce a new popup rather than replacing the first. This is a backend bug/limitation, not a caller error.

This does **not** affect the classic `org.freedesktop.Notifications.Notify` path —
in-place updates work correctly on KDE via the classic interface because `replaces_id`
is a first-class field of the `Notify` method signature, not an inferred semantic.
See §12.3 below.

### What the spec says

The freedesktop.org portal spec says:

> "If a notification with the same `id` already exists, the notification is replaced."

The `xdg-desktop-portal` frontend daemon (`notification.c`) enforces exactly this: it records `(app_id, id) → sender` in its `active` hash table, and a second `AddNotification` with the same key is forwarded to the backend as a replacement call.

### What KDE actually does (UNVERIFIED — see correction above)

`xdg-desktop-portal-kde`'s `NotificationPortal::AddNotification` (`notification.cpp`) **unconditionally** creates a brand new `KNotification` object for every call, regardless of whether a notification with that `(app_id, id)` key already exists in `m_notifications`:

```cpp
void NotificationPortal::AddNotification(const QString &app_id, const QString &id, const QVariantMap &notification)
{
    // Always creates a new object — no check for an existing entry:
    KNotification *notify = new KNotification(QStringLiteral("notification"), ...);
    // ... configure notify ...
    m_notifications.insert(QStringLiteral("%1:%2").arg(app_id, id), notify);
    notify->sendEvent();  // fires a new popup every time
}
```

`QHash::insert` silently replaces the old pointer in the map (causing a memory leak of the old `KNotification` object) and `sendEvent()` fires a fresh popup. The old visible notification is **never closed** before the new one is shown.

### Observed behaviour on KDE Plasma 6.x (UNVERIFIED — contradicted by live re-test, see correction above)

| Call sequence                                            | GNOME / mako / dunst | KDE Plasma   |
| -------------------------------------------------------- | -------------------- | ------------ |
| `AddNotification(id="1", body="0%")`                     | New popup            | New popup    |
| `AddNotification(id="1", body="50%")` — same connection  | In-place update      | Second popup |
| `AddNotification(id="1", body="100%")` — same connection | In-place update      | Third popup  |

### Workaround

There is no clean workaround from the caller side. Options:

1. **Explicit close before update**: call `RemoveNotification(id)` before each
   `AddNotification(id, new_content)`. This avoids stacking popups but produces
   a visible flicker (dismiss + new popup) rather than a smooth in-place update.
   It also requires two round-trip D-Bus calls per update step.

2. **Accept the limitation**: document that `update_with()` / `handle.update()`
   produce smooth in-place replacement on spec-compliant backends (GNOME Shell,
   mako, dunst, …) but will stack new popups on KDE Plasma until upstream fixes
   the issue.

3. **Runtime backend detection**: query `org.freedesktop.portal.Desktop`'s
   `org.freedesktop.DBus.Properties.Get` → `org.freedesktop.portal.Notification`
   → `version` and cross-reference with known broken backend names to choose
   strategy automatically. This is fragile and not recommended.

The **recommended approach** for `notify-rust` is option 2: document the limitation
clearly (in `update_with`'s rustdoc and in the example) without adding workaround
complexity that penalises spec-compliant desktops.

### 12.3 Why the classic `Notify` path works fine on KDE

The classic `org.freedesktop.Notifications` interface defines its `Notify` method as:

```
Notify(app_name s, replaces_id u, app_icon s, summary s, body s,
       actions as, hints a{sv}, expire_timeout i) → id u
```

`replaces_id` is an **explicit parameter** in the method signature. When it is
non-zero the notification daemon is required by the spec to replace the identified
notification in-place. KDE's `plasmashell` notification daemon honours this field
correctly — it looks up the existing notification by numeric ID and replaces it.

The portal `AddNotification` method has **no such parameter**:

```
AddNotification(id s, notification a{sv})
```

The replacement semantic is entirely implicit: the backend is supposed to infer "this
is a replacement" by checking whether `(app_id, id)` already exists in its active
<<<<<<< HEAD
table. That inference step is what `xdg-desktop-portal-kde` skips.
=======
table.  That inference step is what `xdg-desktop-portal-kde` supposedly skips (per the
unverified claim above).
>>>>>>> a1b9ff5 (WIP)

In short (per the unverified claim above — contradicted by live re-test):

<<<<<<< HEAD
| Interface                                             | Update mechanism                   | KDE behaviour       |
| ----------------------------------------------------- | ---------------------------------- | ------------------- |
| `org.freedesktop.Notifications.Notify`                | Explicit `replaces_id u` parameter | ✅ Works            |
| `org.freedesktop.portal.Notification.AddNotification` | Implicit: same `(app_id, id)` key  | ❌ Always new popup |
=======
| Interface | Update mechanism | KDE behaviour |
|-----------|-----------------|---------------|
| `org.freedesktop.Notifications.Notify` | Explicit `replaces_id u` parameter | ✅ Works |
| `org.freedesktop.portal.Notification.AddNotification` | Implicit: same `(app_id, id)` key | ❌ Always new popup (claimed, unverified) |
>>>>>>> a1b9ff5 (WIP)

### Upstream reference (citation could not be verified — file does not exist upstream)

- KDE source: `xdg-desktop-portal-kde/src/notification.cpp`, `NotificationPortal::AddNotification`
  — https://invent.kde.org/plasma/xdg-desktop-portal-kde/-/blob/master/src/notification.cpp
  (checked 2026-09-01: no such file exists in `master`'s `src/` directory)
- No upstream bug filed as of the time of writing (KDE 6.6.x); the issue has been
  present since at least the initial KNotification-based implementation.

</details>

---

## 14. Notifications outlast the app: no auto-removal on `ActionInvoked`

The portal spec says:

> "Note that in contrast to most other portal requests, notifications are expected to
> outlast the running application. If a user clicks on a notification after the
> application has exited, it will get activated again."

Neither `AddNotification` nor the `ActionInvoked` signal description say anything about
the notification being removed when an action is invoked. In other words: clicking a
button fires `ActionInvoked`, but the notification **stays visible** until the app calls
`RemoveNotification` (or the user dismisses it). This was confirmed live on KDE Plasma
6.7.4 — clicking a button did not make the popup disappear on its own.

**Bug this exposed:** `PortalNotificationHandle::wait_for_action` and `::on_close` used
to take `self` (consuming the handle), so there was no way to call `handle.close()`
afterwards even if you wanted to clean up once the action was handled — the handle was
already gone. Fixed by changing both to take `&self`, matching the classic
`ZbusNotificationHandle`'s signatures. `examples/portal-actions.rs` and
`examples/portal-on-close.rs` now call `handle.close()` after handling the response, and
this was verified live to actually withdraw the notification.

---

## Summary: rules for correct portal notification updates

<<<<<<< HEAD
| Rule                                                                                           | Reason                                                                         |
| ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| Keep the `PortalNotificationHandle` alive for the duration                                     | Its `zbus::Connection` must not drop                                           |
| Use `handle.update_with(new_notification)` to change content                                   | Sends `AddNotification` on the existing connection                             |
| Never open a new connection for a second `AddNotification` with the same ID                    | Connection drop evicts the active-table entry; second send creates a new popup |
| Use caller-supplied `.id()` only when you need stable identity across explicit close+reopen    | Within a single handle lifetime the auto-generated ID is sufficient            |
| Themed icons are always safe; file-descriptor icons require a sealed memfd and pass validation | Silent drop on failure; no error returned                                      |
| Do not register a `NotificationClosed` signal listener on the portal interface                 | The signal does not exist; listening for it will block forever                 |
| In-place update via `AddNotification` with the same ID is **not guaranteed** on KDE Plasma     | The KDE backend always creates a new popup; see §12                            |
=======
| Rule | Reason |
|------|--------|
| Keep the `PortalNotificationHandle` alive for the duration | Its `zbus::Connection` must not drop |
| Use `handle.update_with(new_notification)` to change content | Sends `AddNotification` on the existing connection |
| Never open a new connection for a second `AddNotification` with the same ID | Connection drop evicts the active-table entry; second send creates a new popup |
| Use caller-supplied `.id()` only when you need stable identity across explicit close+reopen | Within a single handle lifetime the auto-generated ID is sufficient |
| Themed icons are always safe; file-descriptor icons require a sealed memfd and pass validation | Silent drop on failure; no error returned |
| Do not register a `NotificationClosed` signal listener on the portal interface | The signal does not exist; listening for it will block forever |
| In-place update via `AddNotification` with the same ID replaces the notification correctly on KDE Plasma 6.7.4 (live-verified) | An earlier claim that KDE always creates a new popup turned out to be unverified/incorrect; see §12 |
>>>>>>> a1b9ff5 (WIP)
