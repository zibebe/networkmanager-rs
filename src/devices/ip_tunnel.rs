use super::{Device, IpTunnelDevice};
use crate::dbus_api::DBusAccessor;
use crate::gen::OrgFreedesktopNetworkManagerDeviceIPTunnel;
use crate::Error;

pub trait IpTunnel {
    /// The tunneling mode.
    fn mode(&self) -> Result<u32, Error>;
    /// The parent device.
    fn parent(&self) -> Result<Device, Error>;
    /// The local endpoint of the tunnel.
    fn local(&self) -> Result<String, Error>;
    /// The remote endpoint of the tunnel.
    fn remote(&self) -> Result<String, Error>;
    /// The TTL assigned to tunneled packets.
    /// 0 is a special value meaning that packets inherit the TTL value
    fn ttl(&self) -> Result<u8, Error>;
    /// The type of service (IPv4) or traffic class (IPv6) assigned to tunneled packets.
    fn tos(&self) -> Result<u8, Error>;
    /// Whether path MTU discovery is enabled on this tunnel.
    fn path_mtu_discovery(&self) -> Result<bool, Error>;
    /// The key used for incoming packets.
    fn input_key(&self) -> Result<String, Error>;
    /// The key used for outgoing packets.
    fn output_key(&self) -> Result<String, Error>;
    /// How many additional levels of encapsulation are permitted to be prepended to packets.
    /// This property applies only to IPv6 tunnels.
    fn encapsulation_limit(&self) -> Result<u8, Error>;
    /// The flow label to assign to tunnel packets.
    /// This property applies only to IPv6 tunnels.
    fn flow_label(&self) -> Result<u32, Error>;
    /// The fwmark value to assign to tunnel packets.
    /// This property applies only to VTI tunnels.
    fn fw_mark(&self) -> Result<u32, Error>;
    /// Tunnel flags.
    fn flags(&self) -> Result<u32, Error>;
}

impl<'a> IpTunnel for IpTunnelDevice<'a> {
    fn mode(&self) -> Result<u32, Error> {
        Ok(proxy!(self).mode()?)
    }

    fn parent(&self) -> Result<Device, Error> {
        let parent_path = proxy!(self).parent()?;
        Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &parent_path,
        ))
    }

    fn local(&self) -> Result<String, Error> {
        Ok(proxy!(self).local()?)
    }

    fn remote(&self) -> Result<String, Error> {
        Ok(proxy!(self).remote()?)
    }

    fn ttl(&self) -> Result<u8, Error> {
        Ok(proxy!(self).ttl()?)
    }

    fn tos(&self) -> Result<u8, Error> {
        Ok(proxy!(self).tos()?)
    }

    fn path_mtu_discovery(&self) -> Result<bool, Error> {
        Ok(proxy!(self).path_mtu_discovery()?)
    }

    fn input_key(&self) -> Result<String, Error> {
        Ok(proxy!(self).input_key()?)
    }

    fn output_key(&self) -> Result<String, Error> {
        Ok(proxy!(self).output_key()?)
    }

    fn encapsulation_limit(&self) -> Result<u8, Error> {
        Ok(proxy!(self).encapsulation_limit()?)
    }

    fn flow_label(&self) -> Result<u32, Error> {
        Ok(proxy!(self).flow_label()?)
    }

    fn fw_mark(&self) -> Result<u32, Error> {
        Ok(proxy!(self).fw_mark()?)
    }

    fn flags(&self) -> Result<u32, Error> {
        Ok(proxy!(self).flags()?)
    }
}
