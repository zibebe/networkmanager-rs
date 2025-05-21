//!
//! # networkmanager
//!
//! A [NetworkManager](https://wiki.gnome.org/Projects/NetworkManager) API library using the [D-Bus message bus system](https://www.freedesktop.org/wiki/Software/dbus/)
//!
//! ## Usage
//!
//! Add networkmanager and dbus to your `Cargo.toml` with:
//!
//! ```toml
//! [dependencies]
//! networkmanager = "0.3"
//! dbus = "0.8"
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
//!                 println!("Bitrate: {:?}", x.bitrate()?);
//!                 x.request_scan(std::collections::HashMap::new())?;
//!                 for ap in x.get_all_access_points()? {
//!                     println!("SSID: {:?}", ap.ssid()?);
//!                 }
//!             }
//!             _ => {}
//!         }
//!     }
//!
//!     let eth0 = nm.get_device_by_ip_iface("eth0")?;
//!     match eth0 {
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

mod gen;
#[macro_use]
mod dbus_api;
mod accesspoint;
mod errors;
mod networkmanager;
mod settings;

pub mod configs;
pub mod connection;
pub mod devices;
pub mod types;

pub use crate::errors::Error;
pub use crate::networkmanager::NetworkManager;
pub use crate::settings::Settings;
