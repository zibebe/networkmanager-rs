use super::BluetoothDevice;
use crate::errors::Error;
use crate::gen::OrgFreedesktopNetworkManagerDeviceBluetooth;
use crate::types::NMBluetoothCapabilities;
use num_traits::FromPrimitive;

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
        let bt_cap = proxy!(self).bt_capabilities()?;
        match FromPrimitive::from_u32(bt_cap) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedType),
        }
    }

    fn name(&self) -> Result<String, Error> {
        Ok(proxy!(self).name()?)
    }
}
