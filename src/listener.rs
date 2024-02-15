


#[cfg(target_os="windows")]
pub type NotificationListener = crate::windows::Listener;

#[cfg(target_os="macos")]
pub type NotificationListener = crate::macos::Listener;


impl NotificationListener {
    pub fn new() -> Result<NotificationListener> {
        NotificationListener::new()
    }


    pub fn listen<F>(&self, handler: F) -> Result<()>
    where
        F: FnMut(NotificationTriggerDetails) + Send + 'static,
    {
        self.add_listener(Box::new(handler))
    }
}
