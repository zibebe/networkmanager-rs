//!
//! # networkmanager
//!
//! A [NetworkManager](https://wiki.gnome.org/Projects/NetworkManager) API library using the [D-Bus message bus system](https://www.freedesktop.org/wiki/Software/dbus/)
//!
//! ## Usage
//!
//! Add networkmanager to your `Cargo.toml` with:
//!
//! ```toml
//! [dependencies]
//! networkmanager = "0.2"
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use networkmanager::devices::{Any, Wired, Wireless};
//! use networkmanager::types::DeviceType;
//! use networkmanager::{Error, NetworkManager};
//!
//! fn main() -> Result<(), Error> {
//!     let nm = NetworkManager::new()?;
//!
//!     let enp0s2 = nm.get_device_by_ip_iface("enp0s2")?;
//!     Wired::speed(&enp0s2)?;
//!
//!     for dev in nm.get_devices()? {
//!         println!("Is autoconnected: {:?}", dev.autoconnect()?);
//!         println!("Device Type: {:?}", dev.device_type()?);
//!         match dev.device_type()? {
//!             DeviceType::Wifi => {
//!                 println!("Access Point: {:?}", dev.access_points()?);
//!                 println!("Hardware Address: {:?}", Wireless::hw_address(&dev)?);
//!             }
//!             DeviceType::Ethernet => {
//!                 println!("Speed: {:?}", dev.speed()?);
//!                 println!("Hardware Address: {:?}", Wired::hw_address(&dev)?);
//!                 println!("S390 Subchannels: {:?}", dev.s390_subchannels()?);
//!                 println!("Carrier: {:?}", dev.carrier()?);
//!             }
//!             _ => {}
//!         }
//!     }
//!     Ok(())
//! }
//! ```

mod api;

pub use api::devices;
pub use api::errors::Error;
pub use api::networkmanager::NetworkManager;
pub use api::types;
