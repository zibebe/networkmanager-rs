use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerIP6Config;

pub struct Ip6Config<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> Ip6Config<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Ip6Config { dbus_accessor }
    }

    pub fn addresses(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>)>, Error> {
        Ok(proxy!(self).addresses()?)
    }
    pub fn address_data(
        &self,
    ) -> Result<
        Vec<
            std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<(dyn dbus::arg::RefArg + 'static)>>,
            >,
        >,
        Error,
    > {
        Ok(proxy!(self).address_data()?)
    }
    pub fn gateway(&self) -> Result<String, Error> {
        Ok(proxy!(self).gateway()?)
    }
    pub fn routes(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>, u32)>, Error> {
        Ok(proxy!(self).routes()?)
    }
    pub fn route_data(
        &self,
    ) -> Result<
        Vec<
            std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<(dyn dbus::arg::RefArg + 'static)>>,
            >,
        >,
        Error,
    > {
        Ok(proxy!(self).route_data()?)
    }
    pub fn nameservers(&self) -> Result<Vec<Vec<u8>>, Error> {
        Ok(proxy!(self).nameservers()?)
    }
    pub fn domains(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).domains()?)
    }
    pub fn searches(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).searches()?)
    }
    pub fn dns_options(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).dns_options()?)
    }
    pub fn dns_priority(&self) -> Result<i32, Error> {
        Ok(proxy!(self).dns_priority()?)
    }
}
