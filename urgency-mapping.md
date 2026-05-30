# Urgency / Interruption Level — Cross-Platform Mapping

This document compares the urgency/priority concepts across the three platforms
and answers the question: **is it safe to use `Urgency` as the single
cross-platform abstraction and convert back and forth?**

---

## Platform concepts

### Linux/XDG — `Urgency`

Three levels, defined by the [Desktop Notifications Spec](https://specifications.freedesktop.org/notification-spec/latest/):

| Value      | Int | Semantics |
|------------|:---:|-----------|
| `Low`      |  0  | Informational. Server may suppress sound or display; may auto-expire early. |
| `Normal`   |  1  | Standard notification. Default timeout, default sound. |
| `Critical` |  2  | Must not time out. Server should keep it visible until the user dismisses it. |

This is the most minimal model: a plain linear scale of three steps.

---

### macOS — `InterruptionLevel` (UNUserNotificationCenter, macOS 12+)

Four levels:

| Value           | Semantics |
|-----------------|-----------|
| `Passive`       | Added to Notification Centre silently. No banner, no sound, screen stays off. |
| `Active`        | Standard banner, plays sound, lights screen. **Default.** |
| `TimeSensitive` | Breaks through Focus modes (Do Not Disturb, etc.). Still obeys mute. Requires `com.apple.developer.usernotifications.time-sensitive` entitlement. |
| `Critical`      | Breaks through mute and all Focus modes. Requires a special Apple entitlement (`com.apple.developer.usernotifications.critical-alerts`). |

Key difference from XDG: the axis is **Focus/DND penetration**, not persistence.
A `Critical` macOS notification can still auto-dismiss; it just ignores mute and DND.
There is no built-in "stay on screen forever" concept — that is controlled by the
`UNNotificationContent.interruptionLevel` together with the notification category
and action configuration.

---

### Windows — `ToastScenario` (WinRT)

Windows does not have a direct urgency concept. The closest analogue is
`ToastScenario`, which controls presentation behaviour:

| Value         | Semantics |
|---------------|-----------|
| `Default`     | Standard toast: appears briefly, then auto-dismisses to Action Centre. |
| `Reminder`    | Pre-expanded, stays on screen until user acts. No special sound. |
| `Alarm`       | Pre-expanded, stays on screen. Loops alarm audio by default. |
| `IncomingCall`| Pre-expanded, full-screen on mobile, stays on screen. Loops ringtone audio. |
| `Urgent`      | (Windows App SDK / Build 19041+) Breaks through Focus Assist. Not available on all targets; check `AppNotificationBuilder::IsUrgentScenarioSupported()` at runtime. |

Key difference: Windows scenarios are **use-case labels** (alarm, call, reminder),
not a linear priority scale. There is no concept of "suppress sound" or "silent
banner" at the scenario level — that requires separate audio configuration.

---

## Proposed mapping

```
Urgency::Low      → Passive          → Default      (silent / minimal)
Urgency::Normal   → Active           → Default      (standard)
Urgency::Critical → TimeSensitive*   → Reminder     (high-priority, persistent/DND-piercing)
```

\* `TimeSensitive` is used for `Critical` rather than macOS `Critical` because:
- macOS `Critical` requires a special Apple entitlement most apps cannot obtain.
- `TimeSensitive` pierces Focus/DND and is achievable without special entitlements,
  matching the practical intent of XDG `Critical` (important, don't suppress).
- If the caller has the entitlement and truly needs macOS `Critical`, they should
  use `interruption_level()` directly.

---

## Is round-trip safe?

**Mostly yes, with known lossy edges:**

| Direction | Loss |
|-----------|------|
| XDG → macOS | `Low` and `Normal` both become `Active`; the "suppress display" semantics of `Low` are lost. |
| XDG → Windows | `Low` and `Normal` both become `Default`; no way to express "quieter than normal". |
| macOS → XDG | `TimeSensitive` and `Critical` both map to `Critical`; distinction is lost. `Passive` maps to `Low`. |
| Windows → XDG | `Reminder`/`Alarm`/`IncomingCall` all map to `Critical`; scenario semantics are lost. `Default` maps to `Normal`. |

The three-level XDG scale is the narrowest common denominator. Mapping through
`Urgency` is **safe for the common cases** (silent, normal, important) and **lossy
only at the edges** (macOS `Passive` ≈ XDG `Low` but not identical; Windows
`Alarm`/`IncomingCall` have no urgency equivalent at all).

### Recommendation

- **Use `Urgency` as the cross-platform API.** It covers the 95% case cleanly.
- **Keep `interruption_level()` as a macOS-only escape hatch** for callers that
  need `Passive` or true `Critical` (entitlement required).
- **Do not expose `ToastScenario` through `Urgency`.** `Alarm` and `IncomingCall`
  are use-case labels, not priority levels, and should be surfaced as a separate
  platform-specific API if needed (see P2 in the progress tracker).
- **Document the lossy edges** in the `Urgency` rustdoc so callers know what to
  expect on each platform.

---

## Implementation checklist

- [ ] `Urgency::Low` → macOS: set `InterruptionLevel::Passive`
- [ ] `Urgency::Normal` → macOS: set `InterruptionLevel::Active` (already the default, so a no-op is fine)
- [ ] `Urgency::Critical` → macOS: set `InterruptionLevel::TimeSensitive`
- [ ] `Urgency::Low` / `Normal` → Windows: leave `ToastScenario` as `Default`
- [ ] `Urgency::Critical` → Windows: set `ToastScenario::Reminder`
- [ ] Update `Urgency` rustdoc with per-platform behaviour table
- [ ] Mark S1 ✅ in progress tracker

---

## Implementor note — API design option

Rather than having separate `urgency()` and `interruption_level()` builder methods
that each take their own concrete type, consider making the builder accept
`impl Into<PlatformType>` and implementing `From<Urgency>` for each platform type:

```rust
// macOS builder method
pub fn urgency(self, level: impl Into<InterruptionLevel>) -> Self { ... }

// Implementations
impl From<Urgency> for InterruptionLevel { ... }  // the mapping table above
impl From<Urgency> for ToastScenario { ... }       // Windows equivalent
```

Callers can then pass either the cross-platform type or the native one:

```rust
// cross-platform
Notification::new().urgency(Urgency::Critical);

// native macOS — full control, no abstraction loss
#[cfg(target_os = "macos")]
Notification::new().urgency(InterruptionLevel::Critical); // needs entitlement
```

This pattern is idiomatic Rust, not complicated, and elegantly solves the
"escape hatch" problem: there is only one builder method, but it accepts
anything that converts to the platform type. The `interruption_level()` method
added in 4.x can be deprecated and removed in favour of this.

The main consideration is that `urgency()` would need to be a platform-specific
method (different signature per platform), or the trait bound would need to be
hidden behind a cfg. The cleanest approach is probably a cfg-gated signature:

```rust
#[cfg(target_os = "macos")]
pub fn urgency(self, level: impl Into<InterruptionLevel>) -> Self { ... }

#[cfg(target_os = "windows")]
pub fn urgency(self, level: impl Into<ToastScenario>) -> Self { ... }

#[cfg(all(unix, not(target_os = "macos")))]
pub fn urgency(self, level: impl Into<Urgency>) -> Self { ... }
```

Down side: the method signature varies by platform, which is visible in docs.
An alternative is a single `urgency(Urgency)` method at crate root and separate
`interruption_level(InterruptionLevel)` / `scenario(ToastScenario)` methods as
platform-specific extensions — simpler but requires two method calls for native access.

**Verdict:** the `Into<PlatformType>` approach is worth doing. It is not too
complicated and is the most Rust-idiomatic solution.
