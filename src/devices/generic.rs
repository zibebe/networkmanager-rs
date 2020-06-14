use super::GenericDevice;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDeviceGeneric;

pub trait Generic {
    fn hw_address(&self) -> Result<String, Error>;
    fn type_description(&self) -> Result<String, Error>;
}

impl<'a> Generic for GenericDevice<'a> {
    fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }
    fn type_description(&self) -> Result<String, Error> {
        Ok(proxy!(self).type_description()?)
    }
}
