# Windows back-end — open work

All items below can be done without breaking the existing public API.

## Internal-only changes (no API surface needed)

- **`Scenario::Urgent` → `Reminder` fallback on pre-22H2**
  Detect the Windows build number via `RtlGetVersion` at runtime.
  Use `Scenario::Urgent` on build ≥ 22621, `Scenario::Reminder` otherwise.

- **Looping audio for `Alarm*` / `Call*` sounds**
  `Audio::new(src, loop_, silent)` — `loop_` is always `false` today.
  Set it to `true` when `src` is any `Alarm*` or `Call*` variant, and
  force `ToastDuration::Long` (required by WinRT for looping to work).

- **`with_expiry` for millisecond timeouts**
  `Timeout::Milliseconds(ms)` is currently bucketed into Short/Long only.
  Also call `builder.with_expiry(Duration::from_millis(ms))` so the
  Action Center entry expires at roughly the right time.

## Additive API (new methods, no breaking changes)

- **Action buttons**
  `Notification::actions` (`Vec<String>`) already exists and is populated
  by the existing `.action(id, label)` method — it is just ignored on Windows.
  Wire it up: iterate pairs and add `ActionButton::create(label).with_id(id)`.
  Register a `NotificationActivatedEventHandler` on the builder and surface
  the chosen action ID through `NotificationHandle` (new method or channel).

- **Suppress popup / silent delivery**
  Add `Notification::suppress_popup(bool)` (Windows-only, `cfg`-gated).
  Call `ManageNotification::set_suppress_popup(true)` after `build()`.
  Useful for background updates that should land in the Action Center
  without interrupting the user.

## Future optional features (require new API design)

- **`Scenario::Alarm` / `IncomingCall`**
  No mapping from the existing `notify-rust` API. Would need a new
  Windows-only method or a cross-platform `Scenario` concept.

- **Progress bar**
  `win32_notif` exposes a data-bindable `Progress` visual. Needs a new
  `Notification::progress(value, status)` API and a matching update path
  in `NotificationHandle`.

- **Click / activated handler**
  Register a `NotificationActivatedEventHandler` on the builder.
  Needs a new `NotificationHandle::on_activate(fn)` method and a decision
  on whether action-button IDs and body-click are surfaced separately.

