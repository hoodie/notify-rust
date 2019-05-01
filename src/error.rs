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
