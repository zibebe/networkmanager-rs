use super::AdslDevice;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDevice;

const NM_DEVICE_INTERFACE_FLAG_CARRIER: u32 = 0x10000;

pub trait Adsl {
    /// Indicates whether the physical carrier is found.
    fn carrier(&self) -> Result<bool, Error>;
}

impl<'a> Adsl for AdslDevice<'a> {
    fn carrier(&self) -> Result<bool, Error> {
        let flags = proxy!(self).interface_flags()?;
        Ok((flags & NM_DEVICE_INTERFACE_FLAG_CARRIER) != 0)
    }
}
