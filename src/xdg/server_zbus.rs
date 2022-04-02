#![allow(missing_docs)]
use crate::{ensure, CloseReason, Hint, ServerInformation, NOTIFICATION_NAMESPACE, NOTIFICATION_OBJECTPATH};
use std::{collections::HashMap, error::Error};
use zbus::{dbus_interface, export::futures_util::TryStreamExt, Connection, MessageStream, SignalContext};

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
    stop_listener: event_listener::Event,
}

impl<H: NotificationHandler + 'static + Sync + Send> NotificationServer<H> {
    fn with_handler(handler: H) -> Self {
        let stop_listener = event_listener::Event::new();
        NotificationServer {
            count: 0,
            handler,
            stop_listener,
        }
    }
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
    async fn stop(&self) -> bool {
        log::info!("received stop");
        self.stop_listener.notify(1);
        true
    }

    /// Can be `async` as well.
    #[allow(clippy::too_many_arguments)]
    fn get_server_information(&self) -> ServerInformation {
        log::trace!("received info request");
        ServerInformation {
            name: String::from("name"),
            vendor: String::from("hoodie"),
            version: String::from(env!("CARGO_PKG_VERSION")),
            spec_version: String::from("1.1"),
        }
    }

    /// Can be `async` as well.
    #[allow(clippy::too_many_arguments)]
    async fn notify(
        &mut self,
        appname: String,
        id: u32,
        icon: String,
        summary: String,
        body: String,
        raw_actions: Vec<String>,
        raw_hints: HashMap<String, zvariant::OwnedValue>,
        timeout: i32,
        #[zbus(signal_context)] ctx: SignalContext<'_>,
    ) -> zbus::fdo::Result<u32> {
        let actions = Action::from_vec(&raw_actions);
        let hints = raw_hints
            .into_iter()
            .filter_map(|(k, v)| match Hint::from_zbus(&k, v.into()) {
                Ok(hint) => Some(hint),
                Err(error) => {
                    log::error!("invalid notification hint {error}");
                    None
                }
            })
            .collect();

        let received = ReceivedNotification {
            appname,
            id,
            icon,
            summary,
            body,
            actions,
            hints,
            timeout,
        };
        log::debug!("received {:?}", received);
        // log::debug!("signal context{:?}", ctx);

        if let Some(action) = received.actions.get(0) {
            Self::action_invoked(&ctx, self.count, &action.tag).await?;
        }

        self.handler.call(received);

            log::trace!("sleep");
            async_std::task::sleep(std::time::Duration::from_millis(1600)).await;
            log::trace!("wake up");

            log::trace!("sending closed signal");
            Self::notification_closed(&ctx, self.count, CloseReason::Expired).await.unwrap();
            log::trace!("sent closed signal");
            log::trace!("sleep");
            async_std::task::sleep(std::time::Duration::from_millis(1600)).await;
            log::trace!("wake up");
        self.count += 1;
        Ok(self.count)
    }

    #[dbus_interface(signal)]
    async fn action_invoked(ctx: &SignalContext<'_>, id: u32, action: &str) -> zbus::Result<()>;

    #[dbus_interface(signal)]
    async fn notification_closed(ctx: &SignalContext<'_>, id: u32, reason: CloseReason) -> zbus::Result<()>;
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
    log::info!("start blocking");
    zbus::block_on(start_with_internal(handler))
}

async fn start_with_internal<H: NotificationHandler + 'static + Sync + Send>(handler: H) -> Result<(), Box<dyn Error>> {
    let server_state = NotificationServer::with_handler(handler);
    log::info!("instantiated server");
    let stopped = server_state.stop_listener.listen();

    zbus::ConnectionBuilder::session()?
        .name(NOTIFICATION_NAMESPACE)?
        .serve_at(NOTIFICATION_OBJECTPATH, server_state)?
        .build()
        .await?;
    log::info!(
        "launch session\n {:?}\n {:?}",
        NOTIFICATION_NAMESPACE,
        NOTIFICATION_OBJECTPATH
    );

    stopped.wait();
    log::info!("shutting down");

    Ok(())
}

/// Starts the server
async fn _start_with_internal2<H: NotificationHandler + 'static + Sync + Send>(
    handler: H,
) -> Result<(), Box<dyn Error>> {
    let server_state = NotificationServer::with_handler(handler);
    log::info!("instantiated server");

    let connection = Connection::session().await?;
    log::info!("opened connection");

    let server_available = connection
        .object_server()
        // .name(NOTIFICATION_NAMESPACE)
        .at(NOTIFICATION_OBJECTPATH, server_state)
        .await?;
    ensure!(server_available, "server object-path already taken");
    log::info!("serving interface {:?}", NOTIFICATION_OBJECTPATH);

    connection.request_name(NOTIFICATION_NAMESPACE).await?;
    log::info!("acquired namespace {:?}", NOTIFICATION_NAMESPACE);

    let mut stream = MessageStream::from(connection);
    while let Some(msg) = stream.try_next().await? {
        log::debug!("received message: {}", msg);
        // log::debug!("count: {}", server_state.count);
    }
    log::info!("shutting down");

    Ok(())
}
