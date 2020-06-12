use crate::dbus_api::DBusAccessor;

pub struct Dhcp4Config<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub trait Dhcp4 {}

impl<'a> Dhcp4Config<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Dhcp4Config { dbus_accessor }
    }
}
