use crate::configs::{Dhcp4Config, Dhcp6Config, Ip4Config, Ip6Config};
use crate::dbus_api::DBusAccessor;
use crate::devices::Device;
use crate::gen::OrgFreedesktopNetworkManagerConnectionActive;
use crate::gen::OrgFreedesktopNetworkManagerSettingsConnection;
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
        Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &dev_path,
        ))
    }
    pub fn get_secrets(
        &self,
        setting_name: &str,
    ) -> Result<std::collections::HashMap<String, Box<dyn dbus::arg::RefArg>>, Error> {
        let conn = proxy!(self).connection()?;
        // Our dbus_accessor represents an ActiveConnection, but we need to go to the underlying
        // Connection.
        let conn_accessor = DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &conn,
        );
        let secrets = conn_accessor.create_proxy().get_secrets(setting_name)?;

        let requested = secrets
            .get(setting_name)
            // FIXME: Is that the right error? Does seem so, because a type was requested that
            // NetworkManager does not know.
            .ok_or(Error::UnsupportedType)?;

        use dbus::arg::RefArg;
        Ok(requested
            .iter()
            .map(|(k, v)| (k.clone(), v.box_clone()))
            .collect())
    }
}
