use super::BondDevice;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDeviceBond;

const NM_DEVICE_INTERFACE_FLAG_LOWER_UP: u32 = 0x2;

pub trait Bond {
    fn hw_address(&self) -> Result<String, Error>;
    /// Indicates whether the physical carrier is found (e.g. whether a cable is plugged in or not).
    fn carrier(&self) -> Result<bool, Error>;
    fn slaves(&self) -> Result<Vec<dbus::Path<'_>>, Error>;
}

impl<'a> Bond for BondDevice<'a> {
    fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }

    fn carrier(&self) -> Result<bool, Error> {
        // NOTE: DEPRECATED: Instead of org.freedesktop.NetworkManager.Device.Bond:Carrier
        // Check LOWER_UP flag in InterfaceFlags (see NM docs)
        use crate::gen::OrgFreedesktopNetworkManagerDevice;

        let flags = proxy!(self).interface_flags()?;
        Ok((flags & NM_DEVICE_INTERFACE_FLAG_LOWER_UP) != 0)
    }

    fn slaves(&self) -> Result<Vec<dbus::Path<'_>>, Error> {
        // NOTE: DEPRECATED. Use the "Ports" property in
        // "org.freedesktop.NetworkManager.Device" instead which exists
        // since version NetworkManager 1.34.0.
        use crate::gen::OrgFreedesktopNetworkManagerDevice;

        Ok(proxy!(self).ports()?)
    }
}
