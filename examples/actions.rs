#![allow(unused_imports)]
use notify_rust::{Hint, Notification};

#[cfg(windows)]
fn main () {
    println!("this is a unix only feature");
}

#[cfg(unix)]
fn main() {
    #[cfg(linux)]
    Notification::new()
        .summary("click me")
        .action("default", "default")    // IDENTIFIER, LABEL
        .action("clicked", "click here") // IDENTIFIER, LABEL
        .hint(Hint::Resident(true))
        .show()
        .unwrap()
        .wait_for_action({|action|
            match action {
                "default"  => println!("so boring"),
                "clicked"  => println!("that was correct"),
                // here "__closed" is a hardcoded keyword
                "__closed" => println!("the notification was closed"),
                _ => ()
            }
        });

    #[cfg(target_os = "macos")]
    Notification::new()
        .summary("PLATFORM ERROR")
        .subtitle("unsupported functionality")
        .body("cannot wait for closing on macOS.")
        .show()
        .unwrap();
}
