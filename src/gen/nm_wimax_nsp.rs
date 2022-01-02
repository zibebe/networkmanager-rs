// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerWiMaxNsp {
    fn name(&self) -> Result<String, dbus::Error>;
    fn signal_quality(&self) -> Result<u32, dbus::Error>;
    fn network_type(&self) -> Result<u32, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::Connection>> OrgFreedesktopNetworkManagerWiMaxNsp
    for blocking::Proxy<'a, C>
{
    fn name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.WiMax.Nsp",
            "Name",
        )
    }

    fn signal_quality(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.WiMax.Nsp",
            "SignalQuality",
        )
    }

    fn network_type(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.WiMax.Nsp",
            "NetworkType",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerWiMaxNspPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerWiMaxNspPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerWiMaxNspPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerWiMaxNspPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerWiMaxNspPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.WiMax.Nsp";
}
