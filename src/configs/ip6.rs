use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerIP6Config;

pub struct Ip6Config<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub trait Ip6 {
    fn addresses(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>)>, Error>;
    fn address_data(
        &self,
    ) -> Result<
        Vec<
            ::std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
            >,
        >,
        Error,
    >;
    fn gateway(&self) -> Result<String, Error>;
    fn routes(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>, u32)>, Error>;
    fn route_data(
        &self,
    ) -> Result<
        Vec<
            ::std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
            >,
        >,
        Error,
    >;
    fn nameservers(&self) -> Result<Vec<Vec<u8>>, Error>;
    fn domains(&self) -> Result<Vec<String>, Error>;
    fn searches(&self) -> Result<Vec<String>, Error>;
    fn dns_options(&self) -> Result<Vec<String>, Error>;
    fn dns_priority(&self) -> Result<i32, Error>;
}

impl<'a> Ip6Config<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Ip6Config { dbus_accessor }
    }
}

impl<'a> Ip6 for Ip6Config<'a> {
    fn addresses(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>)>, Error> {
        Ok(proxy!(self).addresses()?)
    }
    fn address_data(
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
    fn gateway(&self) -> Result<String, Error> {
        Ok(proxy!(self).gateway()?)
    }
    fn routes(&self) -> Result<Vec<(Vec<u8>, u32, Vec<u8>, u32)>, Error> {
        Ok(proxy!(self).routes()?)
    }
    fn route_data(
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
    fn nameservers(&self) -> Result<Vec<Vec<u8>>, Error> {
        Ok(proxy!(self).nameservers()?)
    }
    fn domains(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).domains()?)
    }
    fn searches(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).searches()?)
    }
    fn dns_options(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).dns_options()?)
    }
    fn dns_priority(&self) -> Result<i32, Error> {
        Ok(proxy!(self).dns_priority()?)
    }
}
