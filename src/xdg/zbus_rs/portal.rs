use std::{collections::HashMap, fs::File, os::fd::OwnedFd};

use crate::{
    error::*,
    notification::Notification,
    priority::Priority,
    xdg::{
        NOTIFICATION_PORTAL_BUS_NAME, NOTIFICATION_PORTAL_INTERFACE, NOTIFICATION_PORTAL_OBJECTPATH,
    },
    Hint,
};
use zbus::zvariant::{Dict, Fd, SerializeDict, Signature, Type, Value};

pub use super::handle::PortalNotificationHandle as NotificationHandle;

use sound::Sound;
mod sound {
    use super::*;
    #[derive(serde::Serialize, PartialEq, Debug, Type)]
    pub struct Sound(Fd<'static>);
}

use icon::{Icon, RawIcon};
mod icon {
    use std::{
        fs::OpenOptions,
        os::fd::{FromRawFd, IntoRawFd, RawFd},
        path::Path,
    };

    use memfd::{FileSeal, MemfdOptions, SealsHashSet};
    use nix::{
        fcntl::{self, FcntlArg, SealFlag},
        sys::memfd::{memfd_create, MemFdCreateFlag},
    };

    use super::*;

    pub type RawIcon = (&'static str, Value<'static>);
    // pub type RawIcon = Dict<'static, 'static>;
    // pub type RawIcon = HashMap<&'static str, Value<'static>>;
    #[derive(serde::Serialize, PartialEq, Debug)]
    pub enum Icon {
        Themed(Vec<String>),
        File(Fd<'static>),
    }

    impl Into<RawIcon> for Icon {
        fn into(self) -> RawIcon {
            let pair = match self {
                Icon::Themed(names) => ("themed", Value::from(names)),
                Icon::File(fd) => ("file-descriptor", Value::from(fd)),
            };
            // HashMap::from([pair])
            pair
        }
    }

    // impl Into<Dict<'static, 'static>> for Icon {
    //     fn into(self) -> Dict<'static, 'static> {
    //         let icon = match self {
    //             Icon::Themed(names) => ("themed", Value::from(names)),
    //             Icon::File(fd) => ("file-descriptor", Value::from(fd)),
    //         };

    //         let val = Dict::from(HashMap::from([icon]));
    //         val
    //     }
    // }

    impl From<OwnedFd> for Icon {
        fn from(fd: OwnedFd) -> Self {
            Icon::File(Fd::from(fd))
        }
    }

    impl From<File> for Icon {
        fn from(file: File) -> Self {
            Icon::File(Fd::from(OwnedFd::from(file)))
        }
    }

    impl Icon {
        pub fn themed<I: Into<String>>(names: Vec<I>) -> Self {
            Icon::Themed(names.into_iter().map(Into::into).collect())
        }

        pub fn open(path: &str) -> Option<Self> {
            // let file = File::open(path).ok()?;
            // let file = Self::open_and_seal_file(path).unwrap();
            //
            let file = Self::copy_file_to_sealed_memfd(path);

            Some(file.into())
        }
        fn copy_file_to_sealed_memfd<P: AsRef<Path>>(path: P) -> File {
            // Step 1: Open the source file on disk
            let mut src_file = File::open(&path).unwrap();

            // Step 2: Create a sealable memfd with sealing allowed
            let opts = MemfdOptions::default().allow_sealing(true);
            let mfd = opts.create("sealed_memfd").unwrap();

            // Step 3: Resize the memfd to the size of the source file
            let src_size = src_file.metadata().unwrap().len();
            mfd.as_file().set_len(src_size).unwrap();

            // Step 4: Copy the contents from the source file to the memfd
            let mut memfd_file = mfd.as_file().try_clone().unwrap();
            std::io::copy(&mut src_file, &mut memfd_file).unwrap();

            // Step 5: Add seals to prevent any resizing
            let mut seals = SealsHashSet::new();
            seals.insert(FileSeal::SealShrink);
            seals.insert(FileSeal::SealGrow);
            mfd.add_seals(&seals).unwrap();

            // Step 6: Add the SealSeal flag to prevent further modifications
            let add_seal = mfd.add_seal(FileSeal::SealSeal);
            add_seal.unwrap();

            // Return the memfd as a File object for further reading
            mfd.into_file()
        }
    }

    impl Type for Icon {
        fn signature() -> Signature<'static> {
            Signature::from_static_str_unchecked("v")
        }
    }
}

#[derive(Debug, SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
pub struct PortalNotification {
    title: String,
    body: String,
    #[zvariant(rename = "markup-body")]
    markup_body: Option<String>,
    priority: Priority, // low, normal, high, urgent
    icon: Option<RawIcon>,
    sound: Option<Sound>,
    // default_action: Option<String>,
    // default_action_target: Option<String>,
    // actions: Option<Vec<String>>,
    // hints: Option<HashMap<String, Value>>,
    // category: Option<String>,
}

impl From<&Notification> for PortalNotification {
    fn from(notification: &Notification) -> Self {
        let urgency = notification
            .hints
            .iter()
            .find(|h| matches!(h, Hint::Urgency(_)));

        eprintln!("urgency: {:?}", urgency);

        let priority = if let Some(Hint::Urgency(urgency)) = urgency {
            Priority::from(*urgency)
        } else {
            Priority::Normal
        };

        let icon = Icon::open(&notification.icon).map(Into::into);

        eprintln!("priority: {:?}", priority);
        Self {
            title: notification.summary.clone(),
            body: notification.body.clone(),
            markup_body: None,
            priority,
            icon,
            sound: None,
            // default_action: todo!(),
            // default_action_target: todo!(),
            // actions: todo!(),
        }
    }
}

async fn add_notification(
    notification: &Notification,
    id: &str,
    connection: &zbus::Connection,
) -> Result<NotificationHandle> {
    connection
        .call_method(
            NOTIFICATION_PORTAL_BUS_NAME.into(),
            NOTIFICATION_PORTAL_OBJECTPATH,
            NOTIFICATION_PORTAL_INTERFACE.into(),
            "AddNotification",
            &(id, PortalNotification::from(notification)),
        )
        .await?;

    Ok(NotificationHandle::new(
        id,
        connection.clone(),
        notification.clone(),
    ))
}

pub async fn remove_notification(id: &str, connection: &zbus::Connection) -> Result<()> {
    connection
        .call_method(
            NOTIFICATION_PORTAL_BUS_NAME.into(),
            NOTIFICATION_PORTAL_OBJECTPATH,
            NOTIFICATION_PORTAL_INTERFACE.into(),
            "RemoveNotification",
            &id,
        )
        .await?;

    Ok(())
}

pub(crate) async fn connect_and_send_notification(
    notification: &Notification,
    id: &str,
) -> Result<NotificationHandle> {
    let connection = zbus::Connection::session().await?;
    add_notification(notification, id, &connection).await
}
