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
//! networkmanager = "0.3"
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use networkmanager::devices::{Any, Device, Wired, Wireless};
//! use networkmanager::{Error, NetworkManager};
//!
//! use dbus::blocking::Connection;
//!
//! fn main() -> Result<(), Error> {
//!     let dbus_connection = Connection::new_system()?;
//!
//!     let nm = NetworkManager::new(&dbus_connection);
//!
//!     for dev in nm.get_devices()? {
//!         match dev {
//!             Device::Ethernet(x) => {
//!                 println!("Is autoconnected: {:?}", x.autoconnect()?);
//!                 println!("Speed: {:?}", x.speed()?);
//!                 println!("S390 Subchannels: {:?}", x.s390_subchannels()?);
//!                 println!("Carrier: {:?}", x.carrier()?);
//!             }
//!             Device::WiFi(x) => {
//!                 println!("Access Point: {:?}", x.access_points()?);
//!             }
//!             _ => {}
//!         }
//!     }
//!
//!     let enp0s2 = nm.get_device_by_ip_iface("enp0s2")?;
//!     match enp0s2 {
//!         Device::Ethernet(x) => {
//!             // NetworkManager >= 1.24
//!             // println!("Hardware Address: {:?}", Any::hw_address(&x)?);
//!
//!             // NetworkManager < 1.24
//!             // println!("Hardware Address: {:?}", Wired::hw_address(&x)?);
//!
//!             println!("Speed: {:?}", x.speed()?);
//!         }
//!         _ => {}
//!     }
//!
//!     Ok(())
//! }
//! ```

mod api;

pub use api::dbus;
pub use api::devices;
pub use api::errors::Error;
pub use api::networkmanager::NetworkManager;
pub use api::types;
