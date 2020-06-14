use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDHCP6Config;

pub struct Dhcp6Config<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub trait Dhcp6 {
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

impl<'a> Dhcp6Config<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Dhcp6Config { dbus_accessor }
    }
}

impl<'a> Dhcp6 for Dhcp6Config<'a> {
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
