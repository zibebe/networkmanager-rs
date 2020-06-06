use std::time::Duration;

use dbus::blocking::{Connection, Proxy};

use super::device::Device;
use super::gen::OrgFreedesktopNetworkManager;
use super::{Error, DBUS_TIMEOUT_MS};

pub enum ReloadScope {
    Everything = 0x00,
    NmConfig = 0x01,
    ResolvConfig = 0x02,
    DnsPlugin = 0x04,
}

pub struct NetworkManager {
    connection: Connection,
}

impl NetworkManager {
    pub fn new() -> Result<Self, Error> {
        Ok(NetworkManager {
            connection: Connection::new_system()?,
        })
    }

    fn create_proxy(&self) -> Proxy<'_, &Connection> {
        self.connection.with_proxy(
            "org.freedesktop.NetworkManager",
            "/org/freedesktop/NetworkManager",
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }

    /// Returns only realized network devices
    pub fn get_devices(&self) -> Result<Vec<Device>, Error> {
        let dev_paths = self.create_proxy().get_devices()?;
        let devs = dev_paths
            .iter()
            .map(|e| Device::new(&self.connection, e))
            .filter_map(Result::ok)
            .collect();
        Ok(devs)
    }

    /// Returns all the network devices
    pub fn get_all_devices(&self) -> Result<Vec<Device>, Error> {
        let dev_paths = self.create_proxy().get_all_devices()?;
        let devs = dev_paths
            .iter()
            .map(|e| Device::new(&self.connection, e))
            .filter_map(Result::ok)
            .collect();
        Ok(devs)
    }

    /// Reloads NetworkManager by the given scope
    pub fn reload(&self, scope: ReloadScope) -> Result<(), Error> {
        Ok(self.create_proxy().reload(scope as u32)?)
    }

    pub fn networking_enabled(&self) -> Result<bool, Error> {
        Ok(self.create_proxy().networking_enabled()?)
    }

    pub fn wireless_enabled(&self) -> Result<bool, Error> {
        Ok(self.create_proxy().wireless_enabled()?)
    }

    pub fn wireless_hardware_enabled(&self) -> Result<bool, Error> {
        Ok(self.create_proxy().wireless_hardware_enabled()?)
    }

    /// Shows if NetworkManager is currently starting up
    pub fn startup(&self) -> Result<bool, Error> {
        Ok(self.create_proxy().startup()?)
    }
}
