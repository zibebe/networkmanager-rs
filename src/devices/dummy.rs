use super::DummyDevice;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDeviceDummy;

pub trait Dummy {
    fn hw_address(&self) -> Result<String, Error>;
}

impl<'a> Dummy for DummyDevice<'a> {
    fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }
}
