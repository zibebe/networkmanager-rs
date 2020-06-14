use super::EthernetDevice;
use crate::gen::OrgFreedesktopNetworkManagerDeviceWired;
use crate::Error;

pub trait Wired {
    fn hw_address(&self) -> Result<String, Error>;
    fn perm_hw_address(&self) -> Result<String, Error>;
    fn speed(&self) -> Result<u32, Error>;
    fn s390_subchannels(&self) -> Result<Vec<String>, Error>;
    fn carrier(&self) -> Result<bool, Error>;
}

impl<'a> Wired for EthernetDevice<'a> {
    fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }
    fn perm_hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).perm_hw_address()?)
    }
    fn speed(&self) -> Result<u32, Error> {
        Ok(proxy!(self).speed()?)
    }
    fn s390_subchannels(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).s390_subchannels()?)
    }
    fn carrier(&self) -> Result<bool, Error> {
        Ok(proxy!(self).carrier()?)
    }
}
