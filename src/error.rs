#![allow(missing_docs)]
#[cfg(all(unix, not(target_os = "macos")))]
use dbus;

#[cfg(target_os = "macos")]
use mac_notification_sys::error::{ApplicationError, NotificationError};
use std::fmt;
use std::num;
use std::ops::Deref;

#[derive(Debug)]
pub struct Error(pub failure::Error);

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum ErrorKind {
    #[fail(display="{}", _0)]
    #[cfg(all(unix, not(target_os = "macos")))]
    Dbus(#[cause] dbus::Error),

    #[fail(display="{}", _0)]
    Generic(#[cause] failure::Error),

    #[fail(display="")]
    Parse(#[cause] num::ParseIntError),

    #[fail(display="The running server supplied an unknown version: {}", _0 )]
    SpecVersion(String),

    #[fail(display="ParseError: {}", error)]
    ParseError{ #[cause] error: ::std::num::ParseIntError },

}

impl Into<Error> for ErrorKind {
    fn into(self) -> Error { Error(self.into()) }
}

// foreigh errors

impl From<failure::Error> for Error {
    fn from(e: failure::Error) -> Error {
        ErrorKind::Generic(e).into()
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl From<dbus::Error> for Error {
    fn from(e: dbus::Error) -> Error {
        ErrorKind::Dbus(e).into()
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
impl From<dbus::Error> for ErrorKind {
    fn from(e: dbus::Error) -> ErrorKind {
        ErrorKind::Dbus(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Error {
        ErrorKind::Parse(e).into()
    }
}

impl From<num::ParseIntError> for ErrorKind {
    fn from(e: num::ParseIntError) -> ErrorKind {
        ErrorKind::Parse(e)
    }
}

// Stuff we have to do to be compatible with Failure and ErrorChain

impl Deref for Error {
    type Target = failure::Error;

    fn deref(&self) -> &failure::Error {
        &self.0
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.0.downcast_ref() {
            #[cfg(all(unix, not(target_os = "macos")))]
            Some(ErrorKind::Dbus(e)) => Some(e),
            Some(ErrorKind::Parse(e)) => Some(e),
            Some(ErrorKind::ParseError { error }) => Some(error),
            // Some(ErrorKind::SpecVersion(s)) => Some(Box::new(s.to_owned()).into()),
            _ => None
        }
    }
}
