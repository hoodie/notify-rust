# towards notify-rust 5.0

There are effectively **three** macOS stacks in play:

1. **`mac-notification-sys` / legacy** (`NSUserNotificationCenter`, no feature flag, default on macOS) — deprecated by Apple, no dismiss detection, no async, blocking `send()` that can't even observe close. The whole `NotificationHandle` we added here is new and partly fake.
2. **`preview_macos_un`** (`UNUserNotificationCenter`, feature flag) — the new path, futures-based, proper dismiss detection, the right long-term home.
3. **XDG** (Linux/BSD) — callback/closure based today, but already has `response().await` and `response_blocking()` as the modern path alongside the deprecated `wait_for_action(&str)` shim.

### Phase 1 — now (this branch)

**Stop adding new surface area to the legacy path.** The `NotificationHandle` for `mac-notification-sys` should be minimal and clearly tombstoned:

- Keep `show()` working (fire and forget, no blocking) — the banner still appears.
- Keep `wait_for_action<F: FnOnce(&str)>` as the deprecated shim, since that's what we promised not to break.
- **Do not** add `wait_for_action_response`, `response_blocking`, `on_close` to the legacy handle. Those are new API surface on a frozen path. If someone calls `show()` on legacy, they get a handle that only has the old method.
- The `send_and_wait` approach we just added is honest — it at least blocks for a click — but it still can't detect dismiss. That's fine, just document it clearly.

**Align `preview_macos_un::NotificationHandle` with the XDG shape** as closely as possible:

- `response().await` → `UserResponse` (already there)
- `response_blocking()` → `UserResponse` (already there)
- Deprecated `wait_for_action<F: FnOnce(&str)>` shim (already there)
- No `wait_for_action_response` — skip the middle generation entirely on the new path.

### Phase 2 — 4.x minor

**Deprecate `wait_for_action_response` on XDG** (already done with `since = "4.1.18"`). Make `response().await` / `response_blocking()` the single blessed API on all platforms that support it.

**Do not add new API to the `mac-notification-sys` feature path.** It stays present and functional, but is frozen. No `#[deprecated]` yet — that comes in 5.0 when it moves behind an explicit `macos_legacy` feature flag.

**The canonical `NotificationHandle` API going forward is just two methods:**

```/dev/null/strategy.rs#L1-4
// modern, works on XDG + preview_macos_un
async fn response(&self) -> UserResponse;
fn response_blocking(&self) -> UserResponse;  // blocks thread, use sparingly
```

Everything else (`wait_for_action`, `wait_for_action_response`, `on_close`) is deprecated scaffolding kept for back-compat.

### Phase 3 — 5.0

- Remove `wait_for_action(&str)` and the `"__closed"` sentinel everywhere.
- Remove `wait_for_action_response`.
- Remove `on_close` (callers use `response().await` and match on `Closed`).
- Move `mac-notification-sys` / the legacy path behind an explicit opt-in feature flag called `macos_legacy`. It stays functional but users must consciously enable it.
- `set_application` / `get_bundle_identifier_or_default` only available with `macos_legacy`.
- `preview_macos_un` becomes the default on macOS (the flag may be renamed or removed).

---

### Release strategy — 4.x+1 and 5.0-rc1 in parallel

The next release will be a 4.x minor that ships the `preview_macos_un` integration as-is, including the intentional API break on `NotificationHandle::id()` — it now returns `NotificationId` instead of `u32`. This is a semver break that we are accepting early and keeping until 5.0, so users have something concrete to migrate against.

The plan is to publish the 4.x release and a 5.0-rc1 side by side, so users can compare both and decide when to jump. The 4.x release acts as a migration guide in itself.

**What is intentionally not supported in the first 4.x drop:**

- No in-place update support via `preview_macos_un` yet. The `update()` method exists on the handle but the `update_via_stored_id` pattern (passing a string id across two separate `Notification::new()` calls) only works once the caller manages the id string themselves. The `recycling_one_id` XDG pattern translates to macOS by converting the `u32` to a string id, which works but is documented as inadvisable.

---

## Immediate concrete actions for this branch

1. **Remove `wait_for_action_response` and `on_close` from `legacy::NotificationHandle`** — we just added them and they're new API on a path we're killing.
2. **Remove `wait_for_action_response` from `preview_macos_un::NotificationHandle`** — skip the middle generation, the new path should only have `response()` and the deprecated `&str` shim.
3. ~~Add a `#[deprecated]` to `legacy::show_notification` and the `set_application` / `get_bundle_identifier_or_default` re-exports.~~ Deferred to 5.0 — the legacy path is frozen but not deprecated in 4.x.
4. ~~Update the platform table in the docs to reflect the new reality.~~ ✅ done

## Breaking API changes: 4.x → 5.0

| API                                                    | 4.x                                              | 5.0                                           | Notes                                                                                                             |
| ------------------------------------------------------ | ------------------------------------------------ | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `NotificationHandle::id()`                             | returns `NotificationId` (was `u32`)             | returns `NotificationId`                      | intentional early break in 4.x; XDG wraps the `u32` in `NotificationId::Xdg`, macOS returns `NotificationId::Mac` |
| `wait_for_action<F: FnOnce(&str)>`                     | still present                                    | removed                                       | `"__closed"` sentinel gone; use `response_blocking()` and match on `UserResponse::Closed`                         |
| `wait_for_action_response`                             | still present                                    | removed                                       | superseded by `response_blocking()`                                                                               |
| `on_close`                                             | still present                                    | removed                                       | use `response_blocking()` and match on `UserResponse::Closed`                                                     |
| `mac-notification-sys` / legacy macOS path             | present, no new API                              | optional feature called "macos_legacy"        |                                                                                                                   |
| `preview_macos_un` feature flag                  | required to opt in to `UNUserNotificationCenter` | becomes the default (flag renamed or removed) |                                                                                                                   |
| `set_application` / `get_bundle_identifier_or_default` | present (legacy path only)                       | only with "macos_legacy" turned on            | only needed for legacy path                                                                                       |

---

foobar
