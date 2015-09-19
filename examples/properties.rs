extern crate dbus;

use dbus::{Connection, BusType, Props};

fn main() {
    let connection = Connection::get_private(BusType::System).unwrap();
    let props = Props::new(&connection,
        "org.freedesktop.Notifications",
        "/org/freedesktop/Notifications",
        "org.freedesktop.Notifications",
                           10000);

    println!("BackendVersion: {:#?}", props.get_all())
}
