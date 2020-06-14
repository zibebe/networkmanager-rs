use crate::configs::{Dhcp4Config, Dhcp6Config, Ip4Config, Ip6Config};
use crate::dbus_api::DBusAccessor;
use crate::devices::Device;
use crate::gen::OrgFreedesktopNetworkManagerConnectionActive;
use crate::types::{ActivationStateFlags, ActiveConnectionState};
use crate::Error;
use num_traits::FromPrimitive;

pub struct Connection<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> Connection<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Connection { dbus_accessor }
    }
    pub fn connection(&self) -> Result<String, Error> {
        Ok(proxy!(self).connection()?.to_string())
    }
    pub fn specific_object(&self) -> Result<String, Error> {
        Ok(proxy!(self).specific_object()?.to_string())
    }
    pub fn id(&self) -> Result<String, Error> {
        Ok(proxy!(self).id()?)
    }
    pub fn uuid(&self) -> Result<String, Error> {
        Ok(proxy!(self).uuid()?)
    }
    pub fn type_(&self) -> Result<String, Error> {
        Ok(proxy!(self).type_()?)
    }
    pub fn devices(&self) -> Result<Vec<Device>, Error> {
        let device_paths = proxy!(self).devices()?;
        let mut res = Vec::new();
        for dev_path in device_paths {
            res.push(Device::new(DBusAccessor::new(
                self.dbus_accessor.connection,
                &self.dbus_accessor.bus,
                &dev_path,
            ))?);
        }
        Ok(res)
    }
    pub fn state(&self) -> Result<ActiveConnectionState, Error> {
        let state = proxy!(self).state()?;
        match FromPrimitive::from_u32(state) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedType),
        }
    }
    pub fn state_flags(&self) -> Result<ActivationStateFlags, Error> {
        let state = proxy!(self).state_flags()?;
        match FromPrimitive::from_u32(state) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedType),
        }
    }
    pub fn default(&self) -> Result<bool, Error> {
        Ok(proxy!(self).default()?)
    }
    pub fn ip4_config(&self) -> Result<Ip4Config, Error> {
        let path = proxy!(self).ip4_config()?;
        Ok(Ip4Config::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }
    pub fn dhcp4_config(&self) -> Result<Dhcp4Config, Error> {
        let path = proxy!(self).dhcp4_config()?;
        Ok(Dhcp4Config::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }
    pub fn default6(&self) -> Result<bool, Error> {
        Ok(proxy!(self).default6()?)
    }
    pub fn ip6_config(&self) -> Result<Ip6Config, Error> {
        let path = proxy!(self).ip6_config()?;
        Ok(Ip6Config::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }
    pub fn dhcp6_config(&self) -> Result<Dhcp6Config, Error> {
        let path = proxy!(self).dhcp6_config()?;
        Ok(Dhcp6Config::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }
    pub fn vpn(&self) -> Result<bool, Error> {
        Ok(proxy!(self).vpn()?)
    }
    pub fn master(&self) -> Result<Device, Error> {
        let dev_path = proxy!(self).master()?;
        Ok(Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &dev_path,
        ))?)
    }
}
