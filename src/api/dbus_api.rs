use super::errors::Error;
use dbus::blocking::{Connection as DBusConnection, Proxy as DBusProxy};
use std::time::Duration;

const DBUS_TIMEOUT_MS: u64 = 5000;

#[macro_export]
macro_rules! proxy {
    ($input:ident) => {
        $input.dbus_api.create_proxy(&$input.dbus_path)
    };
}

pub struct DBusApi {
    dbus_connection: DBusConnection,
    bus: String,
}

impl DBusApi {
    pub(super) fn new(bus: &str) -> Result<Self, Error> {
        Ok(DBusApi {
            dbus_connection: DBusConnection::new_system()?,
            bus: bus.to_string(),
        })
    }

    pub(super) fn create_proxy(&self, path: &str) -> DBusProxy<'_, &DBusConnection> {
        self.dbus_connection.with_proxy(
            &self.bus,
            path.to_owned(),
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }
}
