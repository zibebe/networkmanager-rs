use crate::connection::Connection;
use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerSettings;
use dbus::blocking::Connection as DBusConnection;

const SETTINGS_BUS: &str = "org.freedesktop.NetworkManager.Settings";
const SETTINGS_PATH: &str = "/org/freedesktop/NetworkManager/Settings";

pub struct Settings<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> Settings<'a> {
    pub fn new(dbus_connection: &'a DBusConnection) -> Self {
        Self {
            dbus_accessor: DBusAccessor::new(dbus_connection, SETTINGS_BUS, SETTINGS_PATH),
        }
    }

    pub fn list_connections(&self) -> Result<impl Iterator<Item = Connection>, Error> {
        Ok(proxy!(self)
            .list_connections()?
            .into_iter()
            .map(move |path| {
                Connection::new(DBusAccessor::new(
                    &self.dbus_accessor.connection,
                    &self.dbus_accessor.bus,
                    &path,
                ))
            }))
    }

    pub fn get_connection_by_uuid(&self, uuid: &str) -> Result<Connection, Error> {
        Ok(proxy!(self).get_connection_by_uuid(uuid).map(move |path| {
            Connection::new(DBusAccessor::new(
                &self.dbus_accessor.connection,
                &self.dbus_accessor.bus,
                &path,
            ))
        })?)
    }
}
