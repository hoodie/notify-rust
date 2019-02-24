#![allow(missing_docs)]
#[cfg(all(unix, not(target_os = "macos")))]
use dbus;

#[cfg(target_os = "macos")]
use mac_notification_sys::error::{ApplicationError, NotificationError};
use std::num;

pub type Result<T> = ::std::result::Result<T, Error>;
pub use failure::Error;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display="{}", _0)]
    #[cfg(all(unix, not(target_os = "macos")))]
    Dbus(#[cause] dbus::Error),

    #[fail(display="Parsing Error")]
    Parse(#[cause] num::ParseIntError),

    #[fail(display="The running server supplied an unknown version: {}", _0 )]
    SpecVersion(String),

    #[fail(display="ParseError: {}", error)]
    ParseError{ #[cause] error: ::std::num::ParseIntError },

}

#[cfg(all(unix, not(target_os = "macos")))]
impl From<dbus::Error> for ErrorKind {
    fn from(e: dbus::Error) -> ErrorKind {
        ErrorKind::Dbus(e)
    }
}

impl From<num::ParseIntError> for ErrorKind {
    fn from(e: num::ParseIntError) -> ErrorKind {
        ErrorKind::Parse(e)
    }
}
