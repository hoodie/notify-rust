pub enum NotificationId {
    Global(u32),
    Portal(String),
}

impl From<u32> for NotificationId {
    fn from(id: u32) -> NotificationId {
        NotificationId::Global(id)
    }
}

impl From<String> for NotificationId {
    fn from(id: String) -> NotificationId {
        NotificationId::Portal(id)
    }
}
