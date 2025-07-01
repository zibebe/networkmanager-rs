use super::Device;
use super::Error;
use super::MacsecDevice;
use crate::dbus_api::DBusAccessor;
use crate::gen::OrgFreedesktopNetworkManagerDeviceMacsec;

/// MACSec Device
pub trait Macsec {
    /// The parent device.
    fn parent(&self) -> Result<Device, Error>;
    /// The Secure Channel Identifier in use.
    fn sci(&self) -> Result<u64, Error>;
    /// The length of ICV (Integrity Check Value).
    fn icv_length(&self) -> Result<u8, Error>;
    /// The set of cryptographic algorithms in use (e.g. 0x0080020001000001 for GCM-AES-128).
    fn cipher_suite(&self) -> Result<u64, Error>;
    /// The size of the replay window.
    fn window(&self) -> Result<u32, Error>;
    /// The value of the Association Number (0..3) for the Security Association in use.
    fn encoding_sa(&self) -> Result<u8, Error>;
    /// The validation mode for incoming packets (strict, check, disabled).
    fn validation(&self) -> Result<String, Error>;
    /// Whether encryption of transmitted frames is enabled.
    fn encrypt(&self) -> Result<bool, Error>;
    /// Whether protection of transmitted frames is enabled.
    fn protect(&self) -> Result<bool, Error>;
    /// Whether the SCI is always included in SecTAG for transmitted frames.
    fn include_sci(&self) -> Result<bool, Error>;
    /// Whether the ES (End station) bit is enabled in SecTAG for transmitted frames.
    fn es(&self) -> Result<bool, Error>;
    /// Whether the SCB (Single Copy Broadcast) bit is enabled in SecTAG for transmitted frames.
    fn scb(&self) -> Result<bool, Error>;
    /// Whether replay protection is enabled.
    fn replay_protect(&self) -> Result<bool, Error>;
}

impl<'a> Macsec for MacsecDevice<'a> {
    fn parent(&self) -> Result<Device, Error> {
        let parent_path = proxy!(self).parent()?;
        Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &parent_path,
        ))
    }

    fn sci(&self) -> Result<u64, Error> {
        Ok(proxy!(self).sci()?)
    }

    fn icv_length(&self) -> Result<u8, Error> {
        Ok(proxy!(self).icv_length()?)
    }

    fn cipher_suite(&self) -> Result<u64, Error> {
        Ok(proxy!(self).cipher_suite()?)
    }

    fn window(&self) -> Result<u32, Error> {
        Ok(proxy!(self).window()?)
    }

    fn encoding_sa(&self) -> Result<u8, Error> {
        Ok(proxy!(self).encoding_sa()?)
    }

    fn validation(&self) -> Result<String, Error> {
        Ok(proxy!(self).validation()?)
    }

    fn encrypt(&self) -> Result<bool, Error> {
        Ok(proxy!(self).encrypt()?)
    }

    fn protect(&self) -> Result<bool, Error> {
        Ok(proxy!(self).protect()?)
    }

    fn include_sci(&self) -> Result<bool, Error> {
        Ok(proxy!(self).include_sci()?)
    }

    fn es(&self) -> Result<bool, Error> {
        Ok(proxy!(self).es()?)
    }

    fn scb(&self) -> Result<bool, Error> {
        Ok(proxy!(self).scb()?)
    }

    fn replay_protect(&self) -> Result<bool, Error> {
        Ok(proxy!(self).replay_protect()?)
    }
}
