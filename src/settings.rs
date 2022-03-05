use crate::dbus_api::DBusAccessor;
use crate::gen::OrgFreedesktopNetworkManagerSettings;
use crate::settings_connection::SettingsConnection;
use crate::types::{HashMapInceptionVariantBox, HashMapVariantBox};
use crate::Error;

use dbus::arg::RefArg;
use dbus::blocking::Connection as DBusConnection;

const NETWORK_MANAGER_SETTINGS_BUS: &str = "org.freedesktop.NetworkManager";
const NETWORK_MANAGER_SETTINGS_PATH: &str = "/org/freedesktop/NetworkManager/Settings";

pub struct Settings<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> Settings<'a> {
    pub fn new(dbus_connection: &'a DBusConnection) -> Self {
        Settings {
            dbus_accessor: DBusAccessor::new(
                dbus_connection,
                NETWORK_MANAGER_SETTINGS_BUS,
                NETWORK_MANAGER_SETTINGS_PATH,
            ),
        }
    }

    fn paths_to_settings_connections(
        &self,
        paths: Vec<dbus::Path<'_>>,
    ) -> Result<Vec<SettingsConnection<'_>>, Error> {
        let mut res = Vec::new();
        for path in paths {
            res.push(SettingsConnection::new(DBusAccessor::new(
                self.dbus_accessor.connection,
                &self.dbus_accessor.bus,
                &path,
            )));
        }
        Ok(res)
    }

    fn path_to_settings_connection(
        &self,
        path: dbus::Path<'_>,
    ) -> Result<SettingsConnection<'_>, Error> {
        Ok(SettingsConnection::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &path,
        )))
    }

    pub fn get_connection_by_uuid(&self, uuid: &str) -> Result<SettingsConnection<'_>, Error> {
        self.path_to_settings_connection(proxy!(self).get_connection_by_uuid(uuid)?)
    }

    pub fn add_connection(
        &self,
        connection: HashMapInceptionVariantBox<&str, dyn RefArg>,
    ) -> Result<SettingsConnection<'_>, Error> {
        self.path_to_settings_connection(proxy!(self).add_connection(connection)?)
    }

    pub fn add_connection_unsaved(
        &self,
        connection: HashMapInceptionVariantBox<&str, dyn RefArg>,
    ) -> Result<SettingsConnection<'_>, Error> {
        self.path_to_settings_connection(proxy!(self).add_connection_unsaved(connection)?)
    }

    pub fn add_connection2(
        &self,
        settings: HashMapInceptionVariantBox<&str, dyn RefArg>,
        flags: u32,
        args: HashMapVariantBox<&str, dyn RefArg>,
    ) -> Result<
        (
            SettingsConnection<'_>,
            HashMapVariantBox<String, dyn RefArg + 'static>,
        ),
        Error,
    > {
        let result = proxy!(self).add_connection2(settings, flags, args)?;
        Ok((self.path_to_settings_connection(result.0)?, result.1))
    }

    pub fn load_connections(&self, filenames: Vec<&str>) -> Result<(bool, Vec<String>), Error> {
        Ok(proxy!(self).load_connections(filenames)?)
    }

    pub fn reload_connections(&self) -> Result<bool, Error> {
        Ok(proxy!(self).reload_connections()?)
    }

    pub fn save_hostname(&self, hostname: &str) -> Result<(), Error> {
        Ok(proxy!(self).save_hostname(hostname)?)
    }

    pub fn get_all_connections(&self) -> Result<Vec<SettingsConnection<'_>>, Error> {
        self.paths_to_settings_connections(proxy!(self).connections()?)
    }

    pub fn get_hostname(&self) -> Result<String, Error> {
        Ok(proxy!(self).hostname()?)
    }

    pub fn can_modify(&self) -> Result<bool, Error> {
        Ok(proxy!(self).can_modify()?)
    }
}
