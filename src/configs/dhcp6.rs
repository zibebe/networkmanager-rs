use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDHCP6Config;

pub struct Dhcp6Config<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> Dhcp6Config<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Dhcp6Config { dbus_accessor }
    }
    pub fn options(
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
