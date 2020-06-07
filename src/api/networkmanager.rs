use super::dbus_api::DBusApi;
use super::devices::Device;
use super::errors::Error;
use super::gen::OrgFreedesktopNetworkManager;
use super::types::ReloadFlag;

const NETWORK_MANAGER_BUS: &str = "org.freedesktop.NetworkManager";
const NETWORK_MANAGER_PATH: &str = "/org/freedesktop/NetworkManager";

pub struct NetworkManager {
    dbus_api: DBusApi,
    dbus_path: &'static str,
}

impl NetworkManager {
    pub fn new() -> Result<Self, Error> {
        Ok(NetworkManager {
            dbus_api: DBusApi::new(NETWORK_MANAGER_BUS)?,
            dbus_path: NETWORK_MANAGER_PATH,
        })
    }

    fn paths_to_devices(&self, paths: Vec<dbus::Path>) -> Result<Vec<Device>, Error> {
        let mut res = Vec::new();
        for path in paths {
            res.push(Device::new(&self.dbus_api, &path)?);
        }
        Ok(res)
    }

    fn path_to_device(&self, path: dbus::Path) -> Result<Device, Error> {
        Ok(Device::new(&self.dbus_api, &path)?)
    }

    /// Reloads NetworkManager by the given scope
    pub fn reload(&self, flags: ReloadFlag) -> Result<(), Error> {
        Ok(proxy!(self).reload(flags as u32)?)
    }

    /// Returns only realized network devices
    pub fn get_devices(&self) -> Result<Vec<Device>, Error> {
        let dev_paths = proxy!(self).get_devices()?;
        Ok(self.paths_to_devices(dev_paths)?)
    }

    /// Returns all the network devices
    pub fn get_all_devices(&self) -> Result<Vec<Device>, Error> {
        let dev_paths = proxy!(self).get_all_devices()?;
        Ok(self.paths_to_devices(dev_paths)?)
    }

    pub fn get_device_by_ip_iface(&self, iface: &str) -> Result<Device, Error> {
        let dev_path = proxy!(self).get_device_by_ip_iface(iface)?;
        Ok(self.path_to_device(dev_path)?)
    }

    pub fn networking_enabled(&self) -> Result<bool, Error> {
        Ok(proxy!(self).networking_enabled()?)
    }

    pub fn wireless_enabled(&self) -> Result<bool, Error> {
        Ok(proxy!(self).wireless_enabled()?)
    }

    pub fn wireless_hardware_enabled(&self) -> Result<bool, Error> {
        Ok(proxy!(self).wireless_hardware_enabled()?)
    }

    /// Shows if NetworkManager is currently starting up
    pub fn startup(&self) -> Result<bool, Error> {
        Ok(proxy!(self).startup()?)
    }
}
