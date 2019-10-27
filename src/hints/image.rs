use dbus::arg::messageitem::{MessageItem, MessageItemArray};
use image::GenericImageView as _;
use thiserror::Error;
use displaydoc::Display;

use std::cmp::Ordering;
use std::convert::TryFrom;
use std::path::Path;

use super::constants;
use crate::miniver::Version;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct NotificationImage {
    width: i32,
    height: i32,
    rowstride: i32,
    alpha: bool,
    bits_per_sample: i32,
    channels: i32,
    data: Vec<u8>,
}

impl NotificationImage {
    /// Creates an image from a raw vector of bytes
    pub fn from_rgb(width: i32, height: i32, data: Vec<u8>) -> Result<Self, Error> {
        const MAX_SIZE: i32 = 0x0fff_ffff;
        if width > MAX_SIZE || height > MAX_SIZE {
            return Err(Error::TooBig);
        }

        let channels = 3i32;
        let bits_per_sample = 8;

        if data.len() != (width * height * channels) as usize {
            Err(Error::WrongDataSize)
        } else {
            Ok(Self {
                width,
                height,
                bits_per_sample,
                channels,
                data,
                rowstride: width * channels,
                alpha: false,
            })
        }
    }

    ///  Attempts to open the given path as image
    pub fn open<T: AsRef<Path> + Sized>(path: T) -> Result<Self, Error> {
        let dyn_img = image::open(&path).map_err(Error::CantOpen)?;
        NotificationImage::try_from(dyn_img)
    }
}

impl TryFrom<image::DynamicImage> for NotificationImage {
    type Error = Error;

    fn try_from (dyn_img: image::DynamicImage) -> Result<Self, Self::Error> {
        if let Some(image_data) = dyn_img.as_rgb8() {
            let (width, height) = dyn_img.dimensions();
            let image_data = image_data.clone().into_raw();
            Ok( NotificationImage::from_rgb(
                width as i32,
                height as i32,
                image_data)?)
        } else {
            Err(Error::CantConvert)
        }
    }
}

/// Errors that can occur when creating an Image
#[derive(Debug, Display, Error)]
pub enum Error {
    /// The given image is too big. DBus only has 32 bits for width / height
    TooBig,

    /// The given bytes don't match the width, height and channel count
    WrongDataSize,

    /// Can't open given path
    CantOpen(#[from] image::ImageError),

    /// Can't open given path
    CantConvert,
}

/// matching image data key for each spec version
pub fn image_spec(version: Version) -> String {
    match version.cmp(&Version::new(1, 1)) {
        Ordering::Less => constants::IMAGE_DATA_1_0.to_owned(),
        Ordering::Equal => constants::IMAGE_DATA_1_1.to_owned(),
        Ordering::Greater => constants::IMAGE_DATA.to_owned(),
    }
}

pub struct NotificationImageMessage(NotificationImage);

impl From<NotificationImage> for NotificationImageMessage {
    fn from(hint: NotificationImage) -> Self {
        NotificationImageMessage(hint)
    }
}

impl std::ops::Deref for NotificationImageMessage {
    type Target = NotificationImage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<NotificationImageMessage> for MessageItem {
    fn from(img_msg: NotificationImageMessage) -> Self {
        let img = img_msg.0;

        let bytes = img.data.into_iter().map(MessageItem::Byte).collect();

        MessageItem::Struct(vec![
            MessageItem::Int32(img.width),
            MessageItem::Int32(img.height),
            MessageItem::Int32(img.rowstride),
            MessageItem::Bool(img.alpha),
            MessageItem::Int32(img.bits_per_sample),
            MessageItem::Int32(img.channels),
            MessageItem::Array(MessageItemArray::new(bytes, "ay".into()).unwrap()),
        ])
    }
}
