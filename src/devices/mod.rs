mod any;
mod wired;
mod wireless;

pub use self::any::Any;
pub use self::wired::Wired;
pub use self::wireless::Wireless;
use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDevice;
use crate::types::DeviceType;
use num_traits::FromPrimitive;

pub enum Device<'a> {
    WiFi(WiFiDevice<'a>),
    Ethernet(EthernetDevice<'a>),
    Generic(GenericDevice<'a>),
}

pub struct GenericDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct WiFiDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct EthernetDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> Device<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Result<Self, Error> {
        let dev_type = dbus_accessor.create_proxy().device_type()?;
        match FromPrimitive::from_u32(dev_type) {
            Some(x) => match x {
                DeviceType::Wifi => Ok(Device::WiFi(WiFiDevice { dbus_accessor })),
                DeviceType::Ethernet => Ok(Device::Ethernet(EthernetDevice { dbus_accessor })),
                DeviceType::Generic => Ok(Device::Generic(GenericDevice { dbus_accessor })),
                _ => Err(Error::UnsupportedDevice),
            },
            None => Err(Error::UnsupportedType),
        }
    }
}
