use super::errors::Error;
use dbus::arg::{RefArg, Variant};
use dbus::blocking::{Connection as DBusConnection, Proxy as DBusProxy};
use std::time::Duration;

const DBUS_TIMEOUT_MS: u64 = 5000;

pub struct DBusApi {
    dbus_connection: DBusConnection,
    bus: String,
}

impl DBusApi {
    pub(super) fn new(bus: &str) -> Result<Self, Error> {
        Ok(DBusApi {
            dbus_connection: DBusConnection::new_system()?,
            bus: bus.to_string(),
        })
    }

    pub(super) fn create_proxy(&self, path: &str) -> DBusProxy<'_, &DBusConnection> {
        self.dbus_connection.with_proxy(
            &self.bus,
            path.to_owned(),
            Duration::from_millis(DBUS_TIMEOUT_MS),
        )
    }
}

pub trait VariantTo<T> {
    fn variant_to(value: &Variant<Box<dyn RefArg>>) -> Option<T>;
}

impl VariantTo<String> for DBusApi {
    fn variant_to(value: &Variant<Box<dyn RefArg>>) -> Option<String> {
        value.0.as_str().and_then(|v| Some(v.to_string()))
    }
}

impl VariantTo<i64> for DBusApi {
    fn variant_to(value: &Variant<Box<dyn RefArg>>) -> Option<i64> {
        value.0.as_i64()
    }
}

impl VariantTo<u32> for DBusApi {
    fn variant_to(value: &Variant<Box<dyn RefArg>>) -> Option<u32> {
        value.0.as_i64().and_then(|v| Some(v as u32))
    }
}

impl VariantTo<bool> for DBusApi {
    fn variant_to(value: &Variant<Box<dyn RefArg>>) -> Option<bool> {
        value.0.as_i64().and_then(|v| Some(v == 0))
    }
}

impl VariantTo<Vec<String>> for DBusApi {
    fn variant_to(value: &Variant<Box<dyn RefArg>>) -> Option<Vec<String>> {
        let mut result = Vec::new();

        if let Some(list) = value.0.as_iter() {
            for element in list {
                if let Some(string) = element.as_str() {
                    result.push(string.to_string());
                } else {
                    return None;
                }
            }

            Some(result)
        } else {
            None
        }
    }
}

impl VariantTo<Vec<u8>> for DBusApi {
    fn variant_to(value: &Variant<Box<dyn RefArg>>) -> Option<Vec<u8>> {
        let mut result = Vec::new();

        if let Some(list) = value.0.as_iter() {
            for element in list {
                if let Some(value) = element.as_i64() {
                    result.push(value as u8);
                } else {
                    return None;
                }
            }

            Some(result)
        } else {
            None
        }
    }
}
