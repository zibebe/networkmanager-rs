// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus;
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceModem {
    fn modem_capabilities(&self) -> Result<u32, dbus::Error>;
    fn current_capabilities(&self) -> Result<u32, dbus::Error>;
    fn device_id(&self) -> Result<String, dbus::Error>;
    fn operator_code(&self) -> Result<String, dbus::Error>;
    fn apn(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::Connection>>
    OrgFreedesktopNetworkManagerDeviceModem for blocking::Proxy<'a, C>
{
    fn modem_capabilities(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Modem",
            "ModemCapabilities",
        )
    }

    fn current_capabilities(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Modem",
            "CurrentCapabilities",
        )
    }

    fn device_id(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Modem",
            "DeviceId",
        )
    }

    fn operator_code(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Modem",
            "OperatorCode",
        )
    }

    fn apn(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Modem",
            "Apn",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceModemPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceModemPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceModemPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceModemPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceModemPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Modem";
}
