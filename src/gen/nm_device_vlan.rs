// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus;
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceVlan {
    fn hw_address(&self) -> Result<String, dbus::Error>;
    fn carrier(&self) -> Result<bool, dbus::Error>;
    fn parent(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn vlan_id(&self) -> Result<u32, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::Connection>> OrgFreedesktopNetworkManagerDeviceVlan
    for blocking::Proxy<'a, C>
{
    fn hw_address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Vlan",
            "HwAddress",
        )
    }

    fn carrier(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Vlan",
            "Carrier",
        )
    }

    fn parent(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Vlan",
            "Parent",
        )
    }

    fn vlan_id(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Vlan",
            "VlanId",
        )
    }
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceVlanPropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceVlanPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceVlanPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopNetworkManagerDeviceVlanPropertiesChanged {
            properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceVlanPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.Vlan";
}
