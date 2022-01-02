use super::BridgeDevice;
use super::Device;
use crate::dbus_api::DBusAccessor;
use crate::gen::OrgFreedesktopNetworkManagerDeviceBridge;
use crate::Error;

pub trait Bridge {
    fn hw_address(&self) -> Result<String, Error>;
    fn carrier(&self) -> Result<bool, Error>;
    fn slaves(&self) -> Result<Vec<Device>, Error>;
}

impl<'a> Bridge for BridgeDevice<'a> {
    fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }

    fn carrier(&self) -> Result<bool, Error> {
        Ok(proxy!(self).carrier()?)
    }

    fn slaves(&self) -> Result<Vec<Device>, Error> {
        let paths = proxy!(self).slaves()?;
        let mut vec = Vec::new();
        for slave_path in paths {
            vec.push(Device::new(DBusAccessor::new(
                self.dbus_accessor.connection,
                &self.dbus_accessor.bus,
                &slave_path,
            ))?)
        }
        Ok(vec)
    }
}
