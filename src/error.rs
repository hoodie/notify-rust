#![allow(missing_docs)]
#[cfg(all(unix, not(target_os = "macos")))]
use dbus;

#[cfg(target_os = "macos")]
use mac_notification_sys::error::{ApplicationError, NotificationError};
use std::{fmt, num};

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    // #[fail(display="{}", _0)]
    #[cfg(all(unix, not(target_os = "macos")))]
    Dbus(dbus::Error),

    // #[fail(display="Parsing Error")]
    Parse(num::ParseIntError),

    // #[fail(display="The running server supplied an unknown version: {}", _0 )]
    SpecVersion(String),
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::Dbus(ref e) => write!(f, "{}", e),
            ErrorKind::Parse(ref e) => write!(f, "Parsing Error: {}", e),
            ErrorKind::SpecVersion(ref e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(all(unix, not(target_os = "macos")))]
impl From<dbus::Error> for Error {
    fn from(e: dbus::Error) -> Error {
        Error { kind: ErrorKind::Dbus(e) }
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        Error { kind: ErrorKind::Parse(e) }
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error { kind }
    }
}

/// Just the usual bail macro
#[macro_export]
#[doc(hidden)]
macro_rules! bail {
    ($e:expr) => {
        return Err($e.into());
    };
    ($fmt:expr, $($arg:tt)+) => {
        return Err(format!($fmt, $($arg)+).into());
    };
}

/// Exits a function early with an `Error` if the condition is not satisfied.
///
/// Similar to `assert!`, `ensure!` takes a condition and exits the function
/// if the condition fails. Unlike `assert!`, `ensure!` returns an `Error`,
/// it does not panic.
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! ensure {
    ($cond:expr, $e:expr) => {
        if !($cond) {
            bail!($e);
        }
    };
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !($cond) {
            bail!($fmt, $($arg)*);
        }
    };
}
