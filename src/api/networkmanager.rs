use super::dbus_api::DBusApi;
use super::device::Device;
use super::errors::Error;
use super::gen::OrgFreedesktopNetworkManager;
use super::types::ReloadFlag;

const NETWORK_MANAGER_BUS: &str = "org.freedesktop.NetworkManager";
const NETWORK_MANAGER_PATH: &str = "/org/freedesktop/NetworkManager";

pub struct NetworkManager {
    dbus_api: DBusApi,
}

impl NetworkManager {
    pub fn new() -> Result<Self, Error> {
        Ok(NetworkManager {
            dbus_api: DBusApi::new(NETWORK_MANAGER_BUS)?,
        })
    }

    fn paths_to_devices(&self, dbus_paths: Vec<dbus::Path>) -> Result<Vec<Device>, Error> {
        let mut res = Vec::new();
        for path in dbus_paths {
            res.push(Device::new(&self.dbus_api, &path)?);
        }
        Ok(res)
    }

    /// Returns only realized network devices
    pub fn get_devices(&self) -> Result<Vec<Device>, Error> {
        let dev_paths = self
            .dbus_api
            .create_proxy(NETWORK_MANAGER_PATH)
            .get_devices()?;
        Ok(self.paths_to_devices(dev_paths)?)
    }

    /// Returns all the network devices
    pub fn get_all_devices(&self) -> Result<Vec<Device>, Error> {
        let dev_paths = self
            .dbus_api
            .create_proxy(NETWORK_MANAGER_PATH)
            .get_all_devices()?;
        Ok(self.paths_to_devices(dev_paths)?)
    }

    /// Reloads NetworkManager by the given scope
    pub fn reload(&self, flags: ReloadFlag) -> Result<(), Error> {
        Ok(self
            .dbus_api
            .create_proxy(NETWORK_MANAGER_PATH)
            .reload(flags as u32)?)
    }

    pub fn networking_enabled(&self) -> Result<bool, Error> {
        Ok(self
            .dbus_api
            .create_proxy(NETWORK_MANAGER_PATH)
            .networking_enabled()?)
    }

    pub fn wireless_enabled(&self) -> Result<bool, Error> {
        Ok(self
            .dbus_api
            .create_proxy(NETWORK_MANAGER_PATH)
            .wireless_enabled()?)
    }

    pub fn wireless_hardware_enabled(&self) -> Result<bool, Error> {
        Ok(self
            .dbus_api
            .create_proxy(NETWORK_MANAGER_PATH)
            .wireless_hardware_enabled()?)
    }

    /// Shows if NetworkManager is currently starting up
    pub fn startup(&self) -> Result<bool, Error> {
        Ok(self.dbus_api.create_proxy(NETWORK_MANAGER_PATH).startup()?)
    }
}
