use super::Device;
use super::Error;
use super::LowpanDevice;
use crate::dbus_api::DBusAccessor;

/// 6LoWPAN Device
pub trait Lowpan {
    /// The active hardware address of the device.
    fn hw_address(&self) -> Result<String, Error>;
    /// The parent device.
    fn parent(&self) -> Result<Device, Error>;
}

impl<'a> Lowpan for LowpanDevice<'a> {
    fn hw_address(&self) -> Result<String, Error> {
        use crate::gen::OrgFreedesktopNetworkManagerDevice;
        // NOTE:  DEPRECATED. Use the "HwAddress" property in
        // "org.freedesktop.NetworkManager.Device" instead which
        // exists since version NetworkManager 1.24.0.

        Ok(proxy!(self).hw_address()?)
    }

    fn parent(&self) -> Result<Device, Error> {
        use crate::gen::OrgFreedesktopNetworkManagerDeviceLowpan;
        let parent_path = proxy!(self).parent()?;
        Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &parent_path,
        ))
    }
}
