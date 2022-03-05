use crate::dbus_api::DBusAccessor;
use crate::gen::OrgFreedesktopNetworkManagerSettingsConnection;
use crate::types::HashMapInceptionVariantBox;
use crate::Error;

use dbus::arg::{RefArg, Variant};
use std::collections::HashMap;

pub struct SettingsConnection<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> SettingsConnection<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Self {
        SettingsConnection { dbus_accessor }
    }

    pub fn update(
        &self,
        properties: HashMapInceptionVariantBox<&str, dyn RefArg>,
    ) -> Result<(), Error> {
        Ok(proxy!(self).update(properties)?)
    }

    pub fn update_unsaved(
        &self,
        properties: HashMapInceptionVariantBox<&str, dyn RefArg>,
    ) -> Result<(), Error> {
        Ok(proxy!(self).update_unsaved(properties)?)
    }

    pub fn delete(&self) -> Result<(), Error> {
        Ok(proxy!(self).delete()?)
    }

    pub fn get_settings(
        &self,
    ) -> Result<HashMapInceptionVariantBox<String, dyn RefArg + 'static>, Error> {
        Ok(proxy!(self).get_settings()?)
    }

    pub fn get_secrets(
        &self,
        setting_name: &str,
    ) -> Result<HashMapInceptionVariantBox<String, dyn RefArg + 'static>, Error> {
        Ok(proxy!(self).get_secrets(setting_name)?)
    }

    pub fn clear_secrets(&self) -> Result<(), Error> {
        Ok(proxy!(self).clear_secrets()?)
    }

    pub fn save(&self) -> Result<(), Error> {
        Ok(proxy!(self).save()?)
    }

    pub fn update2(
        &self,
        settings: HashMapInceptionVariantBox<&str, dyn RefArg>,
        flags: u32,
        args: HashMap<&str, Variant<Box<dyn RefArg>>>,
    ) -> Result<HashMap<String, Variant<Box<dyn RefArg + 'static>>>, Error> {
        Ok(proxy!(self).update2(settings, flags, args)?)
    }

    pub fn unsaved(&self) -> Result<bool, Error> {
        Ok(proxy!(self).unsaved()?)
    }

    pub fn flags(&self) -> Result<u32, Error> {
        Ok(proxy!(self).flags()?)
    }

    pub fn filename(&self) -> Result<String, Error> {
        Ok(proxy!(self).filename()?)
    }
}
