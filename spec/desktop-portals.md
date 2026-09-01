# Desktop Portal Notification — Spec & Implementation Plan

> **Working document.** Do not write or update any `///` doc comments on new or modified methods
> until the implementation is complete and the PR is ready to open. Documenting a moving target
> wastes effort and misleads readers of the source. The final documentation pass is its own task,
> listed at the bottom of this file.

## Reference

- Interface: `org.freedesktop.impl.portal.Notification` (version 2)
- Doc: `doc-org.freedesktop.impl.portal.Notification.rst.txt`
- Prior art: [`ashpd`](https://docs.rs/ashpd) — the reference Rust portal implementation

---

## API Design

### Guiding principles

1. **The type system is the guide.** Methods that do not exist on the portal path must not appear
   after `.portal()`. No fake no-ops, no silently ignored fields. If the compiler accepts it, it works.
2. **No new blocking APIs.** The portal path is inherently async. All portal methods are `async fn`.
   No `block_on` wrappers will be introduced for portal code.
3. **Explicit conversion point.** The transition from classic to portal is visible in the builder
   chain as `.portal()`, not hidden in a terminal method name like `show_via_portal()`.
4. **ID is an implementation detail.** The notification ID is auto-generated on `show()` and owned
   by the handle. The caller does not supply it unless they have a specific reason to (e.g.
   cross-invocation replace/withdraw).

---

### The `.portal()` conversion method

`Notification` gains a `.portal()` method that consumes `self` and returns a
`portal::Notification`. From that point on, only methods defined on `portal::Notification` are
available. Methods from the classic `Notification` that have no portal equivalent — `hint()`,
`urgency()`, `timeout()`, etc. — **do not exist** on `portal::Notification` and will produce a
compiler error if called after `.portal()`.

```rust
// Basic portal send — clean and minimal
Notification::new()
    .summary("hello")
    .body("world")
    .portal()       // consumes Notification, returns portal::Notification
    .show()         // async, auto-generates ID
    .await?;

// Portal-specific features are only available after .portal()
Notification::new()
    .summary("Download complete")
    .portal()
    .priority(Priority::Normal)
    .button(Button::new("open", "Open File"))
    .button(Button::new("dismiss", "Dismiss"))
    .show()
    .await?;

// This must NOT compile — hint() does not exist on portal::Notification
Notification::new()
    .portal()
    .hint(Hint::Transient(true))  // compiler error: no method `hint` on portal::Notification
    .show()
    .await?;
```

The conversion inside `.portal()` does a best-effort translation of whatever was set before it.
See the [full concept comparison table](#classic-vs-portal-concept-map) for the complete picture.
The short version:

| `Notification` field                 | Translation into `portal::Notification`                          | Complexity |
| ------------------------------------ | ---------------------------------------------------------------- | ---------- |
| `summary`                            | `title`                                                          | trivial    |
| `body`                               | `body`                                                           | trivial    |
| `icon` (themed name)                 | `Icon::Themed(vec![name])`                                       | trivial    |
| `icon` (file path via `image_path`)  | `Icon::File(fd)` → sealed memfd                                  | medium     |
| `Hint::Urgency`                      | `Priority::from(Urgency)`                                        | trivial    |
| `Hint::Transient(true)`              | `display-hint: ["transient"]`                                    | easy       |
| `actions` (flat `[id, label, ...]`)  | zipped into `Button { action, label }` array                     | easy       |
| `actions` entry with key `"default"` | promoted to `default-action` field                               | easy       |
| `Hint::ImageData`                    | **dropped** — portal has no raw pixel data equivalent            | —          |
| `Hint::SoundName`                    | **dropped** — portal has no themed sound name equivalent         | —          |
| `Hint::SoundFile`                    | **dropped** — would need memfd path, out of scope for now        | —          |
| `Hint::DesktopEntry`                 | **dropped** — portal derives app identity from D-Bus sender      | —          |
| `Hint::X` / `Hint::Y`                | **dropped** — not supported by portal                            | —          |
| `Hint::ActionIcons`                  | **dropped** — not supported by portal                            | —          |
| `Hint::Resident`                     | **dropped** — no clean equivalent (`persistent` is not the same) | —          |
| `Hint::*` (everything else)          | **dropped** — no portal equivalent                               | —          |
| `timeout`                            | **dropped** — portal manages persistence via `display-hint`      | —          |

Fields that are dropped are silently lost at the conversion boundary. This is intentional — the
portal is a fundamentally different transport, not a superset. Users who need portal-specific
features (`Button::purpose`, `display-hint`, `category`, etc.) should set them after `.portal()`.

---

### `portal::Notification` — standalone use

`portal::Notification` can also be constructed directly without going through `Notification::portal()`,
for code that exclusively targets the portal path:

```rust
use notify_rust::portal::{Notification, Button, Priority, Icon};

Notification::new("Summary")       // title is required at construction, like ashpd
    .body("some body text")
    .priority(Priority::Urgent)
    .icon(Icon::named("dialog-warning"))
    .icon(Icon::file("./img.png"))
    .button(Button::new("open", "Open"))
    .show()
    .await?;
```

---

### ID management

The notification ID is a `String` chosen by the application (unlike the classic D-Bus path where
the server assigns a `u32`). It is:

- **Auto-generated** on `show()` if not explicitly set — a monotonic counter or similar is sufficient;
  a UUID is not required.
- **Overridable** via `.id("my-app.some-id")` for cases where the caller needs stable identity
  across invocations (e.g. a persistent progress notification).
- **Accessible** via `handle.id() -> &str` after `show()` returns, for the rare case where the
  caller needs to persist it.

The ID is used transparently by `handle.update()` (re-sends `AddNotification` with same ID) and
`handle.close()` (sends `RemoveNotification`). The caller does not need to manage it.

---

### `app_id`

The `app_id` parameter required by the portal spec (`AddNotification(app_id s, id s, notification a{sv})`)
is **not exposed as a user-facing builder method**. The portal daemon derives the caller's identity
from the D-Bus sender, so the `app_id` passed in the call is informational. It defaults to the
executable name (same source as `Notification::appname`). No builder method is needed.

---

### Signal handling — Streams not callbacks

`portal::NotificationHandle` exposes action signals as an async `Stream` rather than the
`FnOnce` callback pattern used by the classic `NotificationHandle`. This is more idiomatic for
async code and allows receiving multiple actions:

```rust
use futures_util::StreamExt;

let handle = Notification::new("title").portal().show().await?;

let mut actions = handle.actions().await?;
while let Some(action) = actions.next().await {
    match action.name() {
        "open"    => { /* ... */ }
        "dismiss" => { break; }
        _ => {}
    }
}
```

The classic `NotificationHandle::wait_for_action(FnOnce)` callback API is **unchanged** for
backward compatibility.

---

### Classic vs Portal concept map

This table covers every concept in both APIs, sourced directly from the wire specs:
[`org.freedesktop.Notifications`](https://specifications.freedesktop.org/notification-spec/latest/protocol.html)
and [`org.freedesktop.portal.Notification`](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.Notification.html) (version 2).

| Concept                   | Classic (`org.freedesktop.Notifications`)                                                                       | Portal (`org.freedesktop.portal.Notification` v2)                                                                                       | `.portal()` conversion                                                                  |
| ------------------------- | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| **Method call**           | `Notify(app_name, replaces_id, app_icon, summary, body, actions, hints, expire_timeout)` — positional args      | `AddNotification(id, notification a{sv})` — named vardict                                                                               | structural change                                                                       |
| **App identity**          | `app_name: STRING` in the call                                                                                  | derived from D-Bus sender; not in the call                                                                                              | drop — not needed                                                                       |
| **Notification ID**       | `replaces_id: UINT32` — server assigns; caller supplies to replace                                              | `id: STRING` — caller always supplies; reuse = replace                                                                                  | different types, same concept                                                           |
| **Title / Summary**       | `summary: STRING`                                                                                               | `title: s` in vardict                                                                                                                   | trivial rename                                                                          |
| **Body**                  | `body: STRING`; markup respected only if server advertises `body-markup` capability                             | `body: s` plain; `markup-body: s` is a _separate field_ — portal handles stripping if unsupported                                       | copy body; populate `markup-body` if body contains tags                                 |
| **Icon**                  | `app_icon: STRING` (themed name or `file://` URI) in positional args; also `ImagePath` / `ImageData` hints      | `icon: v` — a `(sv)` variant: `("themed", as)`, `("file-descriptor", h)`, or `("bytes", ay)`; themed takes an _array_ of fallback names | themed name → single-element array; file path → memfd; `ImageData` → dropped            |
| **Actions**               | flat `Vec<String>` of alternating `[id, label, id, label, ...]` pairs                                           | `buttons: aa{sv}` — array of vardicts, each with `label: s`, `action: s`, optional `target: v`, optional `purpose: s`                   | **easy** — zip pairs into `Button` vardicts                                             |
| **Default action**        | clicking notification body sends `ActionInvoked` with key `"default"` — implicit                                | `default-action: s` + optional `default-action-target: v` — explicit fields                                                             | promote `"default"` entry from actions vec                                              |
| **Urgency / Priority**    | `Hint::Urgency(Low/Normal/Critical)` — 3 levels                                                                 | `priority: s` — `"low"`, `"normal"`, `"high"`, `"urgent"` — 4 levels; `High` has no classic equivalent                                  | `Priority::from(Urgency)`; `High` is portal-only                                        |
| **Timeout / Persistence** | `expire_timeout: INT32` — `-1` = server default, `0` = never expire, `N` = ms                                   | no timeout field; persistence controlled by `display-hint: as` with `"transient"`, `"persistent"`, `"tray"` etc.                        | drop — no meaningful mapping                                                            |
| **Transient**             | `Hint::Transient(bool)`                                                                                         | `display-hint: ["transient"]`                                                                                                           | convertible                                                                             |
| **Sound**                 | `Hint::SoundName(String)` / `Hint::SoundFile(String)` / `Hint::SuppressSound(bool)`                             | `sound: v` — `("file-descriptor", h)` for a file, or string `"default"` / `"silent"`                                                    | `SoundName` → dropped; `SoundFile` → memfd (out of scope); `SuppressSound` → `"silent"` |
| **Category**              | `Hint::Category(String)` — free string, server-defined                                                          | `category: s` — standardised values (`"im.received"`, `"call.incoming"`, etc.) plus `x-vendor` extensions                               | pass through as-is                                                                      |
| **Desktop entry**         | `Hint::DesktopEntry(String)`                                                                                    | not in portal spec — identity from D-Bus sender                                                                                         | drop                                                                                    |
| **Image data (raw)**      | `Hint::ImageData(Image)` — raw pixel data as a struct                                                           | no equivalent — portal only accepts themed names or file descriptors                                                                    | drop                                                                                    |
| **Image path**            | `Hint::ImagePath(String)` — filesystem path                                                                     | `icon: ("file-descriptor", h)` — must be a sealed memfd                                                                                 | open + copy to memfd (already implemented)                                              |
| **X/Y position**          | `Hint::X(i32)` / `Hint::Y(i32)`                                                                                 | not supported                                                                                                                           | drop                                                                                    |
| **Action icons**          | `Hint::ActionIcons(bool)` — render action IDs as icon names                                                     | not supported                                                                                                                           | drop                                                                                    |
| **Resident**              | `Hint::Resident(bool)` — notification stays after action                                                        | no direct equivalent; closest is `display-hint: ["persistent"]` but semantics differ                                                    | drop                                                                                    |
| **Button purpose**        | not a concept                                                                                                   | `purpose: s` on a button — e.g. `"call.accept"`, `"im.reply-with-text"` (inline reply), `"system.custom-alert"`                         | portal-only                                                                             |
| **Display hints**         | not a concept                                                                                                   | `display-hint: as` — `"transient"`, `"tray"`, `"persistent"`, `"hide-on-lockscreen"`, `"show-as-new"`, etc.                             | portal-only; `Hint::Transient` partially maps here                                      |
| **Close notification**    | `CloseNotification(id: UINT32)`                                                                                 | `RemoveNotification(id: STRING)`                                                                                                        | trivial — different ID type                                                             |
| **Closed signal**         | `NotificationClosed(id: UINT32, reason: UINT32)` — reason: 1=expired, 2=dismissed, 3=closed by app, 4=undefined | **not defined** on `org.freedesktop.portal.Notification` — only on the backend impl interface; portal apps do not receive this signal   | N/A                                                                                     |
| **Action signal**         | `ActionInvoked(id: UINT32, action_key: STRING)`                                                                 | `ActionInvoked(id: STRING, action: STRING, parameter: av)` — `parameter` = `[target?, {activation-token}, user_response?]`              | compatible at name level; portal payload is richer                                      |
| **Activation token**      | separate `ActivationToken(id, token)` signal emitted _before_ `ActionInvoked`                                   | bundled into `ActionInvoked` as `parameter[1]` — always present                                                                         | portal is cleaner                                                                       |

---

### API concerns and decisions

These concerns were identified during design review and are recorded here for traceability.

#### 1. `actions` should be converted across `.portal()`, not dropped

The classic actions vec (`["id", "label", "id2", "label2", ...]`) can be mechanically converted
to portal `Button` vardicts by zipping the pairs. Dropping them silently was a footgun — a user
writing cross-platform code would lose their buttons without any compiler or runtime warning.

**Decision:** `.portal()` converts the flat actions vec into `Button` structs. The entry with
action key `"default"` is additionally promoted to the `default-action` field. `target` and
`purpose` will not be set during conversion — they are portal-only and can be added after `.portal()`.

#### 2. `Button::new(action, label)` argument order

`(action_key, label)` is the same reversed order as the classic `.action(id, label)` API.
Both are counter-intuitive because you read the visible label first, but the internal ID comes
first in the call.

**Decision:** Introduce a new `Action` type that replaces the raw string pairs in the classic
path _and_ the `Button` struct on the portal path, unifying both under one ergonomic API:

```rust
// Label-first construction — reads naturally
Action::new("Open File")          // label is the required argument
    .id("open")                   // optional explicit ID
// If no .id() is given, the ID is derived via slug::slugify(label):
//   "Open File" → "open-file"

// Used on the classic path:
Notification::new()
    .action(Action::new("Open File").id("open"))

// Used on the portal path (after .portal()):
Notification::new()
    .portal()
    .button(Action::new("Open File").id("open"))

// Or via conversion — .portal() carries actions across:
Notification::new()
    .action(Action::new("Open File").id("open"))
    .portal()   // Action is converted to Button vardict automatically
    .show()
    .await?;
```

`slug::slugify` is already a well-established crate for this purpose. The auto-generated ID means
users who do not care about stable action keys do not need to think about them at all.

#### 3. `NotificationClosed` signal does not exist on the portal interface

The portal spec (`org.freedesktop.portal.Notification`) does **not** define a `NotificationClosed`
signal. That signal only exists on the backend implementation interface
(`org.freedesktop.impl.portal.Notification`), which sandboxed apps cannot access.

**Decision:** Remove the `NotificationClosed` match rule from `wait_for_action_signal_portal`.
The `on_close` callback on `PortalNotificationHandle` cannot be supported via signal and should
either be removed or documented as a no-op on the portal path.

#### 4. `Stream` vs callback for action handling

The spec describes `handle.actions()` returning an async `Stream`, but the current
`PortalNotificationHandle::wait_for_action` uses the same `FnOnce` callback as the classic path.
The `Stream` design is more idiomatic for async code and allows receiving multiple actions without
re-subscribing.

**Decision:** The `Stream` API is the target. The callback form on the portal handle is a
stepping stone, not the final shape. Tracked as a task below.

---

### Comparison with `ashpd`

`ashpd` is the reference Rust portal implementation and informed several decisions here:

| Concern           | `ashpd` approach                                             | `notify-rust` approach                                                      |
| ----------------- | ------------------------------------------------------------ | --------------------------------------------------------------------------- |
| Transport vs data | Separate `NotificationProxy` + `Notification`                | Single builder chain with `.portal()` conversion                            |
| Actions           | `Button` struct with `label`, `action`, optional `target`    | `Action::new(label).id(action)` — label-first, ID auto-slugified if omitted |
| Icon              | `Icon::with_names(&[...])` for themed                        | `Icon::named(...)` / `Icon::file(...)`                                      |
| Priority          | `Display` + `AsRef<str>` + `From<Priority> for &'static str` | Same — drop current `Into`/`ToString` impl                                  |
| Signals           | `Stream<Item = Action>`                                      | Same for portal path                                                        |
| `app_id`          | Not user-facing                                              | Not user-facing                                                             |
| ID                | Caller supplies to `add_notification(id, notif)`             | Auto-generated, overridable via `.id()`                                     |
| Blocking API      | Async only                                                   | Async only for portal path                                                  |

`notify-rust` intentionally does not adopt the proxy-first model — keeping the builder-centric
API consistent across both paths is more important than strict architectural purity.

---

## Current State Overview

The implementation has three layers that need to work together:

1. **`Notification` builder API** — done; `icon`/`icon_named`/`icon_path` all work, `show_via_portal` takes `app_id` + `id` (ID auto-generation still pending)
2. **`PortalNotification` serialization** — mostly done; `title`, `body`, `priority`, `icon`, `default-action`, `buttons` all implemented; `markup-body` and `sound` stubbed as `None`
3. **`PortalNotificationHandle`** — done; `update()`, `close()`, and `wait_for_action()` all implemented
4. **Signal listening (`wait_for_action_signal_portal`)** — `ActionInvoked` works correctly; `NotificationClosed` correctly absent (portal does not define this signal)

**Architecture decisions made:**

- Portal support is **zbus-only**. The `dbus-rs` backend is synchronous/blocking and has no async event loop, making portal signal handling impractical. There is no plan to add portal support to `dbus-rs`.
- Portal support is **not a separate feature flag**. It is compiled in unconditionally whenever the `zbus` feature is active. This keeps the API surface simple and reflects that portals are the preferred path on modern sandboxed desktops.
- `memfd` and `nix` are gated behind the `zbus` feature (not unconditional). They are only needed for the sealed-memfd icon path used by the portal, so `dbus`-only users do not pay the compile cost.

---

## Implementation Plan & Checklist

### Phase 1 — Protocol Correctness (Sending)

**Interface clarification:** `app_id` only appears on the _backend implementation_ interface
(`org.freedesktop.impl.portal.Notification`). The _frontend_ interface that applications call
(`org.freedesktop.portal.Notification`) takes no `app_id` anywhere:

- `AddNotification(id s, notification a{sv})`
- `RemoveNotification(id s)`
- `ActionInvoked(id s, action s, parameter av)`

The `notification` vardict maps to `PortalNotification`.

- [x] `title` (maps to `summary`)
- [x] `body`
- [x] `priority` — correctly derived from `Urgency` hint via `Priority::from(Urgency)`
- [x] `icon` — themed path (via `icon_named`) and `file-descriptor` via sealed memfd
- [x] **`default-action`** — implemented in `PortalNotification::from`: the `"default"` action
      pair is promoted to the `default-action` field.
- [x] **`default-action-target`** — field present in `PortalNotification`; `None` during
      conversion (no classic notification carries a target value).
- [x] **`buttons`** — implemented in `PortalNotification::from`: flat `["id", "label", ...]`
      vec zipped into `Button { action, label }` vardicts; the `"default"` pair is excluded.
- [x] **`app_id` on the wire** — `app_id` has been fully removed from all portal plumbing.
      `show_via_portal()` takes no arguments. Passing it caused a runtime `InvalidArgs` error
      because the frontend interface does not accept it.
- [x] **`Icon::Type` signature** — fixed: `#[zvariant(signature = "(sv)")]` produces the
      correct wire type.
- [ ] **`markup-body`** — field exists but is always `None`. Should be populated when body
      contains HTML tags, or gated on a server capability check.
- [ ] **`sound`** — field exists, always `None`; low priority but stubbed.

---

### Phase 2 — ID Management

- [x] Auto-generate the portal notification ID — implemented as a process-global monotonic
      `AtomicU64` counter (`next_id()` in `portal.rs`). No new dependency required.
      `show_via_portal()` now takes no arguments at all.
- [x] Expose the generated `id` through `NotificationHandle::id()` as `NotificationId::Portal(String)`
      — `PortalNotificationHandle` stores the generated ID string; `NotificationHandle::id()`
      returns `NotificationId::Portal(inner.id.clone().into())`.
- [x] `update()` on `PortalNotificationHandle` — implemented: `update_fallible()` calls
      `update_notification()` with the existing `id`, which the portal spec defines as a replace
      operation.

---

### Phase 3 — Signal Handling (Receiving)

`wait_for_action_signal_portal` in `zbus_rs.rs`:

- [x] Match rule registered for `ActionInvoked`
- [x] `ActionInvoked` message deserialized as `(String, String, Vec<Value>)` (id, action, parameters)
      — matches the frontend spec signature `ActionInvoked(id s, action s, parameter av)`
- [x] `NotificationClosed` match rule and `todo!()` handler removed — `org.freedesktop.portal.Notification`
      does not define this signal (it only exists on the backend impl interface).
- [x] `dbg!(parameters)` removed from the `ActionInvoked` handler.
- [ ] The `parameters` array in `ActionInvoked` contains the action target and an XDG Activation token.
      These are currently ignored; consider exposing the activation token for window focusing.

---

### Phase 4 — API Ergonomics & Correctness

- [x] ~~`server` feature in `default`~~ — already correct (`default = ["z"]`).
- [x] `memfd` and `nix` are now optional dependencies gated behind the `z` and `z-with-tokio`
      features in `Cargo.toml`. `dbus`-only users no longer pull them in. The decision was made
      **not** to introduce a separate `portal` feature — portal support is always present when
      `zbus` is active.
- [x] `eprintln!()` calls in `portal.rs` replaced with `log::debug!`.
- [x] `dbg!(path)` in `Icon::open()` removed; failures reported via `log::warn!`.
- [x] `copy_file_to_sealed_memfd` now returns `Result<File, Box<dyn std::error::Error>>` and
      propagates errors; `Icon::open` logs a warning on failure instead of panicking.
- [x] `Icon` type signature fixed — `#[zvariant(signature = "(sv)")]` produces the correct wire type.

---

### Phase 5 — `Priority` Cleanup

- [x] `impl Into<&str> for &Priority` replaced with `impl From<&Priority> for &'static str`
      — already the form used in `priority.rs`.
- [x] Manual `impl ToString for Priority` removed — `priority.rs` implements `Display` only;
      `ToString` is derived automatically by the compiler.

---

## Test Checklist

### Unit Tests (no live D-Bus required)

- [ ] `PortalNotification::from(&Notification)` maps `summary` → `title` and `body` → `body`
- [ ] `PortalNotification::from` extracts `Urgency::Low/Normal/Critical` from hints and converts to
      `Priority::Low/Normal/Urgent` correctly
- [ ] `PortalNotification::from` sets `icon` to `Themed([name])` when `.icon_named()` was called
- [ ] `PortalNotification::from` sets `icon` to `File(fd)` when `.image_path()` / `.icon_path()`
      was called (requires a real file on disk)
- [ ] `PortalNotification::from` sets `icon` to `None` when neither was called
- [ ] `Priority::from(Urgency::Critical)` → `Priority::Urgent` (not `Priority::High`)
- [ ] `Priority` serializes as the correct lowercase strings: `"low"`, `"normal"`, `"high"`, `"urgent"`
- [ ] `NotificationId::as_portal()` returns `None` for the `Global` variant and vice versa
- [ ] `NotificationId::as_global()` returns the correct `u32` for a `Global` variant
- [ ] `icon_named("foo")` and `icon("foo")` both result in `notification.icon == Some("foo")`
- [ ] `icon_path("some/path")` delegates to `image_path` and stores a `Hint::ImagePath`
- [ ] Auto-generated portal IDs are unique across consecutive calls (once ID generation is implemented)

### Integration Tests (require a portal-capable session bus)

- [ ] `show_via_portal` sends a notification and returns a handle without error
- [ ] Calling `handle.close()` sends `RemoveNotification` and the notification disappears
- [ ] Calling `handle.update()` re-sends `AddNotification` with the same ID and updates the visible
      notification
- [ ] `wait_for_action` resolves when an action button is clicked
- [ ] `on_close` resolves when the notification is dismissed by the user
- [ ] Using the same ID twice replaces the first notification (spec-defined update/replace behaviour)

### Regression Tests

- [x] `build_pattern` test in `tests/realworld.rs` — the incorrect `#[cfg(not(...))]` arm that
      compared `notification.icon` to a bare `"foo"` string has been removed. `icon` is
      `Option<String>` on all platforms; the assertion now uses `as_deref()` unconditionally.
- [ ] `examples/desktop-portal.rs` has no `[[example]]` entry in `Cargo.toml` — needs one with
    `required-features = ["async"]`.
</invoke>

---

## Blocking Issues Before Merge

| #   | Issue                                                                              | Location                                                     | Status   |
| --- | ---------------------------------------------------------------------------------- | ------------------------------------------------------------ | -------- |
| 1   | `app_id` missing from `AddNotification` D-Bus call                                 | `portal.rs` → `add_notification()`                           | ✅ Fixed |
| 2   | `Icon::Type` signature was `Signature::Bool` — wrong wire type (`(sv)` required)   | `portal.rs` → `icon` module → `impl Type for Icon`           | ✅ Fixed |
| 3   | `NotificationClosed` handler was `todo!()` — panics in production                  | `zbus_rs.rs` → `wait_for_action_signal_portal()`             | ✅ Fixed |
| 4   | `update()` was `todo!()` — panics if called on a portal handle                     | `handle.rs` → `PortalNotificationHandle::update_fallible()`  | ✅ Fixed |
| 5   | `dbg!()` and `eprintln!()` left in hot paths                                       | `portal.rs`, `zbus_rs.rs`                                    | ✅ Fixed |
| 6   | `memfd` and `nix` unconditionally pulled in on all Unix targets                    | `Cargo.toml`                                                 | ✅ Fixed |
| 7   | ~~`server` feature enabled in `default`~~ — **already fixed** (`default = ["z"]`)  | `Cargo.toml`                                                 | ✅ Fixed |
| 8   | Test assertion compares `Option<String>` to bare `"foo"` on non-Linux              | `tests/realworld.rs`                                         | ❌ Open  |
| 9   | Auto-generated portal ID not yet implemented; `show_via_portal` requires two args  | `notification.rs`, `examples/desktop-portal.rs`              | ❌ Open  |
| 10  | `actions` not converted in `PortalNotification::from` — buttons silently missing   | `portal.rs` → `PortalNotification::from`                     | ✅ Fixed |
| 11  | `dbus_rs.rs` passed `Option<String>` icon directly to `MessageItem` — compile fail | `xdg/dbus_rs.rs` → `send_notification_via_connection_at_bus` | ✅ Fixed |

---

## Pre-PR Checklist

- [ ] Write `///` doc comments for all new and modified public API items (`portal::Button`,
      `portal::Icon`, `portal::Priority`, `portal::NotificationHandle`,
      `Notification::show_via_portal()`)
- [ ] Add a usage example to the crate-level docs showing the `.show_via_portal()` path alongside
      the classic path
- [ ] Verify `examples/desktop-portal.rs` compiles and runs correctly end-to-end
- [ ] Register `examples/desktop-portal.rs` as a `[[example]]` entry in `Cargo.toml` with
      `required-features = ["async"]`
