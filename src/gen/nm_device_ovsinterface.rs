// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus;
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceOvsInterface {}

impl<'a, C: ::std::ops::Deref<Target = blocking::Connection>>
    OrgFreedesktopNetworkManagerDeviceOvsInterface for blocking::Proxy<'a, C>
{
}

#[derive(Debug)]
pub struct OrgFreedesktopNetworkManagerDeviceOvsInterfacePropertiesChanged {
    pub properties:
        ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>,
}

impl arg::AppendAll for OrgFreedesktopNetworkManagerDeviceOvsInterfacePropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopNetworkManagerDeviceOvsInterfacePropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(
            OrgFreedesktopNetworkManagerDeviceOvsInterfacePropertiesChanged {
                properties: i.read()?,
            },
        )
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopNetworkManagerDeviceOvsInterfacePropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.NetworkManager.Device.OvsInterface";
}
