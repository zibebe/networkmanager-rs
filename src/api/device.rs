use super::dbus_connector::DBusConnector;
use super::gen::OrgFreedesktopNetworkManagerDevice;
use super::Error;

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
    dbus_connector: &'a DBusConnector,
    dbus_path: String,
    _type: Type,
}

impl<'a> Device<'a> {
    pub(super) fn new(dbus_connector: &'a DBusConnector, dbus_path: &str) -> Result<Self, Error> {
        let mut dev = Device {
            dbus_connector,
            dbus_path: dbus_path.to_owned(),
            _type: Type::Dummy,
        };
        dev._type = Device::device_type(&dev)?;
        Ok(dev)
    }

    pub fn device_type(&self) -> Result<Type, Error> {
        let proxy = self.dbus_connector.create_proxy(&self.dbus_path);
        let dev_type = proxy.device_type()?;
        match FromPrimitive::from_u32(dev_type) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedDevice),
        }
    }

    pub fn autoconnect(&self) -> Result<bool, Error> {
        Ok(self
            .dbus_connector
            .create_proxy(&self.dbus_path)
            .autoconnect()?)
    }

    pub fn hw_address(&self) -> Result<String, Error> {
        Ok(self
            .dbus_connector
            .create_proxy(&self.dbus_path)
            .hw_address()?)
    }

    pub fn access_points(&self) -> Result<Vec<String>, Error> {
        match self._type {
            Type::WiFi => Ok(Vec::new()),
            _ => Err(Error::UnsupportedMethod),
        }
    }
}
