use super::HsrDevice;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDeviceHsr;

pub trait Hsr {
    /// The device's port1 device.
    fn port1(&self) -> Result<dbus::Path<'static>, Error>;
    /// The device's port2 device.
    fn port2(&self) -> Result<dbus::Path<'static>, Error>;
    /// The supervision MAC address.
    fn supervision_address(&self) -> Result<String, Error>;
    /// The last byte of the supervision address.
    fn multicast_spec(&self) -> Result<u8, Error>;
    /// Whether the PRP protocol is used or not.
    fn prp(&self) -> Result<bool, Error>;
}

impl<'a> Hsr for HsrDevice<'a> {
    fn port1(&self) -> Result<dbus::Path<'static>, Error> {
        Ok(proxy!(self).port1()?)
    }

    fn port2(&self) -> Result<dbus::Path<'static>, Error> {
        Ok(proxy!(self).port2()?)
    }

    fn supervision_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).supervision_address()?)
    }

    fn multicast_spec(&self) -> Result<u8, Error> {
        Ok(proxy!(self).multicast_spec()?)
    }

    fn prp(&self) -> Result<bool, Error> {
        Ok(proxy!(self).prp()?)
    }
}
