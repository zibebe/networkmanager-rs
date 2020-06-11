use crate::dbus_api::DBusAccessor;
use crate::gen::OrgFreedesktopNetworkManagerAccessPoint;
use crate::Error;

pub struct AccessPoint<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> AccessPoint<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        AccessPoint { dbus_accessor }
    }
    pub fn flags(&self) -> Result<u32, Error> {
        Ok(proxy!(self).flags()?)
    }
    pub fn wpa_flags(&self) -> Result<u32, Error> {
        Ok(proxy!(self).wpa_flags()?)
    }
    pub fn rsn_flags(&self) -> Result<u32, Error> {
        Ok(proxy!(self).rsn_flags()?)
    }
    pub fn ssid(&self) -> Result<String, Error> {
        let utf8 = proxy!(self).ssid()?;
        Ok(std::str::from_utf8(&utf8).unwrap().to_owned())
    }
    pub fn frequency(&self) -> Result<u32, Error> {
        Ok(proxy!(self).frequency()?)
    }
    pub fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }
    pub fn mode(&self) -> Result<u32, Error> {
        Ok(proxy!(self).mode()?)
    }
    pub fn max_bitrate(&self) -> Result<u32, Error> {
        Ok(proxy!(self).max_bitrate()?)
    }
    pub fn strength(&self) -> Result<u8, Error> {
        Ok(proxy!(self).strength()?)
    }
    pub fn last_seen(&self) -> Result<i32, Error> {
        Ok(proxy!(self).last_seen()?)
    }
}
