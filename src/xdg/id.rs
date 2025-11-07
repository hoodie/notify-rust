pub enum NotificationId {
    Global(u32),
    Portal(String),
}

impl NotificationId {
    pub fn as_global(&self) -> Option<u32> {
        match self {
            NotificationId::Global(id) => Some(*id),
            NotificationId::Portal(_) => None,
        }
    }

    pub fn as_portal(&self) -> Option<&str> {
        match self {
            NotificationId::Global(_) => None,
            NotificationId::Portal(id) => Some(id),
        }
    }
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
