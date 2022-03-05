use crate::connection::Connection;
use crate::dbus_api::DBusAccessor;
use crate::devices::Device;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManager;
use crate::types::ReloadFlag;
use dbus::blocking::Connection as DBusConnection;

use num_traits::ToPrimitive;

const NETWORK_MANAGER_BUS: &str = "org.freedesktop.NetworkManager";
const NETWORK_MANAGER_PATH: &str = "/org/freedesktop/NetworkManager";

pub struct NetworkManager<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> NetworkManager<'a> {
    pub fn new(dbus_connection: &'a DBusConnection) -> Self {
        NetworkManager {
            dbus_accessor: DBusAccessor::new(
                dbus_connection,
                NETWORK_MANAGER_BUS,
                NETWORK_MANAGER_PATH,
            ),
        }
    }

    fn paths_to_connections(
        &self,
        paths: Vec<dbus::Path<'_>>,
    ) -> Result<Vec<Connection<'_>>, Error> {
        let mut res = Vec::new();
        for path in paths {
            res.push(Connection::new(DBusAccessor::new(
                self.dbus_accessor.connection,
                &self.dbus_accessor.bus,
                &path,
            )));
        }
        Ok(res)
    }

    fn path_to_connection(&self, path: dbus::Path<'_>) -> Result<Connection<'_>, Error> {
        Ok(Connection::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }

    fn paths_to_devices(&self, paths: Vec<dbus::Path<'_>>) -> Result<Vec<Device<'_>>, Error> {
        let mut res = Vec::new();
        for path in paths {
            res.push(Device::new(DBusAccessor::new(
                self.dbus_accessor.connection,
                &self.dbus_accessor.bus,
                &path,
            ))?);
        }
        Ok(res)
    }

    fn path_to_device(&self, path: dbus::Path<'_>) -> Result<Device<'_>, Error> {
        Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        ))
    }

    /// Reloads NetworkManager by the given scope
    pub fn reload(&self, flags: ReloadFlag) -> Result<(), Error> {
        match ToPrimitive::to_u32(&flags) {
            Some(x) => Ok(proxy!(self).reload(x)?),
            None => Err(Error::UnsupportedType),
        }
    }

    /// Returns only realized network devices
    pub fn get_devices(&self) -> Result<Vec<Device<'_>>, Error> {
        let dev_paths = proxy!(self).get_devices()?;
        self.paths_to_devices(dev_paths)
    }

    /// Returns all the network devices
    pub fn get_all_devices(&self) -> Result<Vec<Device<'_>>, Error> {
        let dev_paths = proxy!(self).get_all_devices()?;
        self.paths_to_devices(dev_paths)
    }

    pub fn get_device_by_ip_iface(&self, iface: &str) -> Result<Device<'_>, Error> {
        let dev_path = proxy!(self).get_device_by_ip_iface(iface)?;
        self.path_to_device(dev_path)
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

    /// Return all active connections
    pub fn get_active_connections(&self) -> Result<Vec<Connection<'_>>, Error> {
        let paths = proxy!(self).active_connections()?;
        self.paths_to_connections(paths)
    }

    /// Return primary connection
    pub fn primary_connection(&self) -> Result<Connection<'_>, Error> {
        let path = proxy!(self).primary_connection()?;
        self.path_to_connection(path)
    }

    /// return primaty connection type
    pub fn primary_connection_type(&self) -> Result<String, Error> {
        Ok(proxy!(self).primary_connection_type()?)
    }
}
