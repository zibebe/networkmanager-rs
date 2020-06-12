use crate::dbus_api::DBusAccessor;

pub struct Dhcp6Config<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub trait Dhcp6 {}

impl<'a> Dhcp6Config<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Dhcp6Config { dbus_accessor }
    }
}
