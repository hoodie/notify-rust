# notify-rust Roadmap — 4.x → 5.0

This document plans the next two notify-rust releases.

- **5.0**: the primary focus. Unified, cross-platform `NotificationHandle` API.
  Drops legacy shims, removes the `"__closed"` sentinel, makes the new backends
  the default, gates the old ones behind explicit features.
- **4.18 (final 4.x)**: a conditional backport on a `4-x` continuation branch,
  done after 5.0-beta ships and only if community demand warrants it.

**Development order:** macOS 5.0 first → Windows 5.0 → cross-platform
reconciliation → 5.0-beta → (optional) 4.x backport + Linux
wayland/desktop-portal.

**`mac-usernotifications` versioning:** 0.1.0 targets the 4.18 backport;
0.2.0 potentially alongside 5.0. Exact cut points depend on what the 5.0 work
requires.

Progress tracking lives in [`notify-rust-progress.md`](./notify-rust-progress.md).

---

## Guiding principles

1. **4.18 must be a non-breaking minor on the default feature set.**
   Anything that would change a public signature on default cfg must be
   deferred to 5.0 or hidden behind an opt-in feature flag.
2. **Preview backends (`preview_macos_un`, `win32`) are opt-in only.**
   Enabling them is documented as "preview, semver-exempt within 4.x". The
   API shapes they expose are the ones we intend to ship in 5.0.
3. **5.0 unifies the handle API across all backends.** One `show()` shape,
   one `response()` model, one `id()` type.
4. **Legacy backends do not disappear in 5.0**, they move behind explicit
   feature flags so existing users can still pin to them deliberately.

---

## 4.18 — final 4.x release

### New features (additive only on default cfg)

| #   | Feature                                                                                                                                                                                              | Default? | Breaking?   | Notes                                                                                           |
| --- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- | ----------- | ----------------------------------------------------------------------------------------------- |
| F1  | Feature flag `preview_macos_un` (macOS, `UNUserNotificationCenter`)                                                                                                                            | no       | no (opt-in) | from `feature/macos-usernotifications`                                                          |
| F2  | Feature flag `win32` (Windows, `win32_notif`)                                                                                                                                                        | no       | no (opt-in) | from `feature/win32-notif`                                                                      |
| F3  | `ActionResponse`, `CloseReason`, `UserResponse` types in `notify_rust::action`                                                                                                                       | yes      | no          | new types, no signature changes                                                                 |
| F4  | `NotificationId` enum                                                                                                                                                                                | yes      | no          | new type; not yet returned from `id()` on default cfg                                           |
| F5  | `Notification::hero_image()` (Windows-only, additive, gated `cfg(target_os="windows")`)                                                                                                              | yes      | no          | only takes effect under `win32` feature; no-op otherwise                                        |
| F6  | XDG: `wait_for_action_response(&ActionResponse)` (additive, alongside existing `wait_for_action(&str)`)                                                                                              | yes      | no          | shipped as the migration target for 5.0                                                         |
| F7  | Deprecation warnings on `wait_for_action(&str)`, the `"__closed"` sentinel, and `on_close(closure)`                                                                                                  | yes      | no          | `#[deprecated]` only, still functional; `on_close` is superseded by `response_blocking()`      |
| F8  | Internal Windows polish from `windows_todo.md` that does not touch the public API (e.g. `Scenario::Urgent` runtime fallback, looping audio for `Alarm*`/`Call*`, `with_expiry` honoring ms timeouts) | yes      | no          | landed in the legacy `winrt-notification` path **and** in the new `win32` path where applicable |
| F9  | Docs: "preview backends" section in `README.md` and crate root, pointing users at the two new flags                                                                                                  | yes      | no          |                                                                                                 |
| F10 | Deprecation warnings on crate-root platform-specific free functions (`dbus_stack`, `get_capabilities`, `get_server_information`, `handle_action`, `set_application`, `get_bundle_identifier_or_default`, `request_auth`, `request_auth_blocking`, `check_bundle`) | yes | no | `#[deprecated]` only, pointing to the future `notify_rust::xdg` / `::macos` submodule paths |

### Explicitly **not** in 4.18 (deferred to 5.0)

| #   | Item                                                                                | Why deferred                                                      |
| --- | ----------------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| D1  | `show()` returning `Result<NotificationHandle>` on default macOS/Windows            | breaks the existing `Result<()>` signature                        |
| D2  | `NotificationHandle::id()` returning `NotificationId`                               | breaks the existing `u32` return type                             |
| D3  | Removal of `wait_for_action(&str)` and `"__closed"`                                 | breaks every existing call site                                   |
| D4  | Replacing `tauri-winrt-notification` with `win32_notif` as the Windows default      | the dep swap changes default-cfg behaviour and error types        |
| D5  | Making `preview_macos_un` the default macOS path                              | flips the macOS default cfg                                       |
| D6  | Unified `response().await` / `response_blocking()` on `NotificationHandle` as **the** primary interaction API | requires changing the handle's public surface across all backends |
| D7  | Moving `set_application` / `get_bundle_identifier_or_default` behind `macos_legacy` | feature-shuffle that breaks unconditional users                    |
| D8  | Creating `notify_rust::xdg`, `::macos`, `::windows` submodules and removing crate-root re-exports | structural break; 4.18 only adds `#[deprecated]` (F10) |

### Feature-flag layout in 4.18

```toml
[features]
default = ["z"]

# preview backends — semver-exempt within 4.x
preview_macos_un = []     # macOS UNUserNotificationCenter
win32                  = []     # Windows win32_notif

# existing flags unchanged
z = ["zbus", "serde", "async"]
# ...
```

The preview backends are mutually exclusive with their legacy counterparts on
the same platform (compile-time `cfg` switch).

### Ordering for 4.18

1. **Land the macOS feature flag** (`preview_macos_un`) by merging
   `feature/macos-usernotifications` to `main`, with the legacy macOS path
   restored as the no-flag default and `show()` reverted to `Result<()>` on
   the legacy path. The macOS spec's Task list applies only behind the flag.
2. **Land the Windows feature flag** (`win32`) by porting
   `feature/win32-notif` to live alongside the existing
   `winrt-notification` path, gated on `feature = "win32"`. Default Windows
   stays on `winrt-notification` and `show() -> Result<()>`.
3. **Land cross-platform `action` module** (`ActionResponse`, `CloseReason`,
   `UserResponse`, `NotificationId`) so 5.0 callers can already write code
   against the target types.
4. **Add XDG `wait_for_action_response`** as an additive method.
5. **Deprecate** `wait_for_action(&str)` and the `"__closed"` sentinel
   with `#[deprecated(since = "4.18.0", note = "…")]`. Do not remove.
6. **Internal Windows polish** from `windows_todo.md` (F8).
7. **Docs and examples** for the two new flags.

### `on_close` deprecation path

`NotificationHandle::on_close(closure)` is part of the existing public API and
cannot be removed in 4.18 without a breaking change. The plan:

- **4.18**: mark `on_close` with `#[deprecated(since = "4.18.0", note = "use response() / response_blocking() and match on UserResponse instead")]`. Still functional.
- **5.0**: remove it. `response().await` / `response_blocking()` are the
  replacement — they cover close, action, and reply events in one unified match.

`on_close` is a callback wrapper over `response_blocking()` that silently
discards action and reply events; it was never a good fit for the macOS UN
backend which delivers the full `UserResponse`.

### Risks for 4.18

- The macOS branch currently has `default = ["z", "preview_macos_un"]`
  in `Cargo.toml`. We must change that back so the default `cargo install`
  user keeps the legacy path.
- `feature/macos-usernotifications` changes `show()` to return
  `Result<NotificationHandle>` on the legacy path too. That has to be backed
  out for 4.18 and re-introduced in 5.0.
- The `mac-usernotifications` crate is currently a path dependency. It needs
  to be published before we can release 4.18.
- The macOS branch already touches XDG (`wait_for_action_response`, an
  internal `ActionResponse` rename). We need to confirm those changes are
  additive and do not break existing XDG callers.

---

## 5.0 — the unification release

### What we break (the full list)

| #   | API today (4.x)                                                           | API in 5.0                                                          | Migration                                                                                                             |
| --- | ------------------------------------------------------------------------- | ------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| B1  | `Notification::show() -> Result<()>` on macOS legacy                      | `Notification::show() -> Result<NotificationHandle>`                | drop `?;` semicolon, optionally inspect handle                                                                        |
| B2  | `Notification::show() -> Result<()>` on Windows                           | `Notification::show() -> Result<NotificationHandle>`                | same                                                                                                                  |
| B3  | `NotificationHandle::id() -> u32` (XDG)                                   | `NotificationHandle::id() -> NotificationId`                        | match on `NotificationId::Xdg(u32)` / `Mac(String)` / `Windows(String)`                                               |
| B4  | `wait_for_action<F: FnOnce(&str)>` (XDG + macOS)                          | removed                                                             | use `response_blocking()` and match on `UserResponse`                                                                 |
| B5  | `"__closed"` sentinel string                                              | removed                                                             | match `UserResponse::Closed(CloseReason)`                                                                             |
| B6  | `wait_for_action_response` (added in 4.18)                                | removed                                                             | superseded by `response()` / `response_blocking()`                                                                    |
| B7  | `on_close(handler)` (XDG, and macOS UN in 4.18)                           | removed                                                             | match `UserResponse::Closed`                                                                                          |
| B8  | macOS default backend = `mac-notification-sys` (legacy)                   | default = `UNUserNotificationCenter` (`preview_macos_un`)     | enable `macos_legacy` feature to opt back in                                                                          |
| B9  | Windows default backend = `tauri-winrt-notification`                      | default = `win32_notif`                                             | enable `windows_legacy` feature to opt back in                                                                        |
| B10 | `set_application`, `get_bundle_identifier_or_default` re-exports on macOS | only under `macos_legacy`                                           | feature-gate or migrate                                                                                               |
| B11 | Public re-export of `Urgency` on macOS (`#[deprecated]` today)            | removed                                                             | use `cfg(not(target_os = "macos"))`                                                                                   |
| B12 | `Notification::show_debug` (already `#[deprecated]`)                      | removed                                                             | use logging                                                                                                           |
| B13 | Feature flag name `preview_macos_un`                                | may be renamed to `macos_un` or dropped entirely (it's the default) | flag rename                                                                                                           |
| B14 | `Notification::action(id: &str, label: &str)`                             | `Notification::action(Action)` where `Action` is a typed builder    | replace `action(id, label)` with `action(Action::button(id, label))`; adopt `Action::reply(…)` for text-input actions |
| B15 | Platform-specific free functions at crate root (`dbus_stack`, `get_capabilities`, `get_server_information`, `set_application`, `get_bundle_identifier_or_default`, `request_auth`, `request_auth_blocking`, `check_bundle`) | moved to platform submodules: `notify_rust::xdg`, `notify_rust::macos`, `notify_rust::windows` | add submodule prefix, e.g. `notify_rust::get_capabilities()` → `notify_rust::xdg::get_capabilities()` |

The list of breaking changes is shorter than it looks because most callers
only use `.show().unwrap()`; the unwrap continues to compile. The two
practical pain points are `wait_for_action` callers and anyone who reads
`handle.id()`.

### Cross-platform ID opacity (5.0 goal)

`NotificationId` must be the single opaque type returned by `handle.id()` on
all three platforms. Today the enum has `Xdg(u32)` and `Mac(String)`; a
`Windows(String)` variant needs to be added for the `win32` backend. This
means callers never touch a raw `i32`, `u32`, or platform string directly.
`close(id)` and `update(id)` work the same way regardless of backend.

Required changes:

- Add `NotificationId::Windows(String)` variant.
- Return `NotificationId::Windows(tag)` from the `win32` `NotificationHandle::id()`.
- Make sure `close` / `update` on the Windows handle accept a `NotificationId` rather than a raw tag string.

### Unified `ActionResponse` across all platforms (5.0 goal)

`ActionResponse` (`Action(String)`, `Reply(String)`, `Closed(CloseReason)`) is
already defined in `src/action.rs` and is used by both XDG and macOS. The
Windows backend currently has no handle and therefore no response path at all.
In 5.0 it must use the same `ActionResponse` type so callers can write one
`match` arm regardless of platform.

Required changes:

- Windows `NotificationHandle::response()` / `response_blocking()` must return `ActionResponse`.
- Map Windows toast activation events to `ActionResponse::Action(key)` and dismiss events to `ActionResponse::Closed(CloseReason::Dismissed)` / `Expired` as appropriate.
- No platform-specific response type should appear in public API.

### Rich `Action` builder (5.0 goal)

`mac-usernotifications` models notification actions as a typed `Action` value
instead of bare `(id, label)` strings:

```rust
// plain button — available on all platforms
Action::button(id, label)

// inline text-input reply — macOS UN today, XDG and Windows stretch
Action::reply(id, label, button_title, placeholder)

// modifier — macOS-only; silently ignored elsewhere
action.requires_authentication()
```

`notify-rust` 5.0 adopts the same pattern. `Notification::action` changes
from `action(id: &str, label: &str)` to `action(Action)`. The migration is
mechanical: every existing `action("foo", "Bar")` becomes
`action(Action::button("foo", "Bar"))`.

`ActionResponse::Reply(String)` is already planned and maps directly to the
text the user typed in a reply action.

Platform support for 5.0:

| `Action` variant/modifier |    XDG     | macOS (UN) | Windows (win32) |
| ------------------------- | :--------: | :--------: | :-------------: |
| `button`                  |     ✅     |     ✅     |       ✅        |
| `reply`                   | 🟡 stretch |     ✅     |       ❌        |
| `requires_authentication` |     ❌     |     ✅     |       ❌        |

The 4.18 `preview_macos_un` preview path can already expose `Action`
as-is (it wraps `mac-usernotifications` directly). The legacy macOS path and
the `win32` preview path continue to use the string-pair form internally and
will be upgraded in 5.0.

### The 5.0 `NotificationHandle` (target shape)

```rust
impl NotificationHandle {
    pub fn id(&self) -> NotificationId;

    pub fn close(self);                         // XDG, macOS UN, Windows
    pub fn update(&mut self) -> Result<()>;     // XDG, macOS UN, Windows
    pub async fn update_async(&mut self) -> Result<()>;  // XDG (zbus), macOS UN

    pub async fn response(self) -> UserResponse;        // unified
    pub fn response_blocking(self) -> UserResponse;     // unified
}
```

`UserResponse` carries either an action identifier, a reply text, or a
`Closed(CloseReason)`. No closures, no sentinel strings, no platform-specific
shapes leaking into user code.

### Feature-flag layout in 5.0

```toml
[features]
default = ["z"]

# explicit legacy opt-ins
macos_legacy     = []   # mac-notification-sys / NSUserNotificationCenter
windows_legacy   = []   # tauri-winrt-notification

# existing flags unchanged
z = ["zbus", "serde", "async"]
```

The 4.18 preview flags (`preview_macos_un`, `win32`) disappear; their
behaviour becomes the default.

### Feature parity target for 5.0

Reading `windows_todo.md` and the current XDG/macOS surfaces, this is where we expect
to land. ✅ = supported, 🟡 = supported with caveats, ❌ = not available.

#### `Notification` builder

| method                             |    XDG     |      macOS (UN)       |   Windows (win32)    |
| ---------------------------------- | :--------: | :-------------------: | :------------------: |
| `appname`                          |     ✅     |          ❌           |          ❌          |
| `summary`                          |     ✅     |          ✅           |          ✅          |
| `subtitle`                         |     ❌     |          ✅           |          ✅          |
| `body`                             |     ✅     |          ✅           |          ✅          |
| `icon`                             |     ✅     |          ❌           | 🟡 app-logo override |
| `image_path`                       |     ✅     |          ✅           |          ✅          |
| `hero_image`                       |     ❌     |          ❌           |          ✅          |
| `auto_icon`                        |     ✅     |          ❌           |          ❌          |
| `hint`                             |     ✅     |          ❌           |          ❌          |
| `timeout`                          |     ✅     |          ✅           |     🟡 bucketed      |
| `urgency`                          |     ✅     | 🟡 interruption_level |   🟡 scenario map    |
| `interruption_level`               |     ❌     |          ✅           |          ❌          |
| `Action::button(id, label)`        |     ✅     |          ✅           |          ✅          |
| `Action::reply(…)`                 | 🟡 stretch |          ✅           |          ❌          |
| `action.requires_authentication()` |     ❌     |          ✅           |          ❌          |
| `id`                               |     ✅     |      ✅ (string)      |    🟡 (tag-based)    |
| `sound`                            |     🟡     |          ✅           |          ✅          |
| `thread_id`                        |     ❌     |          ✅           |          ❌          |
| `schedule_in`                      |     ❌     |          ✅           |          ❌          |
| `suppress_popup`                   |     ❌     |          ❌           |          ✅          |
| progress bar                       |     ❌     |          ❌           |     ✅ (stretch)     |

#### `NotificationHandle`

| method                    | XDG | macOS (UN) | Windows (win32) |
| ------------------------- | :-: | :--------: | :-------------: |
| `id`                      | ✅  |     ✅     |       ✅        |
| `close`                   | ✅  |     ✅     |       ✅        |
| `update` / `update_async` | ✅  |     ✅     |       ✅        |
| `response().await`        | ✅  |     ✅     |       ✅        |
| `response_blocking()`     | ✅  |     ✅     |       ✅        |

Net: **the handle API is fully unified across all three backends in 5.0.** There
is no `on_close` callback — callers pattern-match on the `UserResponse` returned
by `response()` / `response_blocking()` instead.
The builder is not: `hint`, `urgency`, `appname`, and `auto_icon` stay
XDG-exclusive because the underlying systems do not model them; `subtitle`,
`thread_id`, `schedule_in`, `interruption_level`, `Action::reply`, and `requires_authentication` stay macOS-exclusive (or macOS-primary);
`hero_image`, `suppress_popup`, and progress stay Windows-exclusive. Those
remain `cfg`-gated.

### Platform submodules (5.0 goal)

All platform-specific free functions move out of the crate root into dedicated
submodules. The crate root only exposes types and functions that work on every
platform.

```
notify_rust::xdg::get_capabilities()          // was notify_rust::get_capabilities()
notify_rust::xdg::get_server_information()    // was notify_rust::get_server_information()
notify_rust::xdg::dbus_stack()               // was notify_rust::dbus_stack()
notify_rust::xdg::handle_action(…)           // was notify_rust::handle_action(…)

notify_rust::macos::request_auth()           // was notify_rust::request_auth()
notify_rust::macos::request_auth_blocking()  // was notify_rust::request_auth_blocking()
notify_rust::macos::check_bundle()           // was notify_rust::check_bundle()

// macos_legacy only:
notify_rust::macos::set_application(…)                  // was notify_rust::set_application()
notify_rust::macos::get_bundle_identifier_or_default(…)  // was notify_rust::get_bundle_identifier_or_default()
```

Each submodule is `cfg`-gated to its platform so it doesn't appear in
docs or completions on other targets. The 4.18 crate-root re-exports get a
`#[deprecated]` pointing to the new path; the old symbols are removed in 5.0.

### Ordering for 5.0

1. Cut a 5.0 development branch from 4.18.
2. Flip the macOS default to `UNUserNotificationCenter`, gate the legacy
   path behind `macos_legacy`.
3. Flip the Windows default to `win32_notif`, gate `tauri-winrt-notification`
   behind `windows_legacy`.
4. Unify `NotificationHandle::id()` to `NotificationId`.
5. Remove `wait_for_action(&str)`, `wait_for_action_response`, `on_close`,
   and the `"__closed"` sentinel.
6. Land `response()` / `response_blocking()` on every handle.
7. Migrate `Notification::action(id, label)` to `Notification::action(Action)` across all backends; expose `Action::reply` on macOS UN and (stretch) XDG.
8. Move platform-specific free functions into `xdg`, `macos`, `windows` submodules; remove crate-root re-exports.
9. Remove the `Urgency` re-export on macOS and `show_debug`.
10. Publish a 5.0-rc on crates.io alongside the final 4.18 so users have
    both as installable references.
11. After at least one rc cycle with feedback, publish 5.0.

---

## Open questions

These need a decision before work starts on 4.18:

1. **Preview-flag names.** The macOS branch uses `preview_macos_un`;
   the Windows branch has no flag yet. Are we happy with
   `preview_macos_un` and `win32` as the 4.18 names, or do you want a
   uniform scheme like `preview-macos-un` / `preview-windows-win32` (or an
   umbrella `preview-backends`)?
2. **Should 4.18 also publish an `experimental` umbrella feature** that
   enables both preview backends at once, for CI convenience?
3. **Do we deprecate `wait_for_action(&str)` and `"__closed"` in 4.18**
   (my recommendation: yes, with `#[deprecated]`), or leave them silent
   until 5.0?
4. **`mac-usernotifications` publication.** The macOS branch currently
   pulls it as a path dependency. What is the timeline for publishing it to
   crates.io so 4.18 can ship?
5. **Windows dep swap.** Do we tolerate carrying both `tauri-winrt-notification`
   and `win32_notif` in `Cargo.toml` for the duration of 4.18, or do we want
   the `win32` feature to _replace_ `tauri-winrt-notification` (which would
   force every Windows user onto the new dep right away)? My recommendation
   is to keep both, since the whole point of 4.18 is non-breaking opt-in.
6. **`NotificationId` in 4.18.** The macOS branch already returns
   `NotificationId` from `handle.id()`, which is a 4.x break. Confirm that
   we are reverting `id()` back to `u32` on default-cfg XDG for 4.18 and
   only exposing `NotificationId` under the preview flags.
7. **XDG `ActionResponse` shape.** The macOS branch reworked the internal
   XDG `ActionResponse` enum. Does the public re-export of
   `ActionResponse` need to stay byte-compatible with 4.17, or can we use
   4.18 to introduce the new shape (since it is a new public type either
   way)?
8. **MSRV.** Are we comfortable bumping MSRV in 5.0 (current is 1.63)? The
   `win32_notif` dep may push that up.
9. **Urgency on macOS.** It is `#[deprecated]` today. Remove in 5.0 (my
   recommendation), or keep as a no-op?
10. **`set_application` / `get_bundle_identifier_or_default`.** Confirm
    that these only make sense under `macos_legacy` in 5.0 and can be
    feature-gated.
11. **Timeline.** Target dates for 4.18 and 5.0-rc1? Are they to ship side
    by side, as the macOS branch notes already suggest?
12. **`Notification::show_debug`.** Already `#[deprecated]`. Remove in 5.0?
13. **Should the unified handle's `response()` consume `self`** (force
    one-shot, my recommendation, matches macOS UN today) or take `&self`
    (would require interior state and complicates Drop semantics)?
14. **`Action` API in 4.18 preview.** The `preview_macos_un` preview path in 4.18 wraps `mac-usernotifications` directly and can already expose the typed `Action` builder (`Action::button`, `Action::reply`, `requires_authentication`). Should it do so, even though the 4.18 default-cfg path still uses the old `action(id, label)` strings? This would give early adopters the target API a release early, but creates a temporary inconsistency between the preview and default paths.
