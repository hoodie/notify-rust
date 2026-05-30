# notify-rust 4.18 / 5.0 — Progress Tracker

Companion to [`notify-rust-roadmap.md`](./notify-rust-roadmap.md). The
roadmap describes intent. This file tracks state.

Status legend: ☐ todo · 🛠 in progress · ✅ done · ⛔ blocked · ❎ dropped

---

## Strategy

**5.0 first, 4.18 as a backport bonus.**

Build 5.0 correctly on macOS first, then Windows, then reconcile cross-platform
API surface. Publish 5.0-beta once all three platforms are solid. The 4.x
backport (on a `4-x` continuation branch) is only done if community feedback on
the beta shows real demand for staying on `macos_legacy`. Linux
wayland/desktop-portal support runs parallel to or after the beta.

---

## Decisions log

| ID  | Question                                                              | Decision |
|-----|-----------------------------------------------------------------------|----------|
| Q1  | Preview flag names                                                    | `preview_macos_un` and `preview_windows_win32_notif` |
| Q2  | `experimental` umbrella feature                                       | ❎ dropped — platforms stay independent |
| Q3  | Deprecate `wait_for_action(&str)` / `"__closed"` in 4.18             | ✅ yes, in 4.18, contingent on `response()` landing on all platforms first |
| Q4  | `mac-usernotifications` publication timeline                          | 0.1.0 alongside 4.18 backport; dev continues with 5.0 |
| Q5  | `tauri-winrt-notification` + `win32_notif` coexistence in 4.18       | ❎ deferred — moot under 5.0-first strategy |
| Q6  | `id()` stays `u32` on default XDG in 4.18                            | ❎ deferred — moot under 5.0-first strategy |
| Q7  | `ActionResponse` shape for 4.18                                       | ❎ deferred — moot under 5.0-first strategy |
| Q8  | MSRV bump for 5.0                                                     | ✅ fine to bump; exact version determined by deps |
| Q9  | `Urgency` on macOS — remove or keep                                   | ✅ keep and promote: define canonical cross-platform Urgency→platform mappings (see S1) |
| Q10 | `set_application` / `get_bundle_identifier_or_default` in 5.0        | ✅ gated behind `macos_legacy` only; no equivalent on UN path |
| Q11 | Timeline                                                              | milestone-driven; see Strategy section above |
| Q12 | Remove `show_debug` in 5.0                                            | ✅ yes, remove |
| Q13 | `response()` consumes `self` or borrows                               | ✅ consumes `self` — one-shot |
| Q14 | Typed `Action` builder on preview path in 4.18                        | ❎ moot — subsumed by 5.0-first strategy |

---

## 5.0 — unification release

### Milestones

| # | Milestone                                                                 | Status |
|---|---------------------------------------------------------------------------|:------:|
| M1 | macOS 5.0: UN backend feature-complete, legacy path preserved            |   ✅   |
| M2 | Windows 5.0: backend chosen (I2), implemented, `response()` working      |   ☐    |
| M3 | Cross-platform API reconciliation (win/mac/linux feature parity)         |   ☐    |
| M4 | 5.0-beta published                                                        |   ☐    |
| M5 | Linux wayland / desktop-portal support                                    |   ☐    |
| M6 | 4.x backport (conditional on user demand from beta feedback)              |   ☐    |

### Spec tasks

| ID  | Task                                                                              | Status | Notes |
|-----|-----------------------------------------------------------------------------------|:------:|-------|
| S1  | Define and document `Urgency` → platform mappings (XDG Low/Normal/Critical → macOS `InterruptionLevel` → Windows equivalent) | ☐ | prerequisite for cross-platform Urgency implementation |

### Investigation tasks

| ID  | Task                                                                              | Status | Notes |
|-----|-----------------------------------------------------------------------------------|:------:|-------|
| I1  | Audit `tauri-plugin-notification` API usage to validate new `response()`/`ActionResponse` design against their actual needs | ☐ | post-macOS |
| I2  | Evaluate `win7-notifications` (lib.rs/crates/win7-notifications) as alternative to `win32-notif` before committing to a Windows backend | ☐ | post-macOS |

### Breaking-change checklist

| ID  | Change                                                                       | Status | Notes |
|-----|------------------------------------------------------------------------------|:------:|-------|
| B1  | `show() -> Result<NotificationHandle>` on macOS legacy                       |   ✅   | already returns `Result<NotificationHandle>`; confirmed in code |
| B2  | `show() -> Result<NotificationHandle>` on Windows                            |   ☐    | |
| B3  | `NotificationHandle::id() -> NotificationId` everywhere                      |   ☐    | |
| B4  | Remove `wait_for_action(&str)` from macOS UN handle                          |   ✅   | removed from `preview_macos_un::NotificationHandle` |
| B5  | Remove `"__closed"` sentinel                                                 |   ☐    | still present on XDG |
| B6  | Remove `wait_for_action_response` from macOS UN handle                       |   ✅   | removed from `preview_macos_un::NotificationHandle` |
| B7  | Remove `on_close` from macOS UN handle                                       |   ☐    | still present in `usernotifications.rs`; was marked ✅ prematurely — `on_close` survives on XDG but must be removed from the UN handle |
| B8  | Flip macOS default to UN, gate legacy behind `macos_legacy`                  |   ✅   | `macos_legacy` feature added; `mac-notification-sys` now optional |
| B9  | Flip Windows default to new backend, gate legacy behind `windows_legacy`     |   ☐    | backend TBD pending I2 |
| B10 | Move `set_application` / `get_bundle_identifier_or_default` under `macos_legacy` | ✅  | gated on `feature = "macos_legacy"` in `lib.rs` |
| B11 | `Urgency` promoted to cross-platform with defined mappings (not removed)     |   ☐    | depends on S1 |
| B12 | Remove `show_debug`                                                          |   ✅   | |
| B13 | Rename `pure_usernotifications` flag to `preview_macos_un`                  |   ✅   | flag renamed; module is `usernotifications`; no-op alias kept in `Cargo.toml` |

### New unified API

| ID  | Item                                                                  | Status | Notes |
|-----|-----------------------------------------------------------------------|:------:|-------|
| U1  | `NotificationHandle::response() -> UserResponse` on XDG               |   ☐    | consumes `self` |
| U2  | `NotificationHandle::response_blocking() -> UserResponse` on XDG      |   ☐    | consumes `self` |
| U3  | Same on macOS UN                                                       |   ✅   | `response().await` and `response_blocking()` on `preview_macos_un::NotificationHandle` |
| U4  | Same on Windows                                                        |   ☐    | requires plumbing; backend TBD |
| U5  | `close()` on Windows                                                   |   ☐    | |
| U6  | `update()` / `update_async()` on Windows                               |   ☐    | partially present on branch |
| U7  | `Notification::thread_id(…)` builder + field + mapping on macOS UN     |   ✅   | field + builder in `notification.rs`; mapped in `From<&Notification>` in `usernotifications.rs` |
| U8  | Typed `Action` builder — migrate `action(id, label)` → `action(Action)` | ✅    | `Action`/`ActionKind` added to `action.rs`; `actions: Vec<Action>` in `Notification`; XDG helper `actions_xdg_strings()`; `dbus_rs`/`zbus_rs` updated; macOS UN maps `Button` and `Reply` variants with `requires_authentication`; examples updated |

### Validation gates for 5.0

| ID  | Gate                                                                           | Status |
|-----|--------------------------------------------------------------------------------|:------:|
| W1  | `cargo check` with default features on all three platforms                     |   🛠   | macOS ✅; Linux/Windows pending |
| W10 | Fix stale `lib.rs` compat table: `NotificationHandle::id()` and `close()` on macOS UN show blank but are implemented | ✅ | fixed in `lib.rs` |
| W2  | `cargo check --features macos_legacy` builds on macOS                          |   ✅   |
| W3  | `cargo check --features windows_legacy` builds on Windows                      |   ☐    |
| W4  | `cargo hack` feature powerset (depth 2) on all three platforms                 |   ☐    |
| W5  | Migration guide published (covers B1..B13)                                     |   ☐    |
| W6  | 5.0-beta published                                                             |   ☐    |
| W7  | beta feedback cycle (at least 2 weeks)                                         |   ☐    |
| W8  | CHANGELOG entry for 5.0                                                        |   ☐    |
| W9  | docs.rs build green on default features                                        |   ☐    |

---

## 4.18 — final 4.x release (backport, conditional)

> All 4.18 work happens on a `4-x` continuation branch **after** 5.0-beta,
> and only if community demand warrants it. Items below are recorded for
> reference but are not the active focus.

### Pre-flight (decisions to lock in at backport time)

| ID  | Item                                                               | Status | Notes |
|-----|--------------------------------------------------------------------|:------:|-------|
| Q5  | Decide whether new Windows backend and `tauri-winrt-notification` coexist | ☐ | defer to backport |
| Q6  | Confirm `id()` stays `u32` on default-cfg XDG in 4.18              |   ☐    | defer to backport |
| Q7  | Confirm public shape of `ActionResponse` for 4.18                  |   ☐    | defer to backport |

### Features (backport slice of 5.0)

| ID  | Feature                                                                                  | Status | Notes |
|-----|------------------------------------------------------------------------------------------|:------:|-------|
| F1  | macOS preview backend behind `preview_macos_un`                                          |   ☐    | cherry-pick from 5.0; `mac-usernotifications 0.1.0` must be published |
| F2  | Windows preview backend behind `preview_windows_win32_notif`                             |   ☐    | cherry-pick from 5.0 |
| F3  | Cross-platform `action` module (`ActionResponse`, `CloseReason`, `UserResponse`)         |   ☐    | additive on default-cfg |
| F4  | `NotificationId` enum exists, **not** yet returned from default-cfg `id()`               |   ☐    | |
| F5  | `Notification::hero_image` (Windows-only, additive)                                      |   ☐    | |
| F6  | `response()` on all platforms (prerequisite for F7)                                      |   ☐    | consumes `self` |
| F7  | `#[deprecated]` on `wait_for_action(&str)` and `"__closed"`                              |   ☐    | only after F6 is complete on all platforms |
| F9  | Docs: preview-backends section in README and crate root                                  |   ☐    | |
| F10 | macOS UN `interruption_level()` builder method                                           |   ✅   | already on main |

### Revert work needed on the source branches

| ID  | What                                                                  | Status | Notes |
|-----|-----------------------------------------------------------------------|:------:|-------|
| R1  | Restore macOS legacy default in `Cargo.toml` (`default = ["z"]`)      |   ☐    | macOS branch currently defaults to `preview_macos_un` |
| R2  | Restore legacy macOS `show() -> Result<()>`                           |   ☐    | macOS branch changed it on the legacy path too |
| R3  | Restore default Windows `show() -> Result<()>`                        |   ☐    | Windows branch changed it unconditionally |
| R4  | Restore default XDG `handle.id() -> u32`                              |   ☐    | macOS branch changed it crate-wide |
| R5  | Audit XDG public surface for accidental 4.x breaks on default cfg      |   ☐    | macOS branch touched `src/xdg/*` |

### Validation gates for 4.18

| ID  | Gate                                                                           | Status |
|-----|--------------------------------------------------------------------------------|:------:|
| V1  | `cargo check` with default features (Linux, macOS, Windows)                    |   ☐    |
| V2  | `cargo check --no-default-features`                                            |   ☐
    |
| V3  | `cargo check --features preview_macos_un` on macOS                             |   ☐    |
| V4  | `cargo check --features preview_windows_win32_notif` on Windows                |   ☐    |
| V5  | `cargo hack` feature powerset (depth 2) on all three platforms                 |   ☐    |
| V6  | All existing examples compile unchanged on default features                    |   ☐    |
| V7  | New examples for `preview_macos_un` and `preview_windows_win32_notif`          |   ☐    |
| V7b | Example for `interruption_level` feature                                        |   ✅   | `examples/interruption_level.rs` |
| V8  | CHANGELOG entry for 4.18                                                       |   ☐    |
| V9  | Public API diff vs 4.17 reviewed (`cargo public-api` or manual)                |   ☐    |

---

## Open follow-ups (post 5.0, optional)

| ID  | Idea                                                              | Notes |
|-----|-------------------------------------------------------------------|-------|
| P1  | Windows progress-bar API                                          | `windows_todo.md` future work |
| P2  | Windows `Scenario::Alarm` / `IncomingCall`                        | requires cross-platform scenario design |
| P3  | macOS UN `close()` and `notification_id()` on the handle           | spec table shows "not yet" |
| P4  | Remove `macos_legacy` / `windows_legacy` in a later 5.x minor      | once usage is gone |
