use super::Error;
use super::{Device, MacvlanDevice};
use crate::dbus_api::DBusAccessor;
use crate::gen::OrgFreedesktopNetworkManagerDeviceMacvlan;

/// MAC VLAN Device
pub trait Macvlan {
    /// The parent device
    fn parent(&self) -> Result<Device, Error>;
    /// The macvlan mode, one of "private", "vepa", "bridge", or "passthru".
    fn mode(&self) -> Result<String, Error>;
    /// Whether the device is blocked from going into promiscuous mode.
    fn no_promisc(&self) -> Result<bool, Error>;
    /// Whether the device is a macvtap.
    fn tap(&self) -> Result<bool, Error>;
}

impl<'a> Macvlan for MacvlanDevice<'a> {
    fn parent(&self) -> Result<Device, Error> {
        let parent_path = proxy!(self).parent()?;
        Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &parent_path,
        ))
    }

    fn mode(&self) -> Result<String, Error> {
        Ok(proxy!(self).mode()?)
    }

    fn no_promisc(&self) -> Result<bool, Error> {
        Ok(proxy!(self).no_promisc()?)
    }

    fn tap(&self) -> Result<bool, Error> {
        Ok(proxy!(self).tap()?)
    }
}
