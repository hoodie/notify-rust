pub mod notify*rust
pub use notify_rust::get_bundle_identifier_or_default
pub use notify_rust::set_application
pub mod notify_rust::error
pub use notify_rust::error::ApplicationError
pub use notify_rust::error::MacOsError
pub use notify_rust::error::NotificationError #[non_exhaustive] pub enum notify_rust::error::ErrorKind
pub notify_rust::error::ErrorKind::Conversion(alloc::string::String)
pub notify_rust::error::ErrorKind::ImplementationMissing
pub notify_rust::error::ErrorKind::MacNotificationSys(mac_notification_sys::error::Error)
pub notify_rust::error::ErrorKind::Msg(alloc::string::String)
pub notify_rust::error::ErrorKind::Parse(core::num::error::ParseIntError)
pub notify_rust::error::ErrorKind::SpecVersion(alloc::string::String)
impl core::convert::From<notify_rust::error::ErrorKind> for notify_rust::error::Error
pub fn notify_rust::error::Error::from(notify_rust::error::ErrorKind) -> notify_rust::error::Error
impl core::fmt::Debug for notify_rust::error::ErrorKind
pub fn notify_rust::error::ErrorKind::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::marker::Freeze for notify*rust::error::ErrorKind
impl core::marker::Send for notify_rust::error::ErrorKind
impl core::marker::Sync for notify_rust::error::ErrorKind
impl core::marker::Unpin for notify_rust::error::ErrorKind
impl core::marker::UnsafeUnpin for notify_rust::error::ErrorKind
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::error::ErrorKind
impl core::panic::unwind_safe::UnwindSafe for notify_rust::error::ErrorKind
impl<T, U> core::convert::Into<U> for notify_rust::error::ErrorKind where U: core::convert::From<T>
pub fn notify_rust::error::ErrorKind::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::error::ErrorKind where U: core::convert::Into<T>
pub type notify_rust::error::ErrorKind::Error = core::convert::Infallible
pub fn notify_rust::error::ErrorKind::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::error::ErrorKind where U: core::convert::TryFrom<T>
pub type notify_rust::error::ErrorKind::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::error::ErrorKind::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> core::any::Any for notify_rust::error::ErrorKind where T: 'static + ?core::marker::Sized
pub fn notify_rust::error::ErrorKind::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::error::ErrorKind where T: ?core::marker::Sized
pub fn notify_rust::error::ErrorKind::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::error::ErrorKind where T: ?core::marker::Sized
pub fn notify_rust::error::ErrorKind::borrow_mut(&mut self) -> &mut T
impl<T> core::convert::From<T> for notify_rust::error::ErrorKind
pub fn notify_rust::error::ErrorKind::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::error::ErrorKind where T: ?core::marker::Sized
pub struct notify_rust::error::Error
impl core::convert::From<&str> for notify_rust::error::Error
pub fn notify_rust::error::Error::from(&str) -> notify_rust::error::Error
impl core::convert::From<core::num::error::ParseIntError> for notify_rust::error::Error
pub fn notify_rust::error::Error::from(core::num::error::ParseIntError) -> notify_rust::error::Error
impl core::convert::From<mac_notification_sys::error::Error> for notify_rust::error::Error
pub fn notify_rust::error::Error::from(mac_notification_sys::error::Error) -> notify_rust::error::Error
impl core::convert::From<notify_rust::error::ErrorKind> for notify_rust::error::Error
pub fn notify_rust::error::Error::from(notify_rust::error::ErrorKind) -> notify_rust::error::Error
impl core::error::Error for notify_rust::error::Error
impl core::fmt::Debug for notify_rust::error::Error
pub fn notify_rust::error::Error::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::fmt::Display for notify*rust::error::Error
pub fn notify_rust::error::Error::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::marker::Freeze for notify*rust::error::Error
impl core::marker::Send for notify_rust::error::Error
impl core::marker::Sync for notify_rust::error::Error
impl core::marker::Unpin for notify_rust::error::Error
impl core::marker::UnsafeUnpin for notify_rust::error::Error
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::error::Error
impl core::panic::unwind_safe::UnwindSafe for notify_rust::error::Error
impl<T, U> core::convert::Into<U> for notify_rust::error::Error where U: core::convert::From<T>
pub fn notify_rust::error::Error::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::error::Error where U: core::convert::Into<T>
pub type notify_rust::error::Error::Error = core::convert::Infallible
pub fn notify_rust::error::Error::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::error::Error where U: core::convert::TryFrom<T>
pub type notify_rust::error::Error::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::error::Error::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::string::ToString for notify_rust::error::Error where T: core::fmt::Display + ?core::marker::Sized
pub fn notify_rust::error::Error::to_string(&self) -> alloc::string::String
impl<T> core::any::Any for notify_rust::error::Error where T: 'static + ?core::marker::Sized
pub fn notify_rust::error::Error::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::error::Error where T: ?core::marker::Sized
pub fn notify_rust::error::Error::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::error::Error where T: ?core::marker::Sized
pub fn notify_rust::error::Error::borrow_mut(&mut self) -> &mut T
impl<T> core::convert::From<T> for notify_rust::error::Error
pub fn notify_rust::error::Error::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::error::Error where T: ?core::marker::Sized
pub type notify_rust::error::Result<T> = core::result::Result<T, notify_rust::error::Error>
pub enum notify_rust::ActionResponse<'a>
pub notify_rust::ActionResponse::Closed(notify_rust::CloseReason)
pub notify_rust::ActionResponse::Custom(&'a str)
impl<'a> core::clone::Clone for notify_rust::ActionResponse<'a>
pub fn notify_rust::ActionResponse<'a>::clone(&self) -> notify_rust::ActionResponse<'a>
impl<'a> core::convert::From<&'a str> for notify_rust::ActionResponse<'a>
pub fn notify_rust::ActionResponse<'a>::from(&'a str) -> Self
impl<'a> core::fmt::Debug for notify_rust::ActionResponse<'a>
pub fn notify_rust::ActionResponse<'a>::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl<'a> core::marker::Freeze for notify*rust::ActionResponse<'a>
impl<'a> core::marker::Send for notify_rust::ActionResponse<'a>
impl<'a> core::marker::Sync for notify_rust::ActionResponse<'a>
impl<'a> core::marker::Unpin for notify_rust::ActionResponse<'a>
impl<'a> core::marker::UnsafeUnpin for notify_rust::ActionResponse<'a>
impl<'a> core::panic::unwind_safe::RefUnwindSafe for notify_rust::ActionResponse<'a>
impl<'a> core::panic::unwind_safe::UnwindSafe for notify_rust::ActionResponse<'a>
impl<T, U> core::convert::Into<U> for notify_rust::ActionResponse<'a> where U: core::convert::From<T>
pub fn notify_rust::ActionResponse<'a>::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::ActionResponse<'a> where U: core::convert::Into<T>
pub type notify_rust::ActionResponse<'a>::Error = core::convert::Infallible
pub fn notify_rust::ActionResponse<'a>::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::ActionResponse<'a> where U: core::convert::TryFrom<T>
pub type notify_rust::ActionResponse<'a>::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::ActionResponse<'a>::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for notify_rust::ActionResponse<'a> where T: core::clone::Clone
pub type notify_rust::ActionResponse<'a>::Owned = T
pub fn notify_rust::ActionResponse<'a>::clone_into(&self, &mut T)
pub fn notify_rust::ActionResponse<'a>::to_owned(&self) -> T
impl<T> core::any::Any for notify_rust::ActionResponse<'a> where T: 'static + ?core::marker::Sized
pub fn notify_rust::ActionResponse<'a>::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::ActionResponse<'a> where T: ?core::marker::Sized
pub fn notify_rust::ActionResponse<'a>::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::ActionResponse<'a> where T: ?core::marker::Sized
pub fn notify_rust::ActionResponse<'a>::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for notify_rust::ActionResponse<'a> where T: core::clone::Clone
pub unsafe fn notify_rust::ActionResponse<'a>::clone_to_uninit(&self, \*mut u8)
impl<T> core::convert::From<T> for notify_rust::ActionResponse<'a>
pub fn notify_rust::ActionResponse<'a>::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::ActionResponse<'a> where T: ?core::marker::Sized
pub enum notify_rust::CloseReason
pub notify_rust::CloseReason::CloseAction
pub notify_rust::CloseReason::Dismissed
pub notify_rust::CloseReason::Expired
pub notify_rust::CloseReason::Other(u32)
impl core::clone::Clone for notify_rust::CloseReason
pub fn notify_rust::CloseReason::clone(&self) -> notify_rust::CloseReason
impl core::cmp::Eq for notify_rust::CloseReason
impl core::cmp::PartialEq for notify_rust::CloseReason
pub fn notify_rust::CloseReason::eq(&self, &notify_rust::CloseReason) -> bool
impl core::convert::From<u32> for notify_rust::CloseReason
pub fn notify_rust::CloseReason::from(u32) -> Self
impl core::fmt::Debug for notify_rust::CloseReason
pub fn notify_rust::CloseReason::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::hash::Hash for notify*rust::CloseReason
pub fn notify_rust::CloseReason::hash<\**H: core::hash::Hasher>(&self, &mut \**H)
impl core::marker::Copy for notify_rust::CloseReason
impl core::marker::StructuralPartialEq for notify_rust::CloseReason
impl<F> notify_rust::CloseHandler<notify_rust::CloseReason> for F where F: core::ops::function::Fn(notify_rust::CloseReason)
pub fn F::call(&self, notify_rust::CloseReason)
impl core::marker::Freeze for notify_rust::CloseReason
impl core::marker::Send for notify_rust::CloseReason
impl core::marker::Sync for notify_rust::CloseReason
impl core::marker::Unpin for notify_rust::CloseReason
impl core::marker::UnsafeUnpin for notify_rust::CloseReason
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::CloseReason
impl core::panic::unwind_safe::UnwindSafe for notify_rust::CloseReason
impl<T, U> core::convert::Into<U> for notify_rust::CloseReason where U: core::convert::From<T>
pub fn notify_rust::CloseReason::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::CloseReason where U: core::convert::Into<T>
pub type notify_rust::CloseReason::Error = core::convert::Infallible
pub fn notify_rust::CloseReason::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::CloseReason where U: core::convert::TryFrom<T>
pub type notify_rust::CloseReason::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::CloseReason::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for notify_rust::CloseReason where T: core::clone::Clone
pub type notify_rust::CloseReason::Owned = T
pub fn notify_rust::CloseReason::clone_into(&self, &mut T)
pub fn notify_rust::CloseReason::to_owned(&self) -> T
impl<T> core::any::Any for notify_rust::CloseReason where T: 'static + ?core::marker::Sized
pub fn notify_rust::CloseReason::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::CloseReason where T: ?core::marker::Sized
pub fn notify_rust::CloseReason::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::CloseReason where T: ?core::marker::Sized
pub fn notify_rust::CloseReason::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for notify_rust::CloseReason where T: core::clone::Clone
pub unsafe fn notify_rust::CloseReason::clone_to_uninit(&self, \*mut u8)
impl<T> core::convert::From<T> for notify_rust::CloseReason
pub fn notify_rust::CloseReason::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::CloseReason where T: ?core::marker::Sized
pub enum notify_rust::Hint
pub notify_rust::Hint::ActionIcons(bool)
pub notify_rust::Hint::Category(alloc::string::String)
pub notify_rust::Hint::Custom(alloc::string::String, alloc::string::String)
pub notify_rust::Hint::CustomInt(alloc::string::String, i32)
pub notify_rust::Hint::DesktopEntry(alloc::string::String)
pub notify_rust::Hint::ImagePath(alloc::string::String)
pub notify_rust::Hint::Invalid
pub notify_rust::Hint::Resident(bool)
pub notify_rust::Hint::SoundFile(alloc::string::String)
pub notify_rust::Hint::SoundName(alloc::string::String)
pub notify_rust::Hint::SuppressSound(bool)
pub notify_rust::Hint::Transient(bool)
pub notify_rust::Hint::Urgency(notify_rust::Urgency)
pub notify_rust::Hint::X(i32)
pub notify_rust::Hint::Y(i32)
impl notify_rust::Hint
pub fn notify_rust::Hint::as_bool(&self) -> core::option::Option<bool>
pub fn notify_rust::Hint::as_i32(&self) -> core::option::Option<i32>
pub fn notify_rust::Hint::as_str(&self) -> core::option::Option<&str>
pub fn notify_rust::Hint::from_key_val(&str, &str) -> core::result::Result<notify_rust::Hint, alloc::string::String>
impl core::clone::Clone for notify_rust::Hint
pub fn notify_rust::Hint::clone(&self) -> notify_rust::Hint
impl core::cmp::Eq for notify_rust::Hint
impl core::cmp::PartialEq for notify_rust::Hint
pub fn notify_rust::Hint::eq(&self, &notify_rust::Hint) -> bool
impl core::fmt::Debug for notify_rust::Hint
pub fn notify_rust::Hint::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::hash::Hash for notify*rust::Hint
pub fn notify_rust::Hint::hash<\**H: core::hash::Hasher>(&self, &mut \**H)
impl core::marker::StructuralPartialEq for notify_rust::Hint
impl core::marker::Freeze for notify_rust::Hint
impl core::marker::Send for notify_rust::Hint
impl core::marker::Sync for notify_rust::Hint
impl core::marker::Unpin for notify_rust::Hint
impl core::marker::UnsafeUnpin for notify_rust::Hint
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::Hint
impl core::panic::unwind_safe::UnwindSafe for notify_rust::Hint
impl<T, U> core::convert::Into<U> for notify_rust::Hint where U: core::convert::From<T>
pub fn notify_rust::Hint::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::Hint where U: core::convert::Into<T>
pub type notify_rust::Hint::Error = core::convert::Infallible
pub fn notify_rust::Hint::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::Hint where U: core::convert::TryFrom<T>
pub type notify_rust::Hint::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::Hint::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for notify_rust::Hint where T: core::clone::Clone
pub type notify_rust::Hint::Owned = T
pub fn notify_rust::Hint::clone_into(&self, &mut T)
pub fn notify_rust::Hint::to_owned(&self) -> T
impl<T> core::any::Any for notify_rust::Hint where T: 'static + ?core::marker::Sized
pub fn notify_rust::Hint::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::Hint where T: ?core::marker::Sized
pub fn notify_rust::Hint::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::Hint where T: ?core::marker::Sized
pub fn notify_rust::Hint::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for notify_rust::Hint where T: core::clone::Clone
pub unsafe fn notify_rust::Hint::clone_to_uninit(&self, \*mut u8)
impl<T> core::convert::From<T> for notify_rust::Hint
pub fn notify_rust::Hint::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::Hint where T: ?core::marker::Sized
pub enum notify_rust::NotificationId
pub notify_rust::NotificationId::Mac(alloc::string::String)
pub notify_rust::NotificationId::Xdg(u32)
impl core::clone::Clone for notify_rust::NotificationId
pub fn notify_rust::NotificationId::clone(&self) -> notify_rust::NotificationId
impl core::cmp::Eq for notify_rust::NotificationId
impl core::cmp::PartialEq for notify_rust::NotificationId
pub fn notify_rust::NotificationId::eq(&self, &notify_rust::NotificationId) -> bool
impl core::convert::From<&str> for notify_rust::NotificationId
pub fn notify_rust::NotificationId::from(&str) -> Self
impl core::convert::From<alloc::string::String> for notify_rust::NotificationId
pub fn notify_rust::NotificationId::from(alloc::string::String) -> Self
impl core::convert::From<u32> for notify_rust::NotificationId
pub fn notify_rust::NotificationId::from(u32) -> Self
impl core::fmt::Debug for notify_rust::NotificationId
pub fn notify_rust::NotificationId::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::marker::StructuralPartialEq for notify*rust::NotificationId
impl core::marker::Freeze for notify_rust::NotificationId
impl core::marker::Send for notify_rust::NotificationId
impl core::marker::Sync for notify_rust::NotificationId
impl core::marker::Unpin for notify_rust::NotificationId
impl core::marker::UnsafeUnpin for notify_rust::NotificationId
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::NotificationId
impl core::panic::unwind_safe::UnwindSafe for notify_rust::NotificationId
impl<T, U> core::convert::Into<U> for notify_rust::NotificationId where U: core::convert::From<T>
pub fn notify_rust::NotificationId::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::NotificationId where U: core::convert::Into<T>
pub type notify_rust::NotificationId::Error = core::convert::Infallible
pub fn notify_rust::NotificationId::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::NotificationId where U: core::convert::TryFrom<T>
pub type notify_rust::NotificationId::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::NotificationId::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for notify_rust::NotificationId where T: core::clone::Clone
pub type notify_rust::NotificationId::Owned = T
pub fn notify_rust::NotificationId::clone_into(&self, &mut T)
pub fn notify_rust::NotificationId::to_owned(&self) -> T
impl<T> core::any::Any for notify_rust::NotificationId where T: 'static + ?core::marker::Sized
pub fn notify_rust::NotificationId::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::NotificationId where T: ?core::marker::Sized
pub fn notify_rust::NotificationId::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::NotificationId where T: ?core::marker::Sized
pub fn notify_rust::NotificationId::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for notify_rust::NotificationId where T: core::clone::Clone
pub unsafe fn notify_rust::NotificationId::clone_to_uninit(&self, \*mut u8)
impl<T> core::convert::From<T> for notify_rust::NotificationId
pub fn notify_rust::NotificationId::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::NotificationId where T: ?core::marker::Sized
pub enum notify_rust::NotificationResponse
pub notify_rust::NotificationResponse::Action(alloc::string::String)
pub notify_rust::NotificationResponse::Closed(notify_rust::CloseReason)
pub notify_rust::NotificationResponse::Reply(alloc::string::String)
impl notify_rust::NotificationResponse
pub fn notify_rust::NotificationResponse::is_default_action(&self) -> bool
impl core::clone::Clone for notify_rust::NotificationResponse
pub fn notify_rust::NotificationResponse::clone(&self) -> notify_rust::NotificationResponse
impl core::cmp::Eq for notify_rust::NotificationResponse
impl core::cmp::PartialEq for notify_rust::NotificationResponse
pub fn notify_rust::NotificationResponse::eq(&self, &notify_rust::NotificationResponse) -> bool
impl core::convert::From<&str> for notify_rust::NotificationResponse
pub fn notify_rust::NotificationResponse::from(&str) -> Self
impl core::convert::From<alloc::string::String> for notify_rust::NotificationResponse
pub fn notify_rust::NotificationResponse::from(alloc::string::String) -> Self
impl core::fmt::Debug for notify_rust::NotificationResponse
pub fn notify_rust::NotificationResponse::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::hash::Hash for notify*rust::NotificationResponse
pub fn notify_rust::NotificationResponse::hash<\**H: core::hash::Hasher>(&self, &mut \**H)
impl core::marker::StructuralPartialEq for notify_rust::NotificationResponse
impl core::marker::Freeze for notify_rust::NotificationResponse
impl core::marker::Send for notify_rust::NotificationResponse
impl core::marker::Sync for notify_rust::NotificationResponse
impl core::marker::Unpin for notify_rust::NotificationResponse
impl core::marker::UnsafeUnpin for notify_rust::NotificationResponse
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::NotificationResponse
impl core::panic::unwind_safe::UnwindSafe for notify_rust::NotificationResponse
impl<T, U> core::convert::Into<U> for notify_rust::NotificationResponse where U: core::convert::From<T>
pub fn notify_rust::NotificationResponse::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::NotificationResponse where U: core::convert::Into<T>
pub type notify_rust::NotificationResponse::Error = core::convert::Infallible
pub fn notify_rust::NotificationResponse::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::NotificationResponse where U: core::convert::TryFrom<T>
pub type notify_rust::NotificationResponse::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::NotificationResponse::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for notify_rust::NotificationResponse where T: core::clone::Clone
pub type notify_rust::NotificationResponse::Owned = T
pub fn notify_rust::NotificationResponse::clone_into(&self, &mut T)
pub fn notify_rust::NotificationResponse::to_owned(&self) -> T
impl<T> core::any::Any for notify_rust::NotificationResponse where T: 'static + ?core::marker::Sized
pub fn notify_rust::NotificationResponse::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::NotificationResponse where T: ?core::marker::Sized
pub fn notify_rust::NotificationResponse::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::NotificationResponse where T: ?core::marker::Sized
pub fn notify_rust::NotificationResponse::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for notify_rust::NotificationResponse where T: core::clone::Clone
pub unsafe fn notify_rust::NotificationResponse::clone_to_uninit(&self, \*mut u8)
impl<T> core::convert::From<T> for notify_rust::NotificationResponse
pub fn notify_rust::NotificationResponse::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::NotificationResponse where T: ?core::marker::Sized
pub enum notify_rust::Timeout
pub notify_rust::Timeout::Default
pub notify_rust::Timeout::Milliseconds(u32)
pub notify_rust::Timeout::Never
impl core::clone::Clone for notify_rust::Timeout
pub fn notify_rust::Timeout::clone(&self) -> notify_rust::Timeout
impl core::cmp::Eq for notify_rust::Timeout
impl core::cmp::PartialEq for notify_rust::Timeout
pub fn notify_rust::Timeout::eq(&self, &notify_rust::Timeout) -> bool
impl core::convert::From<core::time::Duration> for notify_rust::Timeout
pub fn notify_rust::Timeout::from(core::time::Duration) -> notify_rust::Timeout
impl core::convert::From<i32> for notify_rust::Timeout
pub fn notify_rust::Timeout::from(i32) -> notify_rust::Timeout
impl core::convert::From<notify_rust::Timeout> for i32
pub fn i32::from(notify_rust::Timeout) -> Self
impl core::default::Default for notify_rust::Timeout
pub fn notify_rust::Timeout::default() -> notify_rust::Timeout
impl core::fmt::Debug for notify_rust::Timeout
pub fn notify_rust::Timeout::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::marker::Copy for notify*rust::Timeout
impl core::marker::StructuralPartialEq for notify_rust::Timeout
impl core::str::traits::FromStr for notify_rust::Timeout
pub type notify_rust::Timeout::Err = core::num::error::ParseIntError
pub fn notify_rust::Timeout::from_str(&str) -> core::result::Result<Self, Self::Err>
impl core::marker::Freeze for notify_rust::Timeout
impl core::marker::Send for notify_rust::Timeout
impl core::marker::Sync for notify_rust::Timeout
impl core::marker::Unpin for notify_rust::Timeout
impl core::marker::UnsafeUnpin for notify_rust::Timeout
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::Timeout
impl core::panic::unwind_safe::UnwindSafe for notify_rust::Timeout
impl<T, U> core::convert::Into<U> for notify_rust::Timeout where U: core::convert::From<T>
pub fn notify_rust::Timeout::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::Timeout where U: core::convert::Into<T>
pub type notify_rust::Timeout::Error = core::convert::Infallible
pub fn notify_rust::Timeout::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::Timeout where U: core::convert::TryFrom<T>
pub type notify_rust::Timeout::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::Timeout::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for notify_rust::Timeout where T: core::clone::Clone
pub type notify_rust::Timeout::Owned = T
pub fn notify_rust::Timeout::clone_into(&self, &mut T)
pub fn notify_rust::Timeout::to_owned(&self) -> T
impl<T> core::any::Any for notify_rust::Timeout where T: 'static + ?core::marker::Sized
pub fn notify_rust::Timeout::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::Timeout where T: ?core::marker::Sized
pub fn notify_rust::Timeout::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::Timeout where T: ?core::marker::Sized
pub fn notify_rust::Timeout::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for notify_rust::Timeout where T: core::clone::Clone
pub unsafe fn notify_rust::Timeout::clone_to_uninit(&self, \*mut u8)
impl<T> core::convert::From<T> for notify_rust::Timeout
pub fn notify_rust::Timeout::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::Timeout where T: ?core::marker::Sized
pub enum notify_rust::Urgency
pub notify_rust::Urgency::Critical = 2
pub notify_rust::Urgency::Low = 0
pub notify_rust::Urgency::Normal = 1
impl core::clone::Clone for notify_rust::Urgency
pub fn notify_rust::Urgency::clone(&self) -> notify_rust::Urgency
impl core::cmp::Eq for notify_rust::Urgency
impl core::cmp::PartialEq for notify_rust::Urgency
pub fn notify_rust::Urgency::eq(&self, &notify_rust::Urgency) -> bool
impl core::convert::From<core::option::Option<u64>> for notify_rust::Urgency
pub fn notify_rust::Urgency::from(core::option::Option<u64>) -> notify_rust::Urgency
impl core::convert::From<u64> for notify_rust::Urgency
pub fn notify_rust::Urgency::from(u64) -> notify_rust::Urgency
impl core::convert::TryFrom<&str> for notify_rust::Urgency
pub type notify_rust::Urgency::Error = notify_rust::error::Error
pub fn notify_rust::Urgency::try_from(&str) -> core::result::Result<notify_rust::Urgency, Self::Error>
impl core::fmt::Debug for notify_rust::Urgency
pub fn notify_rust::Urgency::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::hash::Hash for notify*rust::Urgency
pub fn notify_rust::Urgency::hash<\**H: core::hash::Hasher>(&self, &mut \**H)
impl core::marker::Copy for notify_rust::Urgency
impl core::marker::StructuralPartialEq for notify_rust::Urgency
impl core::marker::Freeze for notify_rust::Urgency
impl core::marker::Send for notify_rust::Urgency
impl core::marker::Sync for notify_rust::Urgency
impl core::marker::Unpin for notify_rust::Urgency
impl core::marker::UnsafeUnpin for notify_rust::Urgency
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::Urgency
impl core::panic::unwind_safe::UnwindSafe for notify_rust::Urgency
impl<T, U> core::convert::Into<U> for notify_rust::Urgency where U: core::convert::From<T>
pub fn notify_rust::Urgency::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::Urgency where U: core::convert::Into<T>
pub type notify_rust::Urgency::Error = core::convert::Infallible
pub fn notify_rust::Urgency::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::Urgency where U: core::convert::TryFrom<T>
pub type notify_rust::Urgency::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::Urgency::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for notify_rust::Urgency where T: core::clone::Clone
pub type notify_rust::Urgency::Owned = T
pub fn notify_rust::Urgency::clone_into(&self, &mut T)
pub fn notify_rust::Urgency::to_owned(&self) -> T
impl<T> core::any::Any for notify_rust::Urgency where T: 'static + ?core::marker::Sized
pub fn notify_rust::Urgency::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::Urgency where T: ?core::marker::Sized
pub fn notify_rust::Urgency::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::Urgency where T: ?core::marker::Sized
pub fn notify_rust::Urgency::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for notify_rust::Urgency where T: core::clone::Clone
pub unsafe fn notify_rust::Urgency::clone_to_uninit(&self, \*mut u8)
impl<T> core::convert::From<T> for notify_rust::Urgency
pub fn notify_rust::Urgency::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::Urgency where T: ?core::marker::Sized #[non_exhaustive] pub struct notify_rust::Notification
pub notify_rust::Notification::actions: alloc::vec::Vec<alloc::string::String>
pub notify_rust::Notification::appname: alloc::string::String
pub notify_rust::Notification::body: alloc::string::String
pub notify_rust::Notification::icon: alloc::string::String
pub notify_rust::Notification::subtitle: core::option::Option<alloc::string::String>
pub notify_rust::Notification::summary: alloc::string::String
pub notify_rust::Notification::timeout: notify_rust::Timeout
impl notify_rust::Notification
pub fn notify_rust::Notification::action(&mut self, &str, &str) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::actions(&mut self, alloc::vec::Vec<alloc::string::String>) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::appname(&mut self, &str) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::auto_icon(&mut self) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::body(&mut self, &str) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::finalize(&self) -> notify_rust::Notification
pub fn notify_rust::Notification::icon(&mut self, &str) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::id(&mut self, u32) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::image_path(&mut self, &str) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::new() -> notify_rust::Notification
pub fn notify_rust::Notification::schedule_raw(&self, f64) -> notify_rust::error::Result<notify_rust::NotificationHandle>
pub fn notify_rust::Notification::show(&self) -> notify_rust::error::Result<notify_rust::NotificationHandle>
pub fn notify_rust::Notification::sound_name(&mut self, &str) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::subtitle(&mut self, &str) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::summary(&mut self, &str) -> &mut notify_rust::Notification
pub fn notify_rust::Notification::timeout<T: core::convert::Into<notify_rust::Timeout>>(&mut self, T) -> &mut notify_rust::Notification
impl core::clone::Clone for notify_rust::Notification
pub fn notify_rust::Notification::clone(&self) -> notify_rust::Notification
impl core::default::Default for notify_rust::Notification
pub fn notify_rust::Notification::default() -> notify_rust::Notification
impl core::fmt::Debug for notify_rust::Notification
pub fn notify_rust::Notification::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::marker::Freeze for notify*rust::Notification
impl core::marker::Send for notify_rust::Notification
impl core::marker::Sync for notify_rust::Notification
impl core::marker::Unpin for notify_rust::Notification
impl core::marker::UnsafeUnpin for notify_rust::Notification
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::Notification
impl core::panic::unwind_safe::UnwindSafe for notify_rust::Notification
impl<T, U> core::convert::Into<U> for notify_rust::Notification where U: core::convert::From<T>
pub fn notify_rust::Notification::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::Notification where U: core::convert::Into<T>
pub type notify_rust::Notification::Error = core::convert::Infallible
pub fn notify_rust::Notification::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::Notification where U: core::convert::TryFrom<T>
pub type notify_rust::Notification::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::Notification::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> alloc::borrow::ToOwned for notify_rust::Notification where T: core::clone::Clone
pub type notify_rust::Notification::Owned = T
pub fn notify_rust::Notification::clone_into(&self, &mut T)
pub fn notify_rust::Notification::to_owned(&self) -> T
impl<T> core::any::Any for notify_rust::Notification where T: 'static + ?core::marker::Sized
pub fn notify_rust::Notification::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::Notification where T: ?core::marker::Sized
pub fn notify_rust::Notification::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::Notification where T: ?core::marker::Sized
pub fn notify_rust::Notification::borrow_mut(&mut self) -> &mut T
impl<T> core::clone::CloneToUninit for notify_rust::Notification where T: core::clone::Clone
pub unsafe fn notify_rust::Notification::clone_to_uninit(&self, \*mut u8)
impl<T> core::convert::From<T> for notify_rust::Notification
pub fn notify_rust::Notification::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::Notification where T: ?core::marker::Sized
pub struct notify_rust::NotificationHandle
impl notify_rust::NotificationHandle
pub fn notify_rust::NotificationHandle::new(notify_rust::Notification) -> Self
pub fn notify_rust::NotificationHandle::wait_for_action<F>(self, F) where F: core::ops::function::FnOnce(&str)
impl core::fmt::Debug for notify_rust::NotificationHandle
pub fn notify_rust::NotificationHandle::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::ops::deref::Deref for notify*rust::NotificationHandle
pub type notify_rust::NotificationHandle::Target = notify_rust::Notification
pub fn notify_rust::NotificationHandle::deref(&self) -> &notify_rust::Notification
impl core::ops::deref::DerefMut for notify_rust::NotificationHandle
pub fn notify_rust::NotificationHandle::deref_mut(&mut self) -> &mut notify_rust::Notification
impl core::marker::Freeze for notify_rust::NotificationHandle
impl core::marker::Send for notify_rust::NotificationHandle
impl core::marker::Sync for notify_rust::NotificationHandle
impl core::marker::Unpin for notify_rust::NotificationHandle
impl core::marker::UnsafeUnpin for notify_rust::NotificationHandle
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::NotificationHandle
impl core::panic::unwind_safe::UnwindSafe for notify_rust::NotificationHandle
impl<P, T> core::ops::deref::Receiver for notify_rust::NotificationHandle where P: core::ops::deref::Deref<Target = T> + ?core::marker::Sized, T: ?core::marker::Sized
pub type notify_rust::NotificationHandle::Target = T
impl<T, U> core::convert::Into<U> for notify_rust::NotificationHandle where U: core::convert::From<T>
pub fn notify_rust::NotificationHandle::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::NotificationHandle where U: core::convert::Into<T>
pub type notify_rust::NotificationHandle::Error = core::convert::Infallible
pub fn notify_rust::NotificationHandle::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::NotificationHandle where U: core::convert::TryFrom<T>
pub type notify_rust::NotificationHandle::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::NotificationHandle::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> core::any::Any for notify_rust::NotificationHandle where T: 'static + ?core::marker::Sized
pub fn notify_rust::NotificationHandle::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::NotificationHandle where T: ?core::marker::Sized
pub fn notify_rust::NotificationHandle::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::NotificationHandle where T: ?core::marker::Sized
pub fn notify_rust::NotificationHandle::borrow_mut(&mut self) -> &mut T
impl<T> core::convert::From<T> for notify_rust::NotificationHandle
pub fn notify_rust::NotificationHandle::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::NotificationHandle where T: ?core::marker::Sized
pub struct notify_rust::ServerInformation
pub notify_rust::ServerInformation::name: alloc::string::String
pub notify_rust::ServerInformation::spec_version: alloc::string::String
pub notify_rust::ServerInformation::vendor: alloc::string::String
pub notify_rust::ServerInformation::version: alloc::string::String
impl core::fmt::Debug for notify_rust::ServerInformation
pub fn notify_rust::ServerInformation::fmt(&self, &mut core::fmt::Formatter<'*>) -> core::fmt::Result
impl core::marker::Freeze for notify_rust::ServerInformation
impl core::marker::Send for notify_rust::ServerInformation
impl core::marker::Sync for notify_rust::ServerInformation
impl core::marker::Unpin for notify_rust::ServerInformation
impl core::marker::UnsafeUnpin for notify_rust::ServerInformation
impl core::panic::unwind_safe::RefUnwindSafe for notify_rust::ServerInformation
impl core::panic::unwind_safe::UnwindSafe for notify_rust::ServerInformation
impl<T, U> core::convert::Into<U> for notify_rust::ServerInformation where U: core::convert::From<T>
pub fn notify_rust::ServerInformation::into(self) -> U
impl<T, U> core::convert::TryFrom<U> for notify_rust::ServerInformation where U: core::convert::Into<T>
pub type notify_rust::ServerInformation::Error = core::convert::Infallible
pub fn notify_rust::ServerInformation::try_from(U) -> core::result::Result<T, <T as core::convert::TryFrom<U>>::Error>
impl<T, U> core::convert::TryInto<U> for notify_rust::ServerInformation where U: core::convert::TryFrom<T>
pub type notify_rust::ServerInformation::Error = <U as core::convert::TryFrom<T>>::Error
pub fn notify_rust::ServerInformation::try_into(self) -> core::result::Result<U, <U as core::convert::TryFrom<T>>::Error>
impl<T> core::any::Any for notify_rust::ServerInformation where T: 'static + ?core::marker::Sized
pub fn notify_rust::ServerInformation::type_id(&self) -> core::any::TypeId
impl<T> core::borrow::Borrow<T> for notify_rust::ServerInformation where T: ?core::marker::Sized
pub fn notify_rust::ServerInformation::borrow(&self) -> &T
impl<T> core::borrow::BorrowMut<T> for notify_rust::ServerInformation where T: ?core::marker::Sized
pub fn notify_rust::ServerInformation::borrow_mut(&mut self) -> &mut T
impl<T> core::convert::From<T> for notify_rust::ServerInformation
pub fn notify_rust::ServerInformation::from(T) -> T
impl<T> objc2::rc::autorelease::AutoreleaseSafe for notify_rust::ServerInformation where T: ?core::marker::Sized
pub trait notify_rust::CloseHandler<T>
pub fn notify_rust::CloseHandler::call(&self, notify_rust::CloseReason)
impl<F> notify_rust::CloseHandler<()> for F where F: core::ops::function::Fn()
pub fn F::call(&self, notify_rust::CloseReason)
impl<F> notify_rust::CloseHandler<notify_rust::CloseReason> for F where F: core::ops::function::Fn(notify_rust::CloseReason)
pub fn F::call(&self, notify_rust::CloseReason)
pub trait notify_rust::ResponseHandler
pub fn notify_rust::ResponseHandler::call(self, &notify_rust::NotificationResponse)
impl<F> notify_rust::ResponseHandler for F where F: core::ops::function::FnOnce(&notify_rust::NotificationResponse)
pub fn F::call(self, &notify_rust::NotificationResponse)
