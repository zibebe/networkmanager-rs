use super::BluetoothDevice;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDeviceBluetooth;
use crate::types::NMBluetoothCapabilities;

pub trait Bluetooth {
    fn hw_address(&self) -> Result<String, Error>;
    fn bt_capabilities(&self) -> Result<NMBluetoothCapabilities, Error>;
    fn name(&self) -> Result<String, Error>;
}

impl<'a> Bluetooth for BluetoothDevice<'a> {
    fn hw_address(&self) -> Result<String, Error> {
        Ok(proxy!(self).hw_address()?)
    }

    fn bt_capabilities(&self) -> Result<NMBluetoothCapabilities, Error> {
        Ok(match proxy!(self).bt_capabilities()? {
            0 => NMBluetoothCapabilities::NmBtCapabilityNone,
            1 => NMBluetoothCapabilities::NmBtCapabilityDun,
            _ => NMBluetoothCapabilities::NmBtCapabilityNap,
        })
    }

    fn name(&self) -> Result<String, Error> {
        Ok(proxy!(self).name()?)
    }
}
