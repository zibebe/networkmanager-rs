use super::dbus_connector::DBusConnector;
use super::gen::{
    OrgFreedesktopNetworkManagerDevice, OrgFreedesktopNetworkManagerDeviceDummy,
    OrgFreedesktopNetworkManagerDeviceGeneric, OrgFreedesktopNetworkManagerDeviceWired,
    OrgFreedesktopNetworkManagerDeviceWireless,
};
use super::Error;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(Debug, PartialEq, FromPrimitive)]
pub enum DeviceType {
    Unknown,
    Ethernet,
    WiFi,
    Unused1,
    Unused2,
    Bt,
    OlcpMesh,
    WiMax,
    Modem,
    Infiniband,
    Bond,
    Vlan,
    Adsl,
    Bridge,
    Generic,
    Team,
    TunTap,
    IpTunnel,
    MacVlan,
    VxLan,
    Veth,
    Macsec,
    Dummy,
    PPP,
    OvsInterface,
    OvsPort,
    OvsBridge,
    Wpan,
    SixLoWpan,
    WireGuard,
    WiFiP2p,
    Vrf,
}

pub struct Device<'a> {
    dbus_connector: &'a DBusConnector,
    dbus_path: String,
    _type: DeviceType,
}

impl<'a> Device<'a> {
    pub(super) fn new(dbus_connector: &'a DBusConnector, dbus_path: &str) -> Result<Self, Error> {
        let mut dev = Device {
            dbus_connector,
            dbus_path: dbus_path.to_owned(),
            _type: DeviceType::Dummy,
        };
        dev._type = Device::device_type(&dev)?;
        Ok(dev)
    }

    pub fn device_type(&self) -> Result<DeviceType, Error> {
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
        let proxy = self.dbus_connector.create_proxy(&self.dbus_path);
        match self._type {
            DeviceType::Unknown => Ok(OrgFreedesktopNetworkManagerDevice::hw_address(&proxy)?),
            DeviceType::Generic => Ok(OrgFreedesktopNetworkManagerDeviceGeneric::hw_address(
                &proxy,
            )?),
            DeviceType::Ethernet => {
                Ok(OrgFreedesktopNetworkManagerDeviceWired::hw_address(&proxy)?)
            }
            DeviceType::WiFi => Ok(OrgFreedesktopNetworkManagerDeviceWireless::hw_address(
                &proxy,
            )?),
            DeviceType::Dummy => Ok(OrgFreedesktopNetworkManagerDeviceDummy::hw_address(&proxy)?),
            _ => Err(Error::UnsupportedDevice),
        }
    }

    pub fn access_points(&self) -> Result<Vec<String>, Error> {
        match self._type {
            DeviceType::WiFi => Ok(Vec::new()),
            _ => Err(Error::UnsupportedMethod),
        }
    }
}
