use super::BluetoothDevice;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDeviceBluetooth;

/// Source: https://www.networkmanager.dev/docs/api/latest/nm-dbus-types.html#NMBluetoothCapabilities
pub enum BluetoothCapabilities {
    /// Device has no usable capabilities
    None,
    /// Device provides Dial-Up Networking capability
    DialUpNetworking,
    /// Device provides Network Access Point capability
    NetworkAccessPoint,
}

pub struct BluetoothCapability {
    pub raw: u32,
    pub none: bool,
    pub dial_up_networking: bool,
    pub network_access_point: bool,
}

pub trait Bluetooth {
    fn capabilities(&self) -> Result<BluetoothCapability, Error>;
    fn hw_address(&self) -> Result<String, Error>;
    fn test(&self) -> String;
}

impl<'a> Bluetooth for BluetoothDevice<'a> {
    fn capabilities(&self) -> Result<BluetoothCapability, Error> {
        let raw = proxy!(self).bt_capabilities()?;
        let dial_up_networking = raw & BluetoothCapabilities::DialUpNetworking as u32 == 1;
        let network_access_point = raw & BluetoothCapabilities::NetworkAccessPoint as u32 == 2;
        Ok(BluetoothCapability {
            raw,
            dial_up_networking,
            network_access_point,
            none: raw == BluetoothCapabilities::None as u32,
        })
    }
    fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }
    fn test(&self) -> String {
        return "a".into();
    }
}
