use crate::dbus_api::DBusAccessor;

pub struct Ip6Config<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub trait Ip6 {}

impl<'a> Ip6Config<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Ip6Config { dbus_accessor }
    }
}
