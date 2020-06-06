use super::dbus_api::DBusApi;
use super::errors::Error;
use super::gen::{
    OrgFreedesktopNetworkManagerDevice, OrgFreedesktopNetworkManagerDeviceDummy,
    OrgFreedesktopNetworkManagerDeviceGeneric, OrgFreedesktopNetworkManagerDeviceWired,
    OrgFreedesktopNetworkManagerDeviceWireless,
};
use super::types::DeviceType;

use num_traits::FromPrimitive;
use std::collections::HashMap;

pub struct Device<'a> {
    dbus_api: &'a DBusApi,
    dbus_path: String,
    _type: DeviceType,
}

impl<'a> Device<'a> {
    pub(super) fn new(dbus_api: &'a DBusApi, dbus_path: &str) -> Result<Self, Error> {
        let mut dev = Device {
            dbus_api,
            dbus_path: dbus_path.to_owned(),
            _type: DeviceType::Dummy,
        };
        dev._type = Device::device_type(&dev)?;
        Ok(dev)
    }

    // TODO: Mapping of the connection
    pub fn reapply(
        &self,
        connection: HashMap<&str, HashMap<&str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>>,
        version_id: u64,
        flags: u32,
    ) -> Result<(), Error> {
        Ok(self
            .dbus_api
            .create_proxy(&self.dbus_path)
            .reapply(connection, version_id, flags)?)
    }

    // TODO: Mapping of the result, flags to enum
    pub fn get_applied_connection(
        &self,
        flags: u32,
    ) -> Result<
        ((
            ::std::collections::HashMap<
                String,
                ::std::collections::HashMap<
                    String,
                    dbus::arg::Variant<Box<dyn dbus::arg::RefArg + 'static>>,
                >,
            >,
            u64,
        ),),
        Error,
    > {
        Ok((self
            .dbus_api
            .create_proxy(&self.dbus_path)
            .get_applied_connection(flags)?,))
    }

    pub fn disconnect(&self) -> Result<(), Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).disconnect()?)
    }

    pub fn delete(&self) -> Result<(), Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).delete()?)
    }

    pub fn udi(&self) -> Result<String, Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).udi()?)
    }

    pub fn interface(&self) -> Result<String, Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).interface()?)
    }

    pub fn ip_interface(&self) -> Result<String, Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).ip_interface()?)
    }

    pub fn driver(&self) -> Result<String, Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).driver()?)
    }

    pub fn driver_version(&self) -> Result<String, Error> {
        Ok(self
            .dbus_api
            .create_proxy(&self.dbus_path)
            .driver_version()?)
    }

    pub fn firmware_version(&self) -> Result<String, Error> {
        Ok(self
            .dbus_api
            .create_proxy(&self.dbus_path)
            .firmware_version()?)
    }

    // TODO: Map to enum
    pub fn capabilities(&self) -> Result<u32, Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).capabilities()?)
    }

    // TODO: Map to IP4Address
    pub fn ip4_address(&self) -> Result<u32, Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).ip4_address()?)
    }

    // TODO: Map to State enum
    pub fn state(&self) -> Result<u32, Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).state()?)
    }

    // TODO: Map Tuple to StateReason enum
    pub fn state_reason(&self) -> Result<(u32, u32), Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).state_reason()?)
    }

    pub fn device_type(&self) -> Result<DeviceType, Error> {
        let proxy = self.dbus_api.create_proxy(&self.dbus_path);
        let dev_type = proxy.device_type()?;
        match FromPrimitive::from_u32(dev_type) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedDevice),
        }
    }

    pub fn autoconnect(&self) -> Result<bool, Error> {
        Ok(self.dbus_api.create_proxy(&self.dbus_path).autoconnect()?)
    }

    pub fn hw_address(&self) -> Result<String, Error> {
        let proxy = self.dbus_api.create_proxy(&self.dbus_path);
        match self._type {
            DeviceType::Unknown => Ok(OrgFreedesktopNetworkManagerDevice::hw_address(&proxy)?),
            DeviceType::Generic => Ok(OrgFreedesktopNetworkManagerDeviceGeneric::hw_address(
                &proxy,
            )?),
            DeviceType::Ethernet => {
                Ok(OrgFreedesktopNetworkManagerDeviceWired::hw_address(&proxy)?)
            }
            DeviceType::WiFi => Ok(OrgFreedesktopNetworkManagerDeviceWireless::hw_address(
                &proxy,
            )?),
            DeviceType::Dummy => Ok(OrgFreedesktopNetworkManagerDeviceDummy::hw_address(&proxy)?),
            _ => Err(Error::UnsupportedDevice),
        }
    }

    pub fn speed(&self) -> Result<u32, Error> {
        match self._type {
            DeviceType::Ethernet => Ok(self.dbus_api.create_proxy(&self.dbus_path).speed()?),
            _ => Err(Error::UnsupportedMethod),
        }
    }

    pub fn access_points(&self) -> Result<Vec<String>, Error> {
        match self._type {
            DeviceType::WiFi => Ok(Vec::new()),
            _ => Err(Error::UnsupportedMethod),
        }
    }
}
