mod adsl;
mod any;
mod bond;
mod bridge;
mod bt;
mod dummy;
mod generic;
#[cfg(feature = "v1_46")]
mod hsr;
mod infiniband;
mod ip_tunnel;
#[cfg(feature = "v1_52")]
mod ipvlan;
mod lowpan;
mod macsec;
mod macvlan;
mod modem;
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
pub use self::infiniband::Infiniband;
pub use self::ip_tunnel::IpTunnel;
#[cfg(feature = "v1_52")]
pub use self::ipvlan::IpVlan;
pub use self::lowpan::Lowpan;
pub use self::macsec::Macsec;
pub use self::macvlan::Macvlan;
pub use self::modem::Modem;
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
    Infiniband(InfinibandDevice<'a>),
    IpTunnel(IpTunnelDevice<'a>),
    #[cfg(feature = "v1_52")]
    IpVlan(IpVlanDevice<'a>),
    Lowpan(LowpanDevice<'a>),
    Loopback(LoopbackDevice<'a>),
    Macsec(MacsecDevice<'a>),
    Macvlan(MacvlanDevice<'a>),
    Modem(ModemDevice<'a>),
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

pub struct InfinibandDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct IpTunnelDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

#[cfg(feature = "v1_52")]
pub struct IpVlanDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct LowpanDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct LoopbackDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct MacsecDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct MacvlanDevice<'a> {
    dbus_accessor: DBusAccessor<'a>,
}

pub struct ModemDevice<'a> {
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
                DeviceType::Infiniband => {
                    Ok(Device::Infiniband(InfinibandDevice { dbus_accessor }))
                }
                DeviceType::IpTunnel => Ok(Device::IpTunnel(IpTunnelDevice { dbus_accessor })),
                #[cfg(feature = "v1_52")]
                DeviceType::IpVlan => Ok(Device::IpVlan(IpVlanDevice { dbus_accessor })),
                DeviceType::SixLowpan => Ok(Device::Lowpan(LowpanDevice { dbus_accessor })),
                DeviceType::Loopback => Ok(Device::Loopback(LoopbackDevice { dbus_accessor })),
                DeviceType::Macsec => Ok(Device::Macsec(MacsecDevice { dbus_accessor })),
                DeviceType::Macvlan => Ok(Device::Macvlan(MacvlanDevice { dbus_accessor })),
                DeviceType::Modem => Ok(Device::Modem(ModemDevice { dbus_accessor })),
                DeviceType::Veth => Ok(Device::Veth(VethDevice { dbus_accessor })),
                _ => Ok(Device::UnsupportedDevice),
            },
            None => Err(Error::UnsupportedType),
        }
    }
}
