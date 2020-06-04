// This code was autogenerated with `dbus-codegen-rust -m None`, see https://github.com/diwic/dbus-rs
use dbus;
use dbus::arg;
use dbus::blocking;

pub trait OrgFreedesktopNetworkManagerDeviceVrf {
    fn table(&self) -> Result<u32, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target = blocking::Connection>> OrgFreedesktopNetworkManagerDeviceVrf
    for blocking::Proxy<'a, C>
{
    fn table(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.NetworkManager.Device.Vrf",
            "Table",
        )
    }
}
