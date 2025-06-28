mod adsl;
mod any;
mod bond;
mod bridge;
mod bt;
mod dummy;
mod generic;
#[cfg(feature = "v1_46")]
mod hsr;
mod veth;
mod wired;
mod wireless;

pub use self::adsl::Adsl;
pub use self::any::Any;
pub use self::bond::Bond;
pub use self::bridge::Bridge;
pub use self::bt::Bluetooth;
pub use self::dummy::Dummy;
pub use self::generic::Generic;
#[cfg(feature = "v1_46")]
pub use self::hsr::Hsr;
pub use self::veth::Veth;
pub use self::wired::Wired;
pub use self::wireless::Wireless;
use crate::dbus_api::DBusAccessor;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDevice;
use crate::types::DeviceType;
use num_traits::FromPrimitive;

pub enum Device<'a> {
    WiFi(WiFiDevice<'a>),
    Adsl(AdslDevice<'a>),
    Bluetooth(BluetoothDevice<'a>),
    Bond(BondDevice<'a>),
    Dummy(DummyDevice<'a>),
    #[cfg(feature = "v1_46")]
    Hsr(HsrDevice<'a>),
    Ethernet(EthernetDevice<'a>),
    Generic(GenericDevice<'a>),
    Bridge(BridgeDevice<'a>),
    Veth(VethDevice<'a>),
    UnsupportedDevice,
}

pub struct GenericDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct WiFiDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct AdslDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct BluetoothDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct BondDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct EthernetDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct BridgeDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct DummyDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

#[cfg(feature = "v1_46")]
pub struct HsrDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct VethDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

impl<'a> Device<'a> {
    pub(crate) fn new(dbus_accessor: DBusAccessor<'a>) -> Result<Self, Error> {
        let dev_type = dbus_accessor.create_proxy().device_type()?;
        match FromPrimitive::from_u32(dev_type) {
            Some(x) => match x {
                DeviceType::Wifi => Ok(Device::WiFi(WiFiDevice { dbus_accessor })),
                DeviceType::Adsl => Ok(Device::Adsl(AdslDevice { dbus_accessor })),
                DeviceType::Bt => Ok(Device::Bluetooth(BluetoothDevice { dbus_accessor })),
                DeviceType::Dummy => Ok(Device::Dummy(DummyDevice { dbus_accessor })),
                DeviceType::Bond => Ok(Device::Bond(BondDevice { dbus_accessor })),
                DeviceType::Ethernet => Ok(Device::Ethernet(EthernetDevice { dbus_accessor })),
                DeviceType::Generic => Ok(Device::Generic(GenericDevice { dbus_accessor })),
                DeviceType::Bridge => Ok(Device::Bridge(BridgeDevice { dbus_accessor })),
                #[cfg(feature = "v1_46")]
                DeviceType::Hsr => Ok(Device::Hsr(HsrDevice { dbus_accessor })),
                DeviceType::Veth => Ok(Device::Veth(VethDevice { dbus_accessor })),
                _ => Ok(Device::UnsupportedDevice),
            },
            None => Err(Error::UnsupportedType),
        }
    }
}
