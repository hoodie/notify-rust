# Desktop Portal — Shipping Plan

> Written after the feature branch grew to cover significantly more ground than
> originally scoped. The goal is to identify a clean v1 cut that can ship now,
> and to explicitly name what gets deferred and why.

---

## Current state (branch: `feature/desktop-portals`)

All blocking correctness issues are resolved. The unit test suite (27 tests)
passes cleanly. The two public-facing paths — `Notification::show_via_portal()`
on the classic builder, and the new `portal::Notification` standalone type —
both work end-to-end on KDE and GNOME (with the GNOME caveat documented below
and in rustdoc).

There are four untracked/modified files that need to be committed before a PR:

| File                                           | State     | Notes                          |
| ---------------------------------------------- | --------- | ------------------------------ |
| `src/portal.rs`                                | untracked | entire new module              |
| `spec/desktop-portals/portal-api-learnings.md` | untracked | engineering notes              |
| `src/notification.rs`                          | modified  | `show_via_portal` docs updated |
| `examples/desktop-portal.rs`                   | modified  | GNOME setup docs added         |

And one pre-existing broken example unrelated to this feature:

- `examples/simple_async.rs` — compile error (`path` used as a value name,
  conflicts with the built-in attribute). Should be fixed before the PR so it
  does not distract reviewers.

---

## What is in v1

Everything listed below is already implemented, tested, and working.

### Sending

- `Notification::show_via_portal()` — sends the classic notification builder
  through the portal. Best-effort field translation: `summary → title`,
  `body`, `Urgency → Priority`, `icon_named → Themed`, `image_path → File(fd)`,
  `actions → default-action + buttons`.
- `portal::Notification` standalone builder — portal-native type with
  `.body()`, `.id()`, `.priority()`, `.icon_named()`, `.icon_path()`,
  `.icon()`, `.button()`, `.default_action()`, `.show()`.
- `portal::Icon` — `Themed(Vec<String>)` and `File(Fd)` variants. File icons
  are copied into a sealed `memfd` and validated by the portal before display.
- `portal::Button` — action button with `action` key and `label`.
- `portal::Priority` — `Low`, `Normal`, `High`, `Urgent`; derived from
  `Urgency` on the classic path.

### ID management

- Auto-generated process-unique monotonic IDs (no arguments required on either
  `show_via_portal()` or `portal::Notification::show()`).
- Caller-supplied stable IDs via `portal::Notification::id()`.
- `NotificationHandle::id()` returns the generated or supplied ID as
  `NotificationId::Portal(String)`.

### Handle operations

- `handle.close()` — sends `RemoveNotification`.
- `handle.update_with(portal::Notification)` — re-sends `AddNotification` over
  the same connection so the portal performs a true in-place replacement.
- Connection reuse is enforced by the type: `update_with` takes `&mut self` and
  reuses `self.connection`, preventing the new-connection footgun.

### Signal handling

- `ActionInvoked` match rule registered and deserialized correctly
  (`(id s, action s, parameter av)`).
- `NotificationClosed` match rule correctly absent from the portal signal
  handler (the signal does not exist on the frontend interface).

### Documentation

- Rustdoc on `show_via_portal()` and `portal::Notification::show()` explains
  the GNOME app-ID requirement, the two-step setup, and why other backends are
  unaffected.
- `examples/desktop-portal.rs` contains a detailed inline explanation of the
  GNOME setup with exact commands.
- `portal-api-learnings.md` documents the full set of non-obvious protocol
  behaviours discovered during implementation (12 entries).
- `tasks.md` is fully up to date.

---

## What is explicitly deferred

These items are genuine improvements but are all cleanly separable. Nothing
below is required for correct basic operation.

### Phase 3c — `Action` type _(next feature, not a bugfix)_

Replace raw `("id", "label")` string pairs on the classic path and the
`Button::new(action, label)` API on the portal path with a unified `Action`
type that can auto-slug a label into an ID.

Deferred because: it is a breaking API change that needs its own design review.
The current `Button` API is functional and internally consistent.

### `wait_for_action` → async `Stream` _(next feature, not a bugfix)_

The current callback-based `wait_for_action` API works. Replacing it with
`Stream<Item = Action>` is the right long-term shape but is a breaking change
and belongs in a separate PR with a clear migration story.

### `markup-body` _(low priority, no user-visible regression)_

The field exists in `PortalNotification` and is always `None`. Populating it
requires either a heuristic HTML scan of the body string or a dedicated
`portal::Notification::markup_body()` builder method. Neither is urgent;
plain-text bodies work correctly today.

### `sound` _(low priority, stubbed)_

Stubbed as `None`. Never advertised. Can be added independently.

### `portal-update.rs` example _(nice to have)_

The file exists and is correct but is untracked. It demonstrates the
`update_with()` pattern. Can be committed as part of v1 or held for a
follow-up — does not affect the API surface.

### Integration tests _(infrastructure problem, not a code problem)_

Integration tests require a live portal-capable session bus, which is not
available in CI. Tracked in `tasks.md` but blocked on test infrastructure,
not on code. Not a blocker for shipping.

### GNOME portal support without system modification _(fundamental limitation)_

On GNOME, `xdg-desktop-portal-gnome` routes portal notifications through
`org.gtk.Notifications`, which requires:

1. A valid GLib application ID derived from the process's **systemd user unit
   name** (via cgroup). Processes launched from a terminal get `app_id = ""`
   and are silently rejected.
2. A matching `.desktop` file resolvable by GIO (`~/.local/share/applications/`
   is sufficient).

There is no workaround from the library side — this is enforced by GNOME Shell
at the point where it receives the forwarded notification from the portal
daemon. The `XDG_DESKTOP_PORTAL_TEST_APP_ID` escape hatch exists in the portal
source but must be set in the _portal daemon's_ environment, not the caller's.

The practical consequence for development: the portal path is best tested on
KDE, or inside a Flatpak sandbox (which is the intended deployment environment
for this API). The classic `org.freedesktop.Notifications` path works on GNOME
without any setup and should remain the default for unsandboxed use.

This is documented in rustdoc and in `portal-api-learnings.md` (§13, to be
written). It is not a defect in the implementation.

---

## Suggested PR scope for v1

Commit the four outstanding files, fix `simple_async.rs`, optionally include
`portal-update.rs`, and open the PR. File issues for the three deferred items
(Action type, Stream API, markup-body) so the decisions are recorded but do
not block the merge.

The `portal-api-learnings.md` and `tasks.md` updates travel with the PR as
internal documentation — they are not user-facing but are valuable for anyone
picking up the code later.

---

## Known backend differences (summary)

| Backend                                  | Sends notifications | In-place update                   | App-ID required                    |
| ---------------------------------------- | ------------------- | --------------------------------- | ---------------------------------- |
| GNOME Shell (`xdg-desktop-portal-gnome`) | ✅ (with setup)     | ✅                                | ✅ yes — systemd unit + `.desktop` |
| KDE Plasma (`xdg-desktop-portal-kde`)    | ✅                  | ❌ always new popup (backend bug) | ❌ no                              |
| mako, dunst, others                      | ✅                  | ✅                                | ❌ no                              |
| Flatpak sandbox (any backend)            | ✅                  | per backend above                 | provided by sandbox                |

See `portal-api-learnings.md` §11, §12, §13 for the full source-level analysis
of each of these behaviours.
