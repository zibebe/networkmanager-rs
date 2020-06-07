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
//! networkmanager = "0.1"
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
//!
//!     for dev in nm.get_devices()? {
//!         println!("Is autoconnected: {:?}", Any::autoconnect(&dev)?);
//!         println!("Device Type: {:?}", Any::device_type(&dev)?);
//!         match Any::device_type(&dev)? {
//!             DeviceType::WiFi => {
//!                 println!("Access Point: {:?}", Wireless::access_points(&dev)?);
//!             }
//!             DeviceType::Ethernet => {
//!                 println!("Speed: {:?}", dev.speed()?);
//!                 println!(
//!                     "Permanent Hardware Address: {:?}",
//!                     Wired::perm_hw_address(&dev)?
//!                 );
//!                 println!("S390 Subchannels: {:?}", Wired::s390_subchannels(&dev)?);
//!                 println!("Carrier: {:?}", Wired::carrier(&dev)?);
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
