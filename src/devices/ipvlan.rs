use super::Device;
use super::Error;
use super::IpVlanDevice;
use crate::dbus_api::DBusAccessor;
use crate::gen::OrgFreedesktopNetworkManagerDeviceIpvlan;

pub trait IpVlan {
    /// The parent device.
    fn parent(&self) -> Result<Device, Error>;
    /// The IPVLAN VEPA flag.
    fn vepa(&self) -> Result<bool, Error>;
    /// The IPVLAN mode, one of "l2", "l3", "l3s".
    fn mode(&self) -> Result<String, Error>;
    /// The IPVLAN private flag.
    fn private(&self) -> Result<bool, Error>;
}

impl<'a> IpVlan for IpVlanDevice<'a> {
    fn parent(&self) -> Result<Device, Error> {
        let parent_path = proxy!(self).parent()?;

        Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &parent_path,
        ))
    }

    fn vepa(&self) -> Result<bool, Error> {
        Ok(proxy!(self).vepa()?)
    }

    fn mode(&self) -> Result<String, Error> {
        Ok(proxy!(self).mode()?)
    }

    fn private(&self) -> Result<bool, Error> {
        Ok(proxy!(self).private()?)
    }
}
