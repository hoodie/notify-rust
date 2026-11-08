# notify-rust 4.18

notify-rust 4.18 is out! We now finally support actions and on_close callbacks now work on macOS and Windows too.

Big thanks to everyone who contributed across 3 crates 🙏
Especially the contributions to our windows backend!

📦 https://crates.io/crates/notify-rust

#rustlang #rust

---

On macOS we even support inline replys now.

None of this would have been possible without some significant work on mac-notification-sys which now supports multiple parallel notifications.
Actions from one notification no longer block another.

📦 https://crates.io/crates/mac-notification-sys

---

Notify-rust now also has a preview for a whole new macOS backend for macos using the **not deprecated** UNUserNotificationCenter API 😂.
Though NSUserNotification is still the default, since it's crucial for CLI apps. Try it out using `--features preview-macos-un`.

📦 https://crates.io/crates/mac-usernotifications

---

## LinkedIn post

notify-rust 4.18 is out.

Up until now, the crate offered certain APIs only on linux (xdg).
Interactive actions and close-callbacks were not available on macOS and Windows.
With 4.18 this changes.

Actions and close-callbacks now work on all three platforms. On macOS we also support inline replies now.

We also snuck in a preview of a whole new macOS backend using the `UNUserNotificationCenter` API. The old `NSUserNotification` is still the default since it's crucial for CLI apps, but the new backend is worth trying if you already build a bundled app anyway. Try it with `--features preview-macos-un`.

This was a team effort across three crates and multiple contributors. Big thanks to everyone involved, especially https://www.linkedin.com/in/felix-d%C3%B6ring-7455a6204/ .

📦 https://crates.io/crates/notify-rust
📖 https://docs.rs/notify-rust

#rustlang #rust #opensource
