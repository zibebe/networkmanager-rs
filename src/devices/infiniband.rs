use super::InfinibandDevice;
use crate::gen::OrgFreedesktopNetworkManagerDeviceInfiniband;
use crate::Error;

const NM_DEVICE_INTERFACE_FLAG_LOWER_UP: u32 = 0x2;

pub trait Infiniband {
    fn hw_address(&self) -> Result<String, Error>;
    fn carrier(&self) -> Result<bool, Error>;
}

impl<'a> Infiniband for InfinibandDevice<'a> {
    fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }

    fn carrier(&self) -> Result<bool, Error> {
        // NOTE: DEPRECATED:  DEPRECATED: check for the "lower-up"
        // flag in the "InterfaceFlags" property on the
        // "org.freedesktop.NetworkManager.Device" interface.
        use crate::gen::OrgFreedesktopNetworkManagerDevice;

        let flags = proxy!(self).interface_flags()?;
        Ok((flags & NM_DEVICE_INTERFACE_FLAG_LOWER_UP) != 0)
    }
}
