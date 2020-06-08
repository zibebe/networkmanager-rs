use super::errors::Error;
use dbus::blocking::{Connection, Proxy};
use std::time::Duration;

const DBUS_TIMEOUT_MS: u64 = 5000;

macro_rules! proxy {
    ($input:ident) => {
        $input.dbus_object.create_proxy()
    };
}

pub struct DBusConnection {
    _connection: Connection,
}

impl DBusConnection {
    pub fn new() -> Result<Self, Error> {
        Ok(DBusConnection {
            _connection: Connection::new_system()?,
        })
    }
}

pub(crate) struct DBusObject<'a> {
    pub(crate) connection: &'a DBusConnection,
    pub(crate) bus: String,
    pub(crate) path: String,
}

impl<'a> DBusObject<'a> {
    pub(crate) fn new(connection: &'a DBusConnection, bus: &str, path: &str) -> Self {
        DBusObject {
            connection,
            bus: bus.to_owned(),
            path: path.to_owned(),
        }
    }
    pub(crate) fn create_proxy(&self) -> Proxy<'_, &Connection> {
        self.connection._connection.with_proxy(
            &self.bus,
            &self.path,
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }
}
