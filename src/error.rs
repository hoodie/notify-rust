#![allow(missing_docs)]

use std::{fmt, num};
#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
use crate::image::ImageError;
/// Convenient wrapper around `std::Result`. 
pub type Result<T> = ::std::result::Result<T, Error>;

/// The Error type.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind
}

/// The kind of an error.
#[derive(Debug)]
#[non_exhaustive]
pub enum ErrorKind {
    /// only here for backwards compatibility
    Msg(String),

    #[cfg(linux)]
    Dbus(dbus::Error),

    #[cfg(target_os = "macos")]
    MacNotificationSys(mac_notification_sys::error::Error),

    Parse(num::ParseIntError),

    SpecVersion(String),

    #[cfg(all(feature = "images", unix, not(target_os = "macos")))]
    Image(ImageError)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            #[cfg(linux)]
            ErrorKind::Dbus(ref e) => write!(f, "{}", e),
            #[cfg(target_os = "macos")]
            ErrorKind::MacNotificationSys(ref e) => write!(f, "{}", e),
            ErrorKind::Parse(ref e) => write!(f, "Parsing Error: {}", e),
            ErrorKind::SpecVersion(ref e) => write!(f, "{}", e),
            ErrorKind::Msg(ref e) => write!(f, "{}", e),
            #[cfg(all(feature = "images", unix, not(target_os = "macos")))]
            ErrorKind::Image(ref e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(linux)]
impl From<dbus::Error> for Error {
    fn from(e: dbus::Error) -> Error {
        Error { kind: ErrorKind::Dbus(e) }
    }
}

#[cfg(target_os = "macos")]
impl From<mac_notification_sys::error::Error> for Error {
    fn from(e: mac_notification_sys::error::Error) -> Error {
        Error { kind: ErrorKind::MacNotificationSys(e) }
    }
}

#[cfg(all(feature = "images", unix, not(target_os = "macos")))]
impl From<ImageError> for Error {
    fn from(e: ImageError) -> Error {
        Error { kind: ErrorKind::Image(e) }
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
