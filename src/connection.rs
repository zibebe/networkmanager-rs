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

pub trait ActiveConnection {
    fn connection(&self) -> Result<String, Error>;
    fn specific_object(&self) -> Result<String, Error>;
    fn id(&self) -> Result<String, Error>;
    fn uuid(&self) -> Result<String, Error>;
    fn type_(&self) -> Result<String, Error>;
    fn devices(&self) -> Result<Vec<Device>, Error>;
    fn state(&self) -> Result<ActiveConnectionState, Error>;
    fn state_flags(&self) -> Result<ActivationStateFlags, Error>;
    fn default(&self) -> Result<bool, Error>;
    fn ip4_config(&self) -> Result<Ip4Config, Error>;
    fn dhcp4_config(&self) -> Result<Dhcp4Config, Error>;
    fn default6(&self) -> Result<bool, Error>;
    fn ip6_config(&self) -> Result<Ip6Config, Error>;
    fn dhcp6_config(&self) -> Result<Dhcp6Config, Error>;
    fn vpn(&self) -> Result<bool, Error>;
    fn master(&self) -> Result<Device, Error>;
}

impl<'a> Connection<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        Connection { dbus_accessor }
    }
}

impl<'a> ActiveConnection for Connection<'a> {
    fn connection(&self) -> Result<String, Error> {
        Ok(proxy!(self).connection()?.to_string())
    }
    fn specific_object(&self) -> Result<String, Error> {
        Ok(proxy!(self).specific_object()?.to_string())
    }
    fn id(&self) -> Result<String, Error> {
        Ok(proxy!(self).id()?)
    }
    fn uuid(&self) -> Result<String, Error> {
        Ok(proxy!(self).uuid()?)
    }
    fn type_(&self) -> Result<String, Error> {
        Ok(proxy!(self).type_()?)
    }
    fn devices(&self) -> Result<Vec<Device>, Error> {
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
    fn state(&self) -> Result<ActiveConnectionState, Error> {
        let state = proxy!(self).state()?;
        match FromPrimitive::from_u32(state) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedType),
        }
    }
    fn state_flags(&self) -> Result<ActivationStateFlags, Error> {
        let state = proxy!(self).state_flags()?;
        match FromPrimitive::from_u32(state) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedType),
        }
    }
    fn default(&self) -> Result<bool, Error> {
        Ok(proxy!(self).default()?)
    }
    fn ip4_config(&self) -> Result<Ip4Config, Error> {
        let path = proxy!(self).ip4_config()?;
        Ok(Ip4Config::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }
    fn dhcp4_config(&self) -> Result<Dhcp4Config, Error> {
        let path = proxy!(self).dhcp4_config()?;
        Ok(Dhcp4Config::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }
    fn default6(&self) -> Result<bool, Error> {
        Ok(proxy!(self).default6()?)
    }
    fn ip6_config(&self) -> Result<Ip6Config, Error> {
        let path = proxy!(self).ip6_config()?;
        Ok(Ip6Config::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }
    fn dhcp6_config(&self) -> Result<Dhcp6Config, Error> {
        let path = proxy!(self).dhcp6_config()?;
        Ok(Dhcp6Config::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }
    fn vpn(&self) -> Result<bool, Error> {
        Ok(proxy!(self).vpn()?)
    }
    fn master(&self) -> Result<Device, Error> {
        let dev_path = proxy!(self).master()?;
        Ok(Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &dev_path,
        ))?)
    }
}
