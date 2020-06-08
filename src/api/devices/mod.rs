mod any;
mod wired;
mod wireless;

use crate::api::errors::Error;
use crate::types::DeviceType;

pub use self::any::Any;
pub use self::wired::Wired;
pub use self::wireless::Wireless;
use super::dbus::DBusObject;
use super::gen::OrgFreedesktopNetworkManagerDevice;
use num_traits::FromPrimitive;

pub enum Device<'a> {
    WiFi(WiFiDevice<'a>),
    Ethernet(EthernetDevice<'a>),
    Generic(GenericDevice<'a>),
}

pub struct GenericDevice<'a> {
    dbus_object: DBusObject<'a>,
}

pub struct WiFiDevice<'a> {
    dbus_object: DBusObject<'a>,
}

pub struct EthernetDevice<'a> {
    dbus_object: DBusObject<'a>,
}

impl<'a> Device<'a> {
    pub(crate) fn new(dbus_object: DBusObject<'a>) -> Result<Self, Error> {
        let dev_type = dbus_object.create_proxy().device_type()?;
        match FromPrimitive::from_u32(dev_type) {
            Some(x) => match x {
                DeviceType::Wifi => Ok(Device::WiFi(WiFiDevice { dbus_object })),
                DeviceType::Ethernet => Ok(Device::Ethernet(EthernetDevice { dbus_object })),
                DeviceType::Generic => Ok(Device::Generic(GenericDevice { dbus_object })),
                _ => Err(Error::UnsupportedDevice),
            },
            None => Err(Error::UnsupportedType),
        }
    }
}
