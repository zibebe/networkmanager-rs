use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerIP4Config;

pub struct Ip4Config<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub trait Ip4 {
    fn addresses(&self) -> Result<Vec<Vec<u32>>, Error>;
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
    fn routes(&self) -> Result<Vec<Vec<u32>>, Error>;
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
    fn nameservers(&self) -> Result<Vec<u32>, Error>;
    fn nameserver_data(
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
    fn domains(&self) -> Result<Vec<String>, Error>;
    fn searches(&self) -> Result<Vec<String>, Error>;
    fn dns_options(&self) -> Result<Vec<String>, Error>;
    fn dns_priority(&self) -> Result<i32, Error>;
    fn wins_servers(&self) -> Result<Vec<u32>, Error>;
    fn wins_server_data(&self) -> Result<Vec<String>, Error>;
}

impl<'a> Ip4 for Ip4Config<'a> {
    fn addresses(&self) -> Result<Vec<Vec<u32>>, Error> {
        Ok(proxy!(self).addresses()?)
    }
    fn address_data(
        &self,
    ) -> Result<
        Vec<
            std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
            >,
        >,
        Error,
    > {
        Ok(proxy!(self).address_data()?)
    }
    fn gateway(&self) -> Result<String, Error> {
        Ok(proxy!(self).gateway()?)
    }
    fn routes(&self) -> Result<Vec<Vec<u32>>, Error> {
        Ok(proxy!(self).routes()?)
    }
    fn route_data(
        &self,
    ) -> Result<
        Vec<
            std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
            >,
        >,
        Error,
    > {
        Ok(proxy!(self).route_data()?)
    }
    fn nameservers(&self) -> Result<Vec<u32>, Error> {
        Ok(proxy!(self).nameservers()?)
    }
    fn nameserver_data(
        &self,
    ) -> Result<
        Vec<
            std::collections::HashMap<
                String,
                dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
            >,
        >,
        Error,
    > {
        Ok(proxy!(self).nameserver_data()?)
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
    fn wins_servers(&self) -> Result<Vec<u32>, Error> {
        Ok(proxy!(self).wins_servers()?)
    }
    fn wins_server_data(&self) -> Result<Vec<String>, Error> {
        Ok(proxy!(self).wins_server_data()?)
    }
}

impl<'a> Ip4Config<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Ip4Config { dbus_accessor }
    }
}
