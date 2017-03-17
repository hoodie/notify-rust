#![allow(unused_must_use)]
#![cfg(all(unix, not(target_os = "macos")))]
extern crate notify_rust;

#[cfg(test)]
mod ownworld{

use std::thread;
use notify_rust::*;
use notify_rust::server::*;

#[test]
#[ignore]
fn actions_vec() {

    let thread_handle = thread::spawn(move||{
    let mut server = NotificationServer::new();
        server.start( |notification|{
            assert_eq!(notification.actions[0], "actions_vec0");
            assert_eq!(notification.actions[1], "actions_vec1");
            assert_eq!(notification.actions[2], "actions_vec2");
            assert_eq!(notification.actions[3], "actions_vec3");
        })
    });

    #[allow(deprecated)]
    Notification::new()
        .summary("Notification with actions")
        .body("action1=\"Action One\", something_else=\"Something Else\"")
        .icon("dialog-information")
        .actions(
            vec![
                "actions_vec0".into(),
                "actions_vec1".into(),
                "actions_vec2".into(),
                "actions_vec3".into() ]).show();

    stop_server();
    assert!(thread_handle.join().is_ok());
}

#[test]
#[ignore]
fn actions_automatic() {

    let mut server = NotificationServer::new();
    let thread_handle = thread::spawn(move||{
        server.start( |notification|{
            assert_eq!(notification.actions[0], "actions_built0");
            assert_eq!(notification.actions[1], "actions_built1");
            assert_eq!(notification.actions[2], "actions_built2");
            assert_eq!(notification.actions[3], "actions_built3");
            assert_eq!(notification.timeout, Timeout::Milliseconds(6000));
        })
    });

    Notification::new()
        .summary("Another notification with actions")
        .body("action0=\"Press me please\", action1=\"firefox\"")
        .icon("dialog-information")
        .timeout(6000) //miliseconds
        .action("actions_built0", "actions_built1")
        .action("actions_built2", "actions_built3")
        .show();

    stop_server();
    assert!(thread_handle.join().is_ok());
}

#[test]
#[ignore]
#[should_panic]
fn no_server() {
    let mut server = NotificationServer::new();
    thread::spawn(move||{ server.start( |notification| println!("{:#?}", notification)) });

    stop_server();
    Notification::new()
        .summary("Another notification with actions")
        .body("action0=\"Press me please\", action1=\"firefox\"")
        .show().unwrap();
}

#[test]
#[ignore]
#[should_panic]
fn join_failed(){

    let thread_handle = thread::spawn(||{
        let mut server = NotificationServer::new();
        server.start( |notification|{
            assert_eq!(notification.timeout, Timeout::Milliseconds(6000));
            assert_eq!(notification.actions[0], "this is no action");
        })
    });

    Notification::new().summary("Notification without timeout").show();
    stop_server();
    assert!(thread_handle.join().is_ok());
}

}
