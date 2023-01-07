use notify_rust::server;
use std::error::Error;
#[cfg(target_os = "macos")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(target_os = "windows")]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(unix, not(feature = "server"), not(target_os = "macos")))]
fn main() {
    println!("this is a xdg only feature")
}

#[cfg(all(feature = "server", unix, not(target_os = "macos")))]
//#[async_std::main]
// async
fn main() -> Result<(), Box<dyn Error>> {
    use futures_util::{select, FutureExt};
    use notify_rust::{server::ReceivedNotification, CloseReason};

    let timeout = std::env::args()
        .nth(1)
        .and_then(|x| x.parse::<u64>().ok())
        .unwrap_or(1);

    std::env::set_var("RUST_LOG", "zbus=debug,server=trace,notify_rust=trace");
    color_backtrace::install();
    env_logger::init();

    // notify_rust::server::blocking_start_with(|notification| eprintln!("{notification:#?}"))
    server::blocking_start_with(move |received: ReceivedNotification| {
        log::debug!("enter handler");
        std::thread::sleep(std::time::Duration::from_secs(dbg!(timeout)));
        log::debug!("wake up handler");

        async_std::task::block_on(async {
            if let Some((action, closer)) =
                Option::zip(received.action_tx.upgrade(), received.close_tx.upgrade())
            {
                select!(
                    _ = action.send("action".into()).fuse() => (),
                    _ = closer.send(CloseReason::Dismissed).fuse() => {},
                );
            } else {
                log::warn!("channel upgrade failed, can no longer send action or close")
            }
        });

        log::debug!("handler done");
    })?;

    Ok(())
}
