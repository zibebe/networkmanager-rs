use crate::api::devices::Device;
use crate::api::gen::OrgFreedesktopNetworkManagerDeviceWired;
use crate::Error;

pub trait Wired {
    fn perm_hw_address(&self) -> Result<String, Error>;
    fn speed(&self) -> Result<u32, Error>;
    fn s390_subchannels(&self) -> Result<Vec<String>, Error>;
    fn carrier(&self) -> Result<bool, Error>;
}

impl<'a> Wired for Device<'a> {
    fn perm_hw_address(&self) -> Result<String, Error> {
        todo!()
    }
    fn speed(&self) -> Result<u32, Error> {
        Ok(proxy!(self).speed()?)
    }
    fn s390_subchannels(&self) -> Result<Vec<String>, Error> {
        todo!()
    }
    fn carrier(&self) -> Result<bool, Error> {
        todo!()
    }
}
