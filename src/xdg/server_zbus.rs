#![allow(missing_docs)]
use crate::{ensure, Hint, NOTIFICATION_NAMESPACE, NOTIFICATION_OBJECTPATH};
use std::{collections::HashMap, error::Error};
use zbus::{dbus_interface, export::futures_util::TryStreamExt, Connection, MessageStream};

#[derive(Debug)]
pub struct Action {
    pub tag: String,
    pub description: String,
}
impl Action {
    fn from_pair(pair: (&String, &String)) -> Action {
        Self {
            tag: pair.0.to_owned(),
            description: pair.1.to_owned(),
        }
    }
    fn from_vec(raw: &[String]) -> Vec<Action> {
        raw.iter()
            .zip(raw.iter().map(Some).chain(Some(None)).skip(1))
            .filter_map(|(a, b)| b.map(|b| (a, b)))
            .map(Action::from_pair)
            .collect()
    }
}

#[derive(Debug)]
pub struct ReceivedNotification {
    pub appname: String,
    pub id: u32,
    pub icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<Action>,
    // pub hints: HashMap<String, zvariant::OwnedValue>,
    pub hints: Vec<Hint>,
    pub timeout: i32,
}

pub trait NotificationHandler {
    fn call(&self, notification: ReceivedNotification);
}

impl<F> NotificationHandler for F
where
    F: Fn(ReceivedNotification),
{
    fn call(&self, notification: ReceivedNotification) {
        self(notification)
    }
}

struct NotificationServer<H: NotificationHandler + 'static + Sync + Send> {
    count: u32,
    handler: H,
}

// #[dbus_interface]
#[cfg_attr(feature = "debug_namespace", dbus_interface(name = "de.hoodie.Notifications"))]
#[cfg_attr(
    not(feature = "debug_namespace"),
    dbus_interface(name = "org.freedesktop.Notifications")
)]
impl<H> NotificationServer<H>
where
    H: NotificationHandler + 'static + Sync + Send,
{
    /// Can be `async` as well.
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &mut self,
        appname: String,
        id: u32,
        icon: String,
        summary: String,
        body: String,
        raw_actions: Vec<String>,
        raw_hints: HashMap<String, zvariant::OwnedValue>,
        timeout: i32,
    ) -> u32 {
        let actions = Action::from_vec(&raw_actions);
        let hints = raw_hints
            .into_iter()
            .filter_map(|(k, v)| match Hint::from_zbus(&k, v.into()) {
                Ok(hint) => Some(hint),
                Err(error) => {
                    eprint!("{error}");
                    None
                }
            })
            .collect();

        self.handler.call(ReceivedNotification {
            appname,
            id,
            icon,
            summary,
            body,
            actions,
            hints,
            timeout,
        });

        self.count += 1;
        self.count
    }
}

/// Starts the server
pub async fn start() -> Result<(), Box<dyn Error>> {
    start_with_internal(
        |ReceivedNotification {
             appname,
             id,
             icon,
             summary,
             body,
             actions,
             hints,
             timeout,
         }| {
            eprintln!("app:     {appname:?}");
            eprintln!("id:      {id:?}");
            eprintln!("summary: {summary:?}");
            eprintln!("body:    {body:?}");
            eprintln!("actions: {actions:#?}");
            eprintln!("icon:    {icon:#?}");
            eprintln!("hints:   {hints:#?}");
            eprintln!("timeout: {timeout:#?}");
        },
    )
    .await
}

pub fn start_blocking() -> Result<(), Box<dyn Error>> {
    zbus::block_on(start())
}

/// Starts the server
pub async fn start_with<H: NotificationHandler + 'static + Sync + Send>(handler: H) -> Result<(), Box<dyn Error>> {
    start_with_internal(handler).await
}

/// Starts the server
pub fn start_with_blocking<H: NotificationHandler + 'static + Sync + Send>(handler: H) -> Result<(), Box<dyn Error>> {
    zbus::block_on(start_with_internal(handler))
}

/// Starts the server
async fn start_with_internal<H: NotificationHandler + 'static + Sync + Send>(handler: H) -> Result<(), Box<dyn Error>> {
    let server_state = NotificationServer { count: 0, handler };
    let connection = Connection::session().await?;
    let server_available = connection
        .object_server()
        .at(NOTIFICATION_OBJECTPATH, server_state)
        .await?;
    ensure!(server_available, "server object-path already taken");

    connection.request_name(NOTIFICATION_NAMESPACE).await?;

    let mut stream = MessageStream::from(connection);

    while let Some(msg) = stream.try_next().await? {
        println!("Got message: {}", msg);
    }

    Ok(())
}
