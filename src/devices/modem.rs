use num_traits::FromPrimitive;

use super::Error;
use super::ModemDevice;
use crate::gen::OrgFreedesktopNetworkManagerDeviceModem;
use crate::types::NMDeviceModemCapabilities;

/// Modem Device
pub trait Modem {
    /// The generic family of access technologies the modem supports.
    /// Not all capabilities are available at the same time however;
    /// some modems require a firmware reload or other reinitialization
    /// to switch between eg CDMA/EVDO and GSM/UMTS.
    fn modem_capabilities(&self) -> Result<NMDeviceModemCapabilities, Error>;
    /// The generic family of access technologies the modem currently
    /// supports without a firmware reload or reinitialization.
    fn current_capabilities(&self) -> Result<NMDeviceModemCapabilities, Error>;
    /// An identifier used by the modem backend (ModemManager) that aims to
    /// uniquely identify the a device. Can be used to match a connection
    /// to a particular device.
    fn device_id(&self) -> Result<String, Error>;
    /// The MCC and MNC (concatenated) of the network the modem is connected to.
    /// Blank if disconnected or not a 3GPP modem.
    fn operator_code(&self) -> Result<String, Error>;
    /// The access point name the modem is connected to.
    /// Blank if disconnected.
    fn apn(&self) -> Result<String, Error>;
}

impl<'a> Modem for ModemDevice<'a> {
    fn modem_capabilities(&self) -> Result<NMDeviceModemCapabilities, Error> {
        let cap = proxy!(self).modem_capabilities()?;

        match FromPrimitive::from_u32(cap) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedType),
        }
    }

    fn current_capabilities(&self) -> Result<NMDeviceModemCapabilities, Error> {
        let cap = proxy!(self).current_capabilities()?;

        match FromPrimitive::from_u32(cap) {
            Some(x) => Ok(x),
            None => Err(Error::UnsupportedType),
        }
    }

    fn device_id(&self) -> Result<String, Error> {
        Ok(proxy!(self).device_id()?)
    }

    fn operator_code(&self) -> Result<String, Error> {
        Ok(proxy!(self).operator_code()?)
    }

    fn apn(&self) -> Result<String, Error> {
        Ok(proxy!(self).apn()?)
    }
}
