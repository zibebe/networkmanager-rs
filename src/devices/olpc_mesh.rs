use crate::dbus_api::DBusAccessor;

use super::Device;
use super::Error;
use super::OlpcMeshDevice;

/// OLPC Wireless Mesh Device
pub trait OlpcMesh {
    /// The hardware address of the device.
    fn hw_address(&self) -> Result<String, Error>;
    /// The companion device.
    fn companion(&self) -> Result<Device, Error>;
    /// The currently active channel.
    fn active_channel(&self) -> Result<u32, Error>;
}

impl<'a> OlpcMesh for OlpcMeshDevice<'a> {
    fn hw_address(&self) -> Result<String, Error> {
        // NOTE: DEPRECATED.
        // Use the "HwAddress" property in "org.freedesktop.NetworkManager.Device"
        // instead which exists since version NetworkManager 1.24.0.
        use crate::gen::OrgFreedesktopNetworkManagerDevice;
        Ok(proxy!(self).hw_address()?)
    }

    fn companion(&self) -> Result<Device, Error> {
        use crate::gen::OrgFreedesktopNetworkManagerDeviceOlpcMesh;
        let companion_path = proxy!(self).companion()?;

        Device::new(DBusAccessor::new(
            self.dbus_accessor.connection,
            &self.dbus_accessor.bus,
            &companion_path,
        ))
    }

    fn active_channel(&self) -> Result<u32, Error> {
        use crate::gen::OrgFreedesktopNetworkManagerDeviceOlpcMesh;
        Ok(proxy!(self).active_channel()?)
    }
}
