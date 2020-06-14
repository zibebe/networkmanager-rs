use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDHCP4Config;

pub struct Dhcp4Config<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub trait Dhcp4 {
    fn options(
        &self,
    ) -> Result<
        ::std::collections::HashMap<
            String,
            dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
        >,
        Error,
    >;
}

impl<'a> Dhcp4 for Dhcp4Config<'a> {
    fn options(
        &self,
    ) -> Result<
        std::collections::HashMap<
            String,
            dbus::arg::Variant<std::boxed::Box<(dyn dbus::arg::RefArg + 'static)>>,
        >,
        Error,
    > {
        Ok(proxy!(self).options()?)
    }
}

impl<'a> Dhcp4Config<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Dhcp4Config { dbus_accessor }
    }
}
