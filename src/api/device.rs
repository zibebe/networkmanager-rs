use super::accesspoint::AccessPoint;
use super::config::Config;
use super::connection::Connection;
use super::dbus_api::DBusApi;
use super::errors::Error;
use super::gen::{
    OrgFreedesktopNetworkManagerDevice, OrgFreedesktopNetworkManagerDeviceWired,
    OrgFreedesktopNetworkManagerDeviceWireless,
};
use super::types::*;

use num_traits::FromPrimitive;
use std::fmt::Debug;
pub struct Device<'a> {
    dbus_api: &'a DBusApi,
    dbus_path: String,
    _type: DeviceType,
}

impl<'a> Debug for Device<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("dbus_path", &self.dbus_path)
            .field("_type", &self._type)
            .finish()
    }
}

impl<'a> Device<'a> {
    pub(super) fn new(dbus_api: &'a DBusApi, dbus_path: &str) -> Result<Self, Error> {
        let mut dev = Device {
            dbus_api,
            dbus_path: dbus_path.to_owned(),
            _type: DeviceType::Dummy,
        };
        dev._type = Any::device_type(&dev)?;
        Ok(dev)
    }
}

pub trait Any {
    // fn reapply(
    //     &self,
    //     connection: ::std::collections::HashMap<
    //         &str,
    //         ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    //     >,
    //     version_id: u64,
    //     flags: u32,
    // ) -> Result<(), Error>;
    // fn get_applied_connection(
    //     &self,
    //     flags: u32,
    // ) -> Result<
    //     (
    //         ::std::collections::HashMap<
    //             String,
    //             ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
    //         >,
    //         u64,
    //     ),
    //     Error,
    // >;
    fn disconnect(&self) -> Result<(), Error>;
    fn delete(&self) -> Result<(), Error>;
    fn udi(&self) -> Result<String, Error>;
    fn interface(&self) -> Result<String, Error>;
    fn ip_interface(&self) -> Result<String, Error>;
    fn driver(&self) -> Result<String, Error>;
    fn driver_version(&self) -> Result<String, Error>;
    fn firmware_version(&self) -> Result<String, Error>;
    fn capabilities(&self) -> Result<Capability, Error>;
    fn ip4_address(&self) -> Result<u32, Error>;
    fn state(&self) -> Result<u32, Error>;
    fn state_reason(&self) -> Result<(u32, u32), Error>;
    fn active_connection(&self) -> Result<Connection, Error>;
    fn ip4_config(&self) -> Result<Config, Error>;
    fn dhcp4_config(&self) -> Result<Config, Error>;
    fn ip6_config(&self) -> Result<Config, Error>;
    fn dhcp6_config(&self) -> Result<Config, Error>;
    fn managed(&self) -> Result<bool, Error>;
    fn set_managed(&self, value: bool) -> Result<(), Error>;
    fn autoconnect(&self) -> Result<bool, Error>;
    fn set_autoconnect(&self, value: bool) -> Result<(), Error>;
    fn firmware_missing(&self) -> Result<bool, Error>;
    fn nm_plugin_missing(&self) -> Result<bool, Error>;
    fn device_type(&self) -> Result<DeviceType, Error>;
    fn available_connections(&self) -> Result<Vec<Connection>, Error>;
    fn physical_port_id(&self) -> Result<String, Error>;
    fn mtu(&self) -> Result<u32, Error>;
    fn metered(&self) -> Result<u32, Error>;
    // fn lldp_neighbors(
    //     &self,
    // ) -> Result<
    //     Vec<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>,
    //     Error,
    // >;
    fn real(&self) -> Result<bool, Error>;
    fn ip4_connectivity(&self) -> Result<u32, Error>;
    fn ip6_connectivity(&self) -> Result<u32, Error>;
    fn interface_flags(&self) -> Result<u32, Error>;
    fn hw_address(&self) -> Result<String, Error>;
}

impl<'a> Any for Device<'a> {
    fn disconnect(&self) -> Result<(), Error> {
        Ok(proxy!(self).disconnect()?)
    }
    fn delete(&self) -> Result<(), Error> {
        Ok(proxy!(self).delete()?)
    }
    fn udi(&self) -> Result<String, Error> {
        Ok(proxy!(self).udi()?)
    }
    fn interface(&self) -> Result<String, Error> {
        Ok(proxy!(self).interface()?)
    }
    fn ip_interface(&self) -> Result<String, Error> {
        Ok(proxy!(self).ip_interface()?)
    }
    fn driver(&self) -> Result<String, Error> {
        Ok(proxy!(self).driver()?)
    }
    fn driver_version(&self) -> Result<String, Error> {
        Ok(proxy!(self).driver_version()?)
    }
    fn firmware_version(&self) -> Result<String, Error> {
        Ok(proxy!(self).firmware_version()?)
    }
    fn capabilities(&self) -> Result<Capability, Error> {
        let cap = proxy!(self).capabilities()?;
        match FromPrimitive::from_u32(cap) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedDevice),
        }
    }
    fn ip4_address(&self) -> Result<u32, Error> {
        Ok(proxy!(self).ip4_address()?)
    }
    fn state(&self) -> Result<u32, Error> {
        Ok(proxy!(self).state()?)
    }
    fn state_reason(&self) -> Result<(u32, u32), Error> {
        Ok(proxy!(self).state_reason()?)
    }
    fn active_connection(&self) -> Result<Connection, Error> {
        todo!()
    }
    fn ip4_config(&self) -> Result<Config, Error> {
        todo!()
    }
    fn dhcp4_config(&self) -> Result<Config, Error> {
        todo!()
    }
    fn ip6_config(&self) -> Result<Config, Error> {
        todo!()
    }
    fn dhcp6_config(&self) -> Result<Config, Error> {
        todo!()
    }
    fn managed(&self) -> Result<bool, Error> {
        todo!()
    }

    #[allow(unused_variables)]
    fn set_managed(&self, value: bool) -> Result<(), Error> {
        todo!()
    }
    fn autoconnect(&self) -> Result<bool, Error> {
        Ok(proxy!(self).autoconnect()?)
    }

    #[allow(unused_variables)]
    fn set_autoconnect(&self, value: bool) -> Result<(), Error> {
        todo!()
    }
    fn firmware_missing(&self) -> Result<bool, Error> {
        todo!()
    }
    fn nm_plugin_missing(&self) -> Result<bool, Error> {
        todo!()
    }
    fn device_type(&self) -> Result<DeviceType, Error> {
        let dev_type = proxy!(self).device_type()?;
        match FromPrimitive::from_u32(dev_type) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedDevice),
        }
    }
    fn available_connections(&self) -> Result<Vec<Connection>, Error> {
        todo!()
    }
    fn physical_port_id(&self) -> Result<String, Error> {
        todo!()
    }
    fn mtu(&self) -> Result<u32, Error> {
        todo!()
    }
    fn metered(&self) -> Result<u32, Error> {
        todo!()
    }
    fn real(&self) -> Result<bool, Error> {
        todo!()
    }
    fn ip4_connectivity(&self) -> Result<u32, Error> {
        todo!()
    }
    fn ip6_connectivity(&self) -> Result<u32, Error> {
        todo!()
    }
    fn interface_flags(&self) -> Result<u32, Error> {
        todo!()
    }
    fn hw_address(&self) -> Result<String, Error> {
        Ok(OrgFreedesktopNetworkManagerDevice::hw_address(&proxy!(
            self
        ))?)
    }
}

pub trait Wireless {
    fn get_access_points(&self) -> Result<Vec<AccessPoint>, Error>;
    fn get_all_access_points(&self) -> Result<Vec<AccessPoint>, Error>;
    // fn request_scan(
    //     &self,
    //     options: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>,
    // ) -> Result<(), Error>;
    fn perm_hw_address(&self) -> Result<String, Error>;
    fn mode(&self) -> Result<u32, Error>;
    fn bitrate(&self) -> Result<u32, Error>;
    fn access_points(&self) -> Result<Vec<AccessPoint>, Error>;
    fn active_access_point(&self) -> Result<AccessPoint, Error>;
    fn wireless_capabilities(&self) -> Result<u32, Error>;
    fn last_scan(&self) -> Result<i64, Error>;
}

impl<'a> Wireless for Device<'a> {
    fn get_access_points(&self) -> Result<Vec<AccessPoint>, Error> {
        todo!()
    }
    fn get_all_access_points(&self) -> Result<Vec<AccessPoint>, Error> {
        todo!()
    }
    fn perm_hw_address(&self) -> Result<String, Error> {
        todo!()
    }
    fn mode(&self) -> Result<u32, Error> {
        todo!()
    }
    fn bitrate(&self) -> Result<u32, Error> {
        Ok(proxy!(self).bitrate()?)
    }
    fn access_points(&self) -> Result<Vec<AccessPoint>, Error> {
        todo!()
    }
    fn active_access_point(&self) -> Result<AccessPoint, Error> {
        todo!()
    }
    fn wireless_capabilities(&self) -> Result<u32, Error> {
        todo!()
    }
    fn last_scan(&self) -> Result<i64, Error> {
        todo!()
    }
}

pub trait Wired {
    fn perm_hw_address(&self) -> Result<String, Error>;
    fn speed(&self) -> Result<u32, Error>;
    fn s390_subchannels(&self) -> Result<Vec<String>, Error>;
    fn carrier(&self) -> Result<bool, Error>;
}

impl<'a> Wired for Device<'a> {
    fn perm_hw_address(&self) -> Result<String, Error> {
        todo!()
    }
    fn speed(&self) -> Result<u32, Error> {
        Ok(proxy!(self).speed()?)
    }
    fn s390_subchannels(&self) -> Result<Vec<String>, Error> {
        todo!()
    }
    fn carrier(&self) -> Result<bool, Error> {
        todo!()
    }
}
