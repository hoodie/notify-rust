#[cfg(any(target_os = "windows", target_os = "macos"))]
fn main() {
    println!("this is an xdg only feature")
}

#[cfg(all(unix, not(target_os = "macos")))]
#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    color_backtrace::install();
    use std::time::Duration;

    use async_std::task::sleep;
    use notify_rust::Notification;

    // -------------------------------------------------------------------------
    // GNOME: why you might see nothing when running this from a terminal
    // -------------------------------------------------------------------------
    //
    // The GNOME portal backend (xdg-desktop-portal-gnome) forwards every portal
    // notification through GNOME Shell's org.gtk.Notifications API.  Before
    // showing anything, GNOME Shell does two checks:
    //
    //   1. The app ID must be a valid GLib application ID (reverse-DNS form,
    //      e.g. "org.example.MyApp" — at least two dot-separated components,
    //      no leading/trailing/consecutive dots).
    //
    //   2. A .desktop file named "<app-id>.desktop" must exist somewhere GIO
    //      can find it: $XDG_DATA_HOME/applications (~/.local/share/applications)
    //      or any directory in $XDG_DATA_DIRS (/usr/local/share, /usr/share, …).
    //      ~/.local/share/applications/ is sufficient — no need for system paths.
    //
    // If either check fails, GNOME Shell rejects the notification silently
    // (from the caller's perspective) and logs:
    //
    //   Error from gnome-shell:
    //     GDBus.Error:org.gtk.Notifications.Error.InvalidApp:
    //       The app by ID "" could not be found
    //
    // The portal derives the app ID by looking up the systemd user unit of the
    // D-Bus sender's process (via its cgroup).  When you run this example
    // directly from a terminal (cargo run --example desktop-portal), the process
    // has no matching systemd user service unit, so the app ID falls back to "",
    // which fails check 1 immediately.
    //
    // -------------------------------------------------------------------------
    // How to make it work on GNOME
    // -------------------------------------------------------------------------
    //
    // You need exactly two things:
    //
    // STEP 1 — install a .desktop file.
    //
    //   Create ~/.local/share/applications/org.notify.rust.desktop:
    //
    //     [Desktop Entry]
    //     Name=notify-rust portal example
    //     Type=Application
    //     Exec=/home/<you>/code/rust/notify-rust/target/debug/examples/desktop-portal
    //     NoDisplay=true
    //
    //   Then refresh the cache:
    //     update-desktop-database ~/.local/share/applications/
    //
    //   The stem of the filename ("org.notify.rust") is the app ID.  Choose a
    //   name that is a valid GLib application ID: reverse-DNS, all alphanumeric
    //   segments separated by dots, no hyphens (hyphens are allowed inside
    //   segments but they get systemd-escaped — see the note below).
    //
    // STEP 2 — launch the binary as a systemd user service with a matching name.
    //
    //   xdg-desktop-portal extracts the app ID from the unit name using the
    //   pattern:
    //
    //     app[-<launcher>]-<ApplicationID>[@<instance>].service
    //     app[-<launcher>]-<ApplicationID>-<random>.scope
    //
    //   So for app ID "org.notify.rust", the unit name is:
    //
    //     app-org.notify.rust.service
    //
    //   Run it with:
    //
    //     systemd-run --user --service-type=oneshot \
    //         --working-directory=/path/to/notify-rust \
    //         --unit=app-org.notify.rust.service \
    //         ./target/debug/examples/desktop-portal
    //
    //   NOTE on hyphens: systemd escapes hyphens in unit names using \xHH hex
    //   notation (e.g. "org.notify-rust" becomes "org.notify\x2drust" in the
    //   unit name).  Avoid hyphens in the app ID to keep the command simple.
    //
    //   NOTE on working directory: systemd-run defaults to $HOME, not the
    //   current directory.  The --working-directory flag is needed so that
    //   relative paths like "./examples/octodex-256.png" resolve correctly.
    //
    // -------------------------------------------------------------------------
    // Other portal backends (KDE, mako, dunst, …)
    // -------------------------------------------------------------------------
    //
    // This restriction is specific to the GNOME portal backend.  Other backends
    // do not enforce an app-ID lookup and will show portal notifications
    // unconditionally, even when launched from a terminal.
    // -------------------------------------------------------------------------
    // Expected behavior (verified on KDE Plasma 6.7.4 only, see tasks.md)
    // -------------------------------------------------------------------------
    //
    // Notification with image appears, then closes after 2s via handle.close().
    // -------------------------------------------------------------------------

    let handle = Notification::new()
        .summary("portal notification")
        // .body("this notification was sent via the desktop portal")
        // .urgency(Urgency::Critical)
        //
        // Use a themed icon name (bypasses portal icon validation entirely):
        // .icon_named("dialog-information")
        //
        // Or supply a file path (subject to portal icon validation).
        // xdg-desktop-portal validates file-descriptor icons by running
        // xdg-desktop-portal-validate-icon --ruleset=notification.
        // Icons that fail are silently dropped.  Constraints:
        //   - formats: png, jpeg, svg only
        //   - must be square (width == height)
        //   - max 512x512 px (raster), 4096x4096 px (svg), 4 MB file size
        //
        // octodex.jpg / octodex.png are 896x896 and will be rejected
        // ("Image too large").  octodex-256.png is a 256x256 resized copy
        // that passes validation.
        .image_path("./examples/octodex-256.png")
        .show_via_portal()
        .await?;

    sleep(Duration::from_secs(2)).await;

    handle.close();

    Ok(())
}
