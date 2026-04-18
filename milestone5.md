# Notify-Rust 5.0 Milestone

## Feature Flags: rename `"z"` / `"d"` to `"zbus"` / `"dbus"`

- [ ] switch from "z vs d" to "zbus vs dbus" (already aliased internally, just needs the rename)
- [ ] make serde a default feature with zbus

## Noops Methods for other Platforms

- [x] add `noops` feature that adds no-op stubs so cross-platform code compiles without `cfg` guards

## Issue Template

- [ ] ask people to open a discussion if they have a question

## Allow an `on_close` handler without consuming the `NotificationHandle` #199

- [x] change signature so the handle is not consumed

## Image API — unify and clarify `image()` / `image_path()` / `image_data()`

These three methods do fundamentally different things but the naming doesn't communicate it:

- `image()` opens a file and sends raw pixel data as `Hint::ImageData` — and returns `Result<&mut Self>`
- `image_path()` sets `Hint::ImagePath` (a path hint, no file I/O) — and returns `&mut Self`
- `image_data()` is a thin manual wrapper around `Hint::ImageData`

* [ ] drop or rename `image()` — the odd-one-out `Result` return breaks the builder chain
* [ ] keep `image_path()` and `image_data()` as the canonical pair
* [ ] consider `Notification::image(ImageData)` as a single entry point taking an enum

## Builder Pattern Consistency

Builder methods take `&mut self -> &mut Self`, but `finalize()` clones into an owned value.
This hybrid is awkward — pick one convention.

- [ ] decide between by-ref (`&mut self`) or by-value (`self -> Self`) builders
- [ ] remove `finalize()` if switching to by-value, or document why it's needed with by-ref

## `wait_for_action` — remove magic `"__closed"` sentinel string

Two `// FIXME: remove backward compatibility with 5.0` comments already mark this.
The `ActionResponse` enum exists and has a `Closed(CloseReason)` variant.

- [ ] change closure signature from `FnOnce(&str)` to `FnOnce(ActionResponse)`
- [ ] remove the `"__closed"` hard-coded string

## Remove deprecated items

- [ ] `Notification::actions(Vec<String>)` — deprecated in favour of `.action()`
- [ ] `Notification::show_debug()` — deprecated, "was never meant to be public API"
- [ ] `Notification::at_bus()` — deprecated, "this is a test only feature"

## `Urgency`: fix `From<u64>` → `From<u8>`

Already marked with TODO comments in `urgency.rs`:

```rust
// TODO: remove this in v5.0
impl From<u64> for Urgency { ... }

// TODO: make this the default in v5.0
// impl From<u8> for Urgency { ... }
```

- [ ] remove `From<u64>` impl
- [ ] add `From<u8>` impl (matches the spec)

## `Hint::Invalid` — remove from public API

`Hint::Invalid` is an internal sentinel used when parsing fails. It is public, pollutes
`match` exhaustiveness, and already has a `// TODO find a better solution` comment.

- [ ] make `Hint::Invalid` private or remove it entirely
- [ ] `from_key_val` already returns `Result<Hint, String>` — lean on that

## Unify the two `ServerInformation` structs

There is one in `src/lib.rs` (public) and one in `src/xdg/mod.rs`. Same shape, different types,
only the xdg one gets populated.

- [ ] consolidate into a single canonical `ServerInformation` in `src/xdg/mod.rs` and re-export it
- [ ] implement `serde::Deserialize` for it (see issue #221)

## `ErrorKind::Msg(String)` — remove backwards-compat escape hatch

The source comment says "only here for backwards compatibility."

- [ ] replace `ErrorKind::Msg` with specific, meaningful error variants
- [ ] fix the misleading `ImplementationMissing` message which still references the old feature flag names

## `Notification` struct fields — reduce public surface

Several fields are `pub` on the struct itself (`appname`, `summary`, `body`, `icon`, `hints`,
`actions`, `timeout`) while others are `pub(crate)`. Mixing these levels is inconsistent and
locks in internal representation details.

- [ ] audit which fields genuinely need to be `pub`
- [ ] expose read access via methods instead where appropriate
- [ ] `hints` in particular leaks the internal `HashSet` split between `hints` and `hints_unique`

## Linux Wayland / Desktop Portals (`feature/desktop-portals` branch)

There is a WIP branch (`origin/feature/desktop-portals`) that adds support for sending
notifications via the [XDG Desktop Portal](https://flatpak.github.io/xdg-desktop-portal/)
(`org.freedesktop.portal.Notification`) in addition to the classic
`org.freedesktop.Notifications` D-Bus interface. This is the correct path for sandboxed
apps (Flatpak/Snap) and Wayland compositors that don't expose the legacy interface.

Merging it for 5.0 would be natural, but it already introduces several breaking changes:

- **`Notification::icon` changes from `String` to `Option<String>`** — the field type
  changes and `Default` no longer initialises it to an empty string.
- **New `icon_named()` and `icon_path()` methods** — the portal distinguishes between
  themed icon names and file-based icons, so the API is extended accordingly.
- **New `Priority` enum** — the portal uses `low / normal / high / urgent` strings
  instead of the D-Bus `Urgency` byte. A `From<Urgency> for Priority` conversion is
  provided but `Priority` is a separate public type that needs to be exposed.
- **`NotificationHandle::id()` return type changes** — from `u32` to a new
  `NotificationId` type that can hold either a numeric (D-Bus) or string (portal) ID.
- **`show_via_portal()`** — new async method on `Notification`; the signature still has
  rough edges (currently requires a caller-supplied `id: &str`, marked WIP).
- **New `ZbusPortal` variant on the internal `NotificationHandleInner` enum** — not
  public but affects any code pattern-matching on the handle.
- **New dependencies** — `memfd` and `nix` are added unconditionally on Linux for icon
  file-descriptor passing; this may need to be gated behind the portal feature flag.

- [ ] decide whether portal support ships as an opt-in feature flag or becomes the default on Linux
- [ ] finalise the `NotificationId` type and update `NotificationHandle::id()` accordingly
- [ ] resolve the caller-supplied `id` problem in `show_via_portal()` (generate one internally)
- [ ] gate `memfd` / `nix` dependencies behind the portal feature flag
- [ ] expose `Priority` in the public API if portals are enabled
- [ ] update platform support table in `lib.rs` docs

## Usorted thoughts

- higher level abstraction over bundle configuration
- maybe rename the image feature
