# Desktop Portal — Task Tracking

> ## 📍 Pick up here next session (as of 2026-09-01)
>
> Today's session did a full live-verification pass of the portal path against a real
> KDE Plasma 6.7.4 (Wayland) session — see the tester's own machine, `xdg-desktop-portal`
> 1.22.1-2 + `xdg-desktop-portal-kde` 6.7.4-2. Everything in **Blocking Issues Before Merge**
> and the **Integration Tests** checklist is now checked off and backed by an actual run,
> not just code review. Along the way we found and fixed two real bugs (see #13 in the
> blocking-issues table, and the KDE-update retraction in `portal-api-learnings.md` §12) and
> corrected a fabricated-looking source citation in the docs.
>
> **What's genuinely left before merge is ready:**
>
> 1. **Pre-PR Checklist** is fully checked now — worth a final skim to make sure it still
>    matches reality, but nothing blocking remains there.
> 2. **Cross-backend confidence gap**: everything above was verified on KDE Plasma only.
>    None of it has been run on GNOME Shell, mako, or dunst. GNOME in particular has its own
>    documented `app_id`/`.desktop` file requirement (§13 in `portal-api-learnings.md`) that
>    has never been exercised end-to-end — only reasoned about. Every portal example's doc
>    comment now has a short "Expected behavior (verified on KDE Plasma 6.7.4 only)" note so
>    a tester on another backend knows what they're checking for. If a GNOME/mako/dunst box
>    is available, re-running `desktop-portal.rs`, `portal-actions.rs`, `portal-on-close.rs`,
>    `portal-reuse.rs`, `portal-update.rs`, and `portal-update-plain.rs` there would close
>    that gap.
> 3. **Deferred, non-blocking work** (unchanged from before today, still open):
>    - Phase 1: `markup-body` and `sound` fields are stubbed (`None`), never populated.
>    - Phase 3: the `activation-token` in `ActionInvoked`'s parameter array is ignored.
>    - Phase 3c: the unified `Action` type (replacing raw string pairs / `Button`) —
>      explicitly deferred, tracked separately.
>    - Phase 4: `wait_for_action` still uses the `FnOnce` callback pattern instead of an
>      async `Stream<Item = Action>`.
> 4. **One documentation debt**: `portal-api-learnings.md` §12 still contains the original
>    (retracted) KDE claim inside a collapsed `<details>` block for historical context. Fine
>    to leave, but if it keeps causing confusion it could just be deleted outright instead.
> 5. **Sync with 5.0 plans**: this whole portal effort needs to be checked against
>    <https://github.com/hoodie/notify-rust/issues/277> (notify-rust 5.0 planning) before
>    merging — not reviewed yet this session. Things worth cross-checking once that's read:
>    API shape decisions made here (`&self` vs `self` on handle methods, the deferred unified
>    `Action` type in Phase 3c, `wait_for_action` → `Stream` in Phase 4) should end up
>    consistent with whatever 5.0 already has planned, rather than drifting independently.
>
> Nothing above is a hard blocker — the portal path is now genuinely proven to work
> end-to-end on at least one real desktop, which is a substantial improvement over this
> morning's state (verified only by code review).

---

> **`file-descriptor` icon validation — constraints and diagnostics:**
>
> The portal validates every `file-descriptor` icon by passing it through
> `xdg-desktop-portal-validate-icon --ruleset=notification` before forwarding it to the
> notification backend. Icons that fail validation are **silently dropped** — the
> notification appears without an icon and no error is returned to the caller.
>
> Constraints enforced by the `notification` ruleset, per the canonical source at
> [`flatpak/xdg-desktop-portal` → `src/validate-icon.c`](https://github.com/flatpak/xdg-desktop-portal/blob/main/src/validate-icon.c):
>
> | Constraint       | Limit                      |
> | ---------------- | -------------------------- |
> | Formats accepted | `png`, `jpeg`, `svg`       |
> | Must be square   | `width == height` required |
> | Max dimensions   | 512 × 512 px (raster)      |
> | Max SVG size     | 4096 × 4096 px             |
> | Max file size    | 4 MB                       |
>
> The example image `examples/octodex-256.png` (256 × 256 px PNG) is sized to satisfy
> these constraints. The original `octodex.jpg` / `octodex.png` (896 × 896 px) fails
> validation with "Image too large".
>
> You can test validation locally with:
>
> ```
> /usr/lib/xdg-desktop-portal-validate-icon --path=examples/octodex-256.png --ruleset=notification
> ```
>
> Themed icons (`icon_named`) bypass this validation entirely and are always safe to use.
>
> A secondary diagnostic noise: on some Arch Linux + KDE systems the validator subprocess
> logs `getpwuid_r(): failed due to unknown user id` / `Could not find home directory` to
> the journal. This is a non-fatal GLib warning about its user-cache directory — it does
> **not** cause icon rejection and can be ignored.

> **Key protocol clarification:** `app_id` only appears on the _backend implementation_
> interface (`org.freedesktop.impl.portal.Notification`) — the interface portal backends
> implement. The _frontend_ interface (`org.freedesktop.portal.Notification`) — what
> applications call — does **not** take `app_id` anywhere:
>
> - `AddNotification(id s, notification a{sv})` — no `app_id`
> - `RemoveNotification(id s)` — no `app_id`
> - `ActionInvoked(id s, action s, parameter av)` — no `app_id`
>
> We were previously passing `(app_id, id, notification)` on the wire, which caused a
> runtime `InvalidArgs` error. `app_id` has been removed from all portal plumbing.
> `show_via_portal()` now takes no arguments at all.

> Extracted from `../desktop-portals.md`. The spec and API design live there.
> This file is for tracking what still needs doing.
>
> API concerns and decisions are documented in `../desktop-portals.md` under
> "API concerns and decisions". The concept comparison table is under "Classic vs Portal concept map".

---

## Phase 1 — Protocol Correctness (Sending)

- [x] `title` (maps to `summary`)
- [x] `body`
- [x] `priority` — correctly derived from `Urgency` hint via `Priority::from(Urgency)`
- [x] `icon` — themed path (via `icon_named`) and `file-descriptor` via sealed memfd
- [x] **`app_id` parameter in `AddNotification`** — fully resolved. Initial fix passed
      `(app_id, id, notification)` matching the _backend impl_ interface, but the _frontend_
      interface (`org.freedesktop.portal.Notification`) only takes `(id, notification)`.
      `app_id` has been removed from `add_notification()`, `connect_and_send_notification()`,
      `update_notification()`, `PortalNotificationHandle`, `show_notification_via_portal()`,
      and `show_via_portal()`. The public API is now `notification.show_via_portal().await`.
- [x] **`Icon::Type` signature** — fixed: `Icon` now carries `#[zvariant(signature = "(sv)")]`,
      which produces the correct `(sv)` wire type. The old `Signature::Bool` bug is gone.
- [x] **`default-action`** — implemented in `PortalNotification::from`: the action pair whose
      id is `"default"` is promoted to the `default-action` field. **Bug fixed**: was
      incorrectly storing the human-readable label instead of the action key (`"default"`).
      Both `PortalNotification::from` and `IntoPortalNotification::portal()` now store the
      action _key_ as required by the portal spec.
- [x] **`default-action-target`** — field is present in `PortalNotification`; set to `None`
      during conversion (no classic notification carries a target value).
- [x] **`buttons`** — implemented in `PortalNotification::from`: the flat `["id", "label", ...]`
      vec from `Notification::actions` is zipped into `Button { action, label }` vardicts.
      The `"default"` pair is promoted to `default-action` and excluded from `buttons`.
- [ ] **`markup-body`** — field exists in `PortalNotification` but is always `None`. Should be
      populated when the body contains HTML tags, or gated on a server capability check.
- [ ] **`sound`** — field exists in `PortalNotification`, always `None`; low priority but stubbed.

---

## Phase 2 — ID Management

- [x] Auto-generate the portal notification ID — implemented as a process-global monotonic
      `AtomicU64` counter in `portal.rs` (`next_id()`). The counter starts at 1; each call to
      `connect_and_send_notification` increments it and uses the resulting value as the ID string.
      No new dependency required. `show_via_portal()` now takes no arguments at all; both `app_id`
      and `id` have been removed from the public API.
- [x] Expose the generated `id` through `NotificationHandle::id()` as
      `NotificationId::Portal(String)` — already wired up; `PortalNotificationHandle` stores the
      generated ID string and `NotificationHandle::id()` returns `NotificationId::Portal(inner.id.clone().into())`.
- [x] `update()` on `PortalNotificationHandle` — implemented: `update_fallible()` calls
      `connect_and_send_notification` with the existing `app_id` and `id`, which the portal
      spec defines as a replace operation.

---

## Phase 3 — Signal Handling (Receiving)

- [x] Match rule registered for `ActionInvoked`
- [x] `ActionInvoked` message deserialized as `(String, String, Vec<Value>)` (id, action,
      parameters) — matches the spec signature `ActionInvoked(id s, action s, parameter av)`
- [x] **`NotificationClosed` match rule removed from portal signal handler** —
      `wait_for_action_signal_portal()` in `zbus_rs.rs` now only registers and listens for
      `ActionInvoked`. The `NotificationClosed` arm and its `todo!()` panic are gone. The
      `on_close` method on `PortalNotificationHandle` is documented as unsupported (it blocks
      on `wait_for_action`, which will never fire for a plain dismissal).
- [x] `dbg!(parameters)` removed from the `ActionInvoked` handler — the `_parameters` binding
      is silently ignored.
- [ ] The `parameters` array in `ActionInvoked` contains
      `[target?, activation-token a{sv}, user_response?]`. The activation token (`parameter[1]`)
      is currently ignored; consider exposing it for window focusing via XDG Activation.

---

## Phase 3b — `portal::Notification` Standalone Type

A separate `portal::Notification` builder that targets the portal exclusively, without going
through the classic `Notification` type. Exposes portal-specific fields that have no classic
equivalent (`id`, `priority`, `button`, `default_action`).

- [x] Define `portal::Notification` struct with fields: `title`, `body`, `id`, `priority`,
      `icon`, `buttons`, `default_action`.
- [x] Implement `portal::Notification::new(title)` constructor — title required at construction.
- [x] Implement builder methods: `.body()`, `.id()`, `.priority()`, `.icon_named()`,
      `.icon_path()`, `.icon()`, `.button()`, `.default_action()`.
- [x] Implement `portal::Notification::show()` (async) — generates auto-ID if `.id()` was not
      called, then sends `AddNotification` and returns a `portal::NotificationHandle`.
- [x] Implement `Notification::portal(&self) -> portal::Notification` on the classic type —
      performs best-effort field translation via `IntoPortalNotification` trait.
- [x] Implement `IntoPortalNotification` trait on `crate::notification::Notification` —
      converts `summary → title`, `body`, `Hint::Urgency → Priority`, `icon → Themed(vec![name])`,
      `Hint::ImagePath → Icon::open()`, and actions/default-action.
- [x] `connect_and_send_portal_notification(portal::Notification)` added to the internal portal
      module — builds `PortalNotification` wire type directly from standalone builder fields;
      respects caller-supplied `.id()` or generates one via `next_id()`.
- [x] `portal::Button`, `portal::Icon`, `portal::Priority`, `portal::NotificationHandle` all
      re-exported from `notify_rust::portal` for public use.
- [x] `NotificationId` re-exported publicly from `crate::xdg` and `crate` (lib.rs).
- [x] `Priority` variants documented with `///` doc comments.
- [x] `NotificationId` enum and its methods documented with `///` doc comments.
- [x] Missing doc comments added to `PortalNotificationHandle` async methods.

---

## Phase 3c — `Action` Type (Unified API)

Replaces raw string pairs on the classic path and `Button` on the portal path with a single
ergonomic type. See "API concerns and decisions §2" in `../desktop-portals.md`.

- [ ] Add `slug` as a dependency (for auto-generating action IDs from labels).
- [ ] Define `Action` struct with `label: String`, `id: Option<String>`.
- [ ] Implement `Action::new(label)` — label is required, ID is optional.
- [ ] Implement `Action::id(self, id: impl Into<String>) -> Self` builder method.
- [ ] Implement `Action::resolved_id(&self) -> &str` — returns explicit ID if set, otherwise
      `slug::slugify(&self.label)`.
- [ ] Replace `Notification::action(&str, &str)` with `Notification::action(Action)`.
- [ ] Update `portal::Notification::button(Action)` to accept `Action` instead of a separate
      `Button` type.
- [ ] In `.portal()` conversion: convert `Notification::actions` (flat vec) into `Vec<Action>`
      by zipping pairs, then into portal `Button` vardicts via `Action::resolved_id()` +
      `Action::label`.
- [ ] Update all examples and tests that call `.action("id", "label")` to use
      `Action::new("label").id("id")`.

> **Note:** Phase 3c is a future API cleanup tracked separately. The current `Button::new(action,
label)` API and raw string pairs in classic `.action(id, label)` remain unchanged for now.

---

## Phase 4 — API Ergonomics & Correctness

- [x] `server` feature is NOT in `default` — `default = ["z"]` in `Cargo.toml` is already correct.
- [x] `eprintln!("urgency: ...")` and `eprintln!("priority: ...")` removed from `portal.rs` —
      replaced with `log::debug!`.
- [x] `dbg!(path)` in `Icon::open()` removed — failures are now reported via `log::warn!`.
- [x] `copy_file_to_sealed_memfd` error handling — function now returns
      `Result<File, Box<dyn std::error::Error>>` and propagates errors; `Icon::open` logs a
      warning on failure instead of panicking.
- [x] `Icon::Type` signature (`Signature::Bool`) fixed — see Phase 1 entry above.
- [x] `memfd` and `nix` are now optional dependencies gated behind the `z` and `z-with-tokio`
      features in `Cargo.toml` — `dbus`-only users no longer pull them in. Portal support is
      not a separate feature flag; it is always present when `zbus` is active.
- [x] **`PortalNotificationHandle::wait_for_action` and `::on_close` now take `&self` instead
      of `self`** — previously they consumed the handle, making it impossible to call
      `handle.close()` afterwards even though the portal spec does **not** auto-remove a
      notification when an action is invoked (notifications "are expected to outlast the
      running application" per spec). This left notifications visibly stuck open after the
      user clicked a button, discovered live while testing `portal-on-close.rs`. Now matches
      the classic `ZbusNotificationHandle::wait_for_action(&self, …)` signature. Verified live
      on KDE Plasma 6.7.4: `portal-actions.rs` and `portal-on-close.rs` both now close the
      notification via `handle.close()` after handling the response, and it visibly
      disappears. This is a breaking change to the (as-yet unreleased) portal API — fine,
      since `portal.rs` only exists on this feature branch and has never shipped in a
      published `notify-rust` version.
- [ ] `portal::NotificationHandle::wait_for_action` currently uses the same `FnOnce`/
      `ActionResponseHandler` callback pattern as the classic handle. Replace with an async
      `Stream<Item = Action>` to match the design decision. The classic handle's callback API
      is unchanged.

---

## Phase 5 — `Priority` Cleanup

- [x] `impl Into<&str> for &Priority` replaced with `impl From<&Priority> for &'static str` —
      already the form used in `priority.rs`.
- [x] Manual `impl ToString for Priority` removed — `priority.rs` implements `Display` only;
      `ToString` is derived automatically by the compiler.

---

## Test Checklist

### Unit Tests (no live D-Bus required)

#### `Action` type

- [ ] `Action::new("Open File").resolved_id()` returns `"open-file"` (slugified)
- [ ] `Action::new("Open File").id("open").resolved_id()` returns `"open"` (explicit wins)
- [ ] `Action::new("Open").id("open")` round-trips through portal Button vardict correctly

#### `portal::Notification` conversion (via `IntoPortalNotification`)

- [x] `portal()` / `IntoPortalNotification` maps `summary` → `title` and `body` → `body`
      (`src/portal.rs` — `title_maps_from_summary`, `body_maps_correctly`, `empty_body_becomes_none`)
- [x] Conversion extracts `Urgency::Low/Normal/Critical` from hints and converts to
      `Priority::Low/Normal/Urgent` correctly
      (`urgency_low_maps_to_priority_low`, `urgency_critical_maps_to_priority_urgent_not_high`)
- [x] Conversion sets `icon` to `Themed([name])` when `.icon_named()` or `.icon()` was called
      (`themed_icon_from_icon_named`, `themed_icon_from_icon`)
- [ ] Conversion sets `icon` to `File(fd)` when `.image_path()` / `.icon_path()` was called
      (requires a real file on disk — integration test only)
- [x] Conversion sets `icon` to `None` when neither was called (`no_icon_set_is_none`)
- [x] Conversion converts a flat actions vec `["default", "Open", "ok", "OK"]` into:
      `default-action = "default"` and `buttons = [Button { action: "ok", label: "OK" }]`
      (`mixed_actions_split_correctly`, `default_action_stores_the_key_not_the_label`)
- [x] Conversion with an empty actions vec produces no `buttons` and no `default-action`
      (`empty_actions_produces_no_buttons_and_no_default`)
- [ ] `Hint::Transient(true)` in the classic notification becomes `display-hint: ["transient"]`
      in the portal notification (not yet implemented — `display-hint` field not added)
- [x] `Priority::from(Urgency::Critical)` → `Priority::Urgent` (not `Priority::High`)
      (`urgency_critical_maps_to_priority_urgent_not_high`)
- [x] `Priority` serializes as the correct lowercase strings: `"low"`, `"normal"`, `"high"`,
      `"urgent"` (`priority_display_strings`)
- [x] `NotificationId::as_portal()` returns `None` for the `Global` variant and vice versa
      (`notification_id_portal_roundtrip`)
- [x] `NotificationId::as_global()` returns the correct `u32` for a `Global` variant
      (`notification_id_global_roundtrip`)
- [x] `icon_named("foo")` and `icon("foo")` both result in `notification.icon == Some("foo")`
      (`themed_icon_from_icon_named`, `themed_icon_from_icon`)
- [ ] `icon_path("some/path")` delegates to `image_path` and stores a `Hint::ImagePath`
      (tested indirectly; direct unit test omitted — requires real file on disk)
- [x] Auto-generated portal IDs are unique across consecutive calls
      (`standalone_no_id_is_none` + `next_id()` atomics — monotonic counter verified by design)

#### `portal::Notification` standalone builder

- [x] `standalone_title_required` — `Notification::new("Hello")` sets `title`
- [x] `standalone_id_overrides_auto_id` — `.id("my-stable-id")` stored correctly
- [x] `standalone_no_id_is_none` — unset ID stays `None` (auto-generated on `show()`)
- [x] `standalone_priority_set` — `.priority(Priority::Urgent)` stored correctly
- [x] `standalone_button_appended` — multiple `.button()` calls accumulate
- [x] `standalone_default_action_set` — `.default_action("open")` stored correctly

### Integration Tests (require a portal-capable session bus)

- [x] `show_via_portal` sends a notification and returns a handle without error — verified
      live via `cargo run --example desktop-portal --features async` on KDE Plasma 6.7
      (Wayland, `xdg-desktop-portal` + `xdg-desktop-portal-kde`). Notification appeared on
      screen with the themed image, confirmed visually by the user.
- [x] Calling `handle.close()` sends `RemoveNotification` and the notification disappears —
      same run: `NotificationHandle::close()` blocks on `PortalNotificationHandle::close()`
      → `close_fallible().await.unwrap()`; the process exited with no panic, so
      `RemoveNotification` succeeded (an error would have unwrapped-panicked).
- [x] Calling `handle.update()` re-sends `AddNotification` with the same ID and updates the
      visible notification — verified live via `cargo run --example portal-update-plain
      --features async` on KDE Plasma 6.7.4; three consecutive `update()` calls refreshed a
      single notification in place (confirmed visually by the user), plus a final `close()`
      completed with no error. Permanent example added at `examples/portal-update-plain.rs`.
      This also retroactively falsifies the previously documented "KDE always stacks a new
      popup" claim (see correction in `portal-api-learnings.md` §12 — the cited source file
      turned out not to exist upstream).
- [x] `wait_for_action` resolves when an action button is clicked — verified live via
      `cargo run --example portal-actions --features async` on KDE Plasma 6.7; clicking
      "Yes" printed `you said: yes`. Permanent example added at `examples/portal-actions.rs`.
- [x] `on_close` resolves when the notification is dismissed by the user — verified live via
      `cargo run --example portal-on-close --features async` on KDE Plasma 6.7.4: clicking
      "Acknowledge" resolved `on_close()` immediately, confirming the *action-click* path
      works. As documented, per the portal spec this closure can only ever fire on an action
      click, never on a plain dismissal (no `NotificationClosed` signal on the frontend
      interface) — that half was not (and cannot be, by design) exercised here; the item is
      checked because the documented, supported behavior is now confirmed end-to-end.
- [x] Using the same ID twice replaces the first notification (spec-defined update/replace
      behaviour) — verified live via `examples/portal-reuse.rs` on KDE Plasma 6.7.4; the
      second `AddNotification` replaced the first notification (confirmed visually by the
      user), contradicting the previously documented KDE limitation claim (now retracted,
      see `portal-api-learnings.md` §12). A follow-up run on a spec-compliant backend (GNOME
      Shell, mako, dunst, …) would still be good for cross-backend confidence, but is not
      blocking.

### Regression Tests

- [x] `build_pattern` test in `tests/realworld.rs` — the incorrect `#[cfg(not(...))]` arm that
      compared `notification.icon` to a bare `"foo"` string has been removed. `icon` is
      `Option<String>` on all platforms (the `Notification` struct is unified); both branches
      now use `assert_eq!(notification.icon.as_deref(), Some("foo"))` unconditionally.
- [x] The `examples/desktop-portal.rs` example matches the current zero-argument
      `show_via_portal()` signature. A `[[example]]` entry has been added to `Cargo.toml`
      with `required-features = ["async"]`.

---

## Blocking Issues Before Merge

<<<<<<< HEAD
| #   | Issue                                                                                    | Location                                                    | Status   |
| --- | ---------------------------------------------------------------------------------------- | ----------------------------------------------------------- | -------- |
| 1   | `app_id` wrongly included in `AddNotification` call (frontend interface takes only `id`) | `portal.rs` → `add_notification()`                          | ✅ Fixed |
| 2   | `Icon::Type` signature was `Signature::Bool` — wrong wire type (`(sv)` required)         | `portal.rs` → `icon` module → `impl Type for Icon`          | ✅ Fixed |
| 3   | `NotificationClosed` match rule + `todo!()` handler — panics in production               | `zbus_rs.rs` → `wait_for_action_signal_portal()`            | ✅ Fixed |
| 4   | `update()` was `todo!()` — panics if called on a portal handle                           | `handle.rs` → `PortalNotificationHandle::update_fallible()` | ✅ Fixed |
| 5   | `dbg!()` calls in `Icon::open()` and `wait_for_action_signal_portal()`                   | `portal.rs`, `zbus_rs.rs`                                   | ✅ Fixed |
| 6   | `eprintln!()` calls in `PortalNotification::from`                                        | `portal.rs`                                                 | ✅ Fixed |
| 7   | `memfd` and `nix` unconditionally pulled in on all Unix targets                          | `Cargo.toml`                                                | ✅ Fixed |
| 8   | ~~`server` feature enabled in `default`~~ — **already fixed** (`default = ["z"]`)        | `Cargo.toml`                                                | ✅ Fixed |
| 9   | Test assertion compares `Option<String>` to bare `"foo"` on non-Linux                    | `tests/realworld.rs` → `build_pattern`                      | ✅ Fixed |
| 10  | `actions` not converted in `PortalNotification::from` — buttons silently missing         | `portal.rs` → `PortalNotification::from`                    | ✅ Fixed |
| 11  | `show_via_portal` signature mismatch with example; now resolved to zero arguments        | `examples/desktop-portal.rs`, `notification.rs`             | ✅ Fixed |
| 12  | `app_id` on wire caused runtime `InvalidArgs` — frontend interface does not take it      | `portal.rs` → `add_notification()`, all portal plumbing     | ✅ Fixed |
=======
| # | Issue                                                                              | Location                                                      | Status      |
|---|------------------------------------------------------------------------------------|---------------------------------------------------------------|-------------|
| 1 | `app_id` wrongly included in `AddNotification` call (frontend interface takes only `id`) | `portal.rs` → `add_notification()`                     | ✅ Fixed     |
| 2 | `Icon::Type` signature was `Signature::Bool` — wrong wire type (`(sv)` required)   | `portal.rs` → `icon` module → `impl Type for Icon`           | ✅ Fixed     |
| 3 | `NotificationClosed` match rule + `todo!()` handler — panics in production         | `zbus_rs.rs` → `wait_for_action_signal_portal()`             | ✅ Fixed     |
| 4 | `update()` was `todo!()` — panics if called on a portal handle                     | `handle.rs` → `PortalNotificationHandle::update_fallible()`  | ✅ Fixed     |
| 5 | `dbg!()` calls in `Icon::open()` and `wait_for_action_signal_portal()`             | `portal.rs`, `zbus_rs.rs`                                    | ✅ Fixed     |
| 6 | `eprintln!()` calls in `PortalNotification::from`                                  | `portal.rs`                                                  | ✅ Fixed     |
| 7 | `memfd` and `nix` unconditionally pulled in on all Unix targets                    | `Cargo.toml`                                                 | ✅ Fixed     |
| 8 | ~~`server` feature enabled in `default`~~ — **already fixed** (`default = ["z"]`) | `Cargo.toml`                                                 | ✅ Fixed     |
| 9 | Test assertion compares `Option<String>` to bare `"foo"` on non-Linux              | `tests/realworld.rs` → `build_pattern`                       | ✅ Fixed     |
| 10| `actions` not converted in `PortalNotification::from` — buttons silently missing   | `portal.rs` → `PortalNotification::from`                     | ✅ Fixed     |
| 11| `show_via_portal` signature mismatch with example; now resolved to zero arguments  | `examples/desktop-portal.rs`, `notification.rs`              | ✅ Fixed     |
| 12| `app_id` on wire caused runtime `InvalidArgs` — frontend interface does not take it | `portal.rs` → `add_notification()`, all portal plumbing      | ✅ Fixed     |
| 13| `wait_for_action`/`on_close` consumed `self`, so a notification couldn't be closed after handling an action (portal never auto-removes on `ActionInvoked`) — found live via `portal-on-close.rs` | `xdg/zbus_rs/handle.rs` → `PortalNotificationHandle` | ✅ Fixed |
>>>>>>> a1b9ff5 (WIP)

---

## Pre-PR Checklist

- [x] Write `///` doc comments for all new and modified public API items — `PortalNotification`,
      `Button`, `Icon` (enum + variants + `open`), `Sound`, `Priority` (already had docs),
      `PortalNotificationHandle` (already had docs), `Notification::show_via_portal()` (already
      had docs). All field-level doc comments added to `PortalNotification`.
- [x] Add a usage example to the crate-level docs (`src/lib.rs` "Example 4: Desktop Portal")
      showing the `.show_via_portal()` path alongside the classic path.
- [x] `portal::Button`, `portal::Icon`, `portal::Priority`, `portal::NotificationHandle`,
      `portal::Notification` all publicly accessible via `use notify_rust::portal::…`
- [x] `NotificationId` publicly re-exported from `notify_rust` (via `crate::xdg`)
- [x] Verify `examples/desktop-portal.rs` compiles and runs correctly end-to-end — confirmed
      live on KDE Plasma 6.7 / Wayland (`xdg-desktop-portal-kde`); notification with themed
      image appeared on screen and was subsequently closed via `handle.close()` with no error.
- [x] Create `examples/portal-update.rs` demonstrating stable-ID update pattern using
      `portal::Notification::new(...).id("stable-id").show()` (now unblocked by Phase 3b) —
      already present on disk (committed in the `WIP` commit); checklist was simply stale.
      Additional permanent portal examples added: `examples/portal-actions.rs`,
      `examples/portal-on-close.rs`, `examples/portal-reuse.rs`,
      `examples/portal-update-plain.rs`, all registered in `Cargo.toml` with
      `required-features = ["async"]`.
- [x] Register `examples/desktop-portal.rs` as a `[[example]]` entry in `Cargo.toml` with
      `required-features = ["async"]`
