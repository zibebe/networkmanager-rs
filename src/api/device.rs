use std::time::Duration;

use dbus::blocking::{Connection, Proxy};

use super::gen::OrgFreedesktopNetworkManagerDevice;
use super::{Error, DBUS_TIMEOUT_MS};

pub struct Device<'a> {
    connection: &'a Connection,
    path: String,
}

impl<'a> Device<'a> {
    pub(super) fn new(connection: &'a Connection, path: &str) -> Self {
        Device {
            connection,
            path: path.to_owned(),
        }
    }

    fn create_proxy(&self) -> Proxy<'_, &Connection> {
        self.connection.with_proxy(
            "org.freedesktop.NetworkManager",
            &self.path,
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }

    pub fn delete(&self) -> Result<(), Error> {
        Ok(self.create_proxy().delete()?)
    }

    pub fn disconnect(&self) -> Result<(), Error> {
        Ok(self.create_proxy().disconnect()?)
    }

    pub fn autoconnect(&self) -> Result<bool, Error> {
        Ok(self.create_proxy().autoconnect()?)
    }

    pub fn set_autoconnect(&self, value: bool) -> Result<(), Error> {
        Ok(self.create_proxy().set_autoconnect(value)?)
    }
}
