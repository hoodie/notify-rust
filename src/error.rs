#![allow(missing_docs)]
#[cfg(all(unix, not(target_os = "macos")))]
use dbus;

#[cfg(target_os= "windows")]
use winrt;

#[cfg(target_os= "macos")]
use mac_notification_sys;

use std::num;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Dbus(dbus::Error) #[cfg(all(unix, not(target_os = "macos")))] ;
        MacNotificationSys(mac_notification_sys::error::Error) #[cfg(target_os = "macos")] ;
        Parse(num::ParseIntError);
    }

    errors {
        SpecVersion(version:String) {
            description("unknown spec version")
            display("The running server supplied an unknown version: {}", version)
        }
    }
}
