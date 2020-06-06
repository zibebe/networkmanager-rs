use super::Error;
use dbus::blocking::{Connection as DBusConnection, Proxy as DBusProxy};
use std::time::Duration;

const DBUS_TIMEOUT_MS: u64 = 5000;
const NETWORK_MANAGER_BUS_NAME: &str = "org.freedesktop.NetworkManager";

pub struct DBusConnector {
    dbus_connection: DBusConnection,
}

impl DBusConnector {
    pub(super) fn new() -> Result<Self, Error> {
        Ok(DBusConnector {
            dbus_connection: DBusConnection::new_system()?,
        })
    }

    pub(super) fn create_proxy(&self, path: &str) -> DBusProxy<'_, &DBusConnection> {
        self.dbus_connection.with_proxy(
            NETWORK_MANAGER_BUS_NAME,
            path.to_owned(),
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }
}
