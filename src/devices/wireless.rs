use crate::accesspoint::AccessPoint;
use crate::devices::WiFiDevice;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDeviceWireless;

pub trait Wireless {
    fn get_access_points(&self) -> Result<Vec<AccessPoint>, Error>;
    fn get_all_access_points(&self) -> Result<Vec<AccessPoint>, Error>;
    fn request_scan(
        &self,
        options: ::std::collections::HashMap<&str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>,
    ) -> Result<(), Error>;
    fn hw_address(&self) -> Result<String, Error>;
    fn perm_hw_address(&self) -> Result<String, Error>;
    fn mode(&self) -> Result<u32, Error>;
    fn bitrate(&self) -> Result<u32, Error>;
    fn access_points(&self) -> Result<Vec<AccessPoint>, Error>;
    fn active_access_point(&self) -> Result<AccessPoint, Error>;
    fn wireless_capabilities(&self) -> Result<u32, Error>;
    fn last_scan(&self) -> Result<i64, Error>;
}

impl<'a> Wireless for WiFiDevice<'a> {
    fn request_scan(
        &self,
        _options: std::collections::HashMap<&str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>,
    ) -> Result<(), Error> {
        todo!()
    }
    fn get_access_points(&self) -> Result<Vec<AccessPoint>, Error> {
        todo!()
    }
    fn get_all_access_points(&self) -> Result<Vec<AccessPoint>, Error> {
        todo!()
    }
    fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }
    fn perm_hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).perm_hw_address()?)
    }
    fn mode(&self) -> Result<u32, Error> {
        Ok(proxy!(self).mode()?)
    }
    fn bitrate(&self) -> Result<u32, Error> {
        Ok(proxy!(self).bitrate()?)
    }
    fn access_points(&self) -> Result<Vec<AccessPoint>, Error> {
        todo!()
    }
    fn active_access_point(&self) -> Result<AccessPoint, Error> {
        todo!()
    }
    fn wireless_capabilities(&self) -> Result<u32, Error> {
        Ok(proxy!(self).wireless_capabilities()?)
    }
    fn last_scan(&self) -> Result<i64, Error> {
        Ok(proxy!(self).last_scan()?)
    }
}
