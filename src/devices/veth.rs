// Virtual Ethernet Device

use super::Device;
use super::VethDevice;
use crate::dbus_api::DBusAccessor;
use crate::gen::OrgFreedesktopNetworkManagerDeviceVeth;
use crate::Error;

pub trait Veth {
    fn peer(&self) -> Result<Device, Error>;
}

impl<'a> Veth for VethDevice<'a> {
    fn peer(&self) -> Result<Device, Error> {
        let path = proxy!(self).peer()?;
        Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        ))
    }
}
