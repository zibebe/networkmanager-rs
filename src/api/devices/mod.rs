mod any;
mod wired;
mod wireless;

use crate::api::dbus_api::DBusApi;
use crate::api::errors::Error;
use crate::types::DeviceType;

pub use self::any::Any;
pub use self::wired::Wired;
pub use self::wireless::Wireless;

pub struct Device<'a> {
    dbus_api: &'a DBusApi,
    dbus_path: String,
    _type: DeviceType,
}

impl<'a> Device<'a> {
    pub(crate) fn new(dbus_api: &'a DBusApi, dbus_path: &str) -> Result<Self, Error> {
        let mut dev = Device {
            dbus_api,
            dbus_path: dbus_path.to_owned(),
            _type: DeviceType::Dummy,
        };
        dev._type = Any::device_type(&dev)?;
        Ok(dev)
    }
}
