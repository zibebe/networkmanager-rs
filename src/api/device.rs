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
pub enum DeviceKind {
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
    kind: DeviceKind,
}

impl<'a> Device<'a> {
    pub(super) fn new(dbus_connector: &'a DBusConnector, dbus_path: &str) -> Result<Self, Error> {
        let mut dev = Device {
            dbus_connector,
            dbus_path: dbus_path.to_owned(),
            kind: DeviceKind::Dummy,
        };
        dev.kind = Device::device_type(&dev)?;
        Ok(dev)
    }

    pub fn device_type(&self) -> Result<DeviceKind, Error> {
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
        match self.kind {
            DeviceKind::Unknown => Ok(OrgFreedesktopNetworkManagerDevice::hw_address(&proxy)?),
            DeviceKind::Generic => Ok(OrgFreedesktopNetworkManagerDeviceGeneric::hw_address(
                &proxy,
            )?),
            DeviceKind::Ethernet => {
                Ok(OrgFreedesktopNetworkManagerDeviceWired::hw_address(&proxy)?)
            }
            DeviceKind::WiFi => Ok(OrgFreedesktopNetworkManagerDeviceWireless::hw_address(
                &proxy,
            )?),
            DeviceKind::Dummy => Ok(OrgFreedesktopNetworkManagerDeviceDummy::hw_address(&proxy)?),
            _ => Err(Error::UnsupportedDevice),
        }
    }

    pub fn access_points(&self) -> Result<Vec<String>, Error> {
        match self.kind {
            DeviceKind::WiFi => Ok(Vec::new()),
            _ => Err(Error::UnsupportedMethod),
        }
    }
}
