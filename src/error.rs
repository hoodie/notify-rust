#![allow(missing_docs)]
#[cfg(all(unix, not(target_os = "macos")))]
use dbus;

use std::num;

error_chain!{
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Dbus(dbus::Error) #[cfg(all(unix, not(target_os = "macos")))] ;
        Parse(num::ParseIntError);
    }

    errors {
        SpecVersion(version:String) {
            description("unknown spec version")
            display("The running server supplied an unknown version: {}", version)
        }
    }
}
