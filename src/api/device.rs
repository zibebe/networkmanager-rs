use std::time::Duration;

use dbus::blocking::{Connection as DBusConnection, Proxy as DBusProxy};

use super::gen::{OrgFreedesktopNetworkManagerDevice};
use super::{Error, DBUS_TIMEOUT_MS};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum Type {
    Unknown,
    Ethernet,
    WiFi,
    Unused1,
    Unused2,
    Bt,
    OlpcMesh,
    Wimax,
    Modem,
    Infiniband,
    Bond,
    Vlan,
    Adsl,
    Bridge,
    Generic,
    Team,
    Tun,
    IpTunnel,
    Macvlan,
    Vxlan,
    Veth,
    Macsec,
    Dummy,
}

pub struct Device<'a> {
    dbus_connection: &'a DBusConnection,
    path: String,
    _type: Type,
}

impl<'a> Device<'a> {
    pub(super) fn new(dbus_connection: &'a DBusConnection, path: &str) -> Result<Self, Error> {
        let mut dev = Device {
            dbus_connection,
            path: path.to_owned(),
            _type: Type::Dummy,
        };
        dev._type = Device::device_type(&dev)?;
        Ok(dev)
    }

    fn create_proxy(&self) -> DBusProxy<'_, &DBusConnection> {
        self.dbus_connection.with_proxy(
            "org.freedesktop.NetworkManager",
            &self.path,
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }

    pub fn device_type(&self) -> Result<Type, Error> {
        let proxy = self.create_proxy();
        let dev_type = proxy.device_type()?;
        match FromPrimitive::from_u32(dev_type) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedDevice),
        }     
    }

    pub fn autoconnect(&self) -> Result<bool, Error> {
        Ok(self.create_proxy().autoconnect()?)
    }

    pub fn hw_address(&self) -> Result<String, Error> {
        Ok(self.create_proxy().hw_address()?)
    }

    pub fn access_points(&self) -> Result<Vec<String>, Error> {
        match self._type {
            Type::WiFi => Ok(Vec::new()),
            _ => Err(Error::UnsupportedMethod),
        }
    }
}
