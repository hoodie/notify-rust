use dbus::arg::{messageitem::{MessageItem, MessageItemArray}, RefArg};

use std::cmp::Ordering;

use super::constants;
use crate::miniver::Version;


#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct NotificationImage {
    width:           i32,
    height:          i32,
    rowstride:       i32,
    alpha:           bool,
    bits_per_sample: i32,
    channels:        i32,
    data:            Vec<u8>
}

impl NotificationImage {
    /// Creates an image from a raw vector of bytes
    pub fn from_rgb(width: i32, height: i32, data: Vec<u8>) -> Result<Self, ImageError> {
        const MAX_SIZE: i32 = 0x0fff_ffff;
        if width > MAX_SIZE || height > MAX_SIZE {
            return Err(ImageError::TooBig);
        }

        let channels = 3i32;
        let bits_per_sample = 8;

        if data.len() != (width * height * channels) as usize {
            Err(ImageError::WrongDataSize)
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
}

/// Errors that can occur when creating an Image
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageError {
    /// The given image is too big. DBus only has 32 bits for width / height
    TooBig,
    /// The given bytes don't match the width, height and channel count
    WrongDataSize
}

impl From<NotificationImage> for MessageItem {
    fn from(img: NotificationImage) -> Self {
        let bytes = img.data.into_iter().map(MessageItem::Byte).collect();

        MessageItem::Struct(vec![MessageItem::Int32(img.width),
                                 MessageItem::Int32(img.height),
                                 MessageItem::Int32(img.rowstride),
                                 MessageItem::Bool(img.alpha),
                                 MessageItem::Int32(img.bits_per_sample),
                                 MessageItem::Int32(img.channels),
                                 MessageItem::Array(MessageItemArray::new(bytes, "ay".into()).unwrap()  )
                                ])
    }
}

/// matching image data key for each spec version
pub fn image_spec(version: Version) -> String {
    match version.cmp(&Version::new(1, 1)) {
        Ordering::Less => constants::IMAGE_DATA_1_0.to_owned(),
        Ordering::Equal => constants::IMAGE_DATA_1_1.to_owned(),
        Ordering::Greater => constants::IMAGE_DATA.to_owned()
    }
}