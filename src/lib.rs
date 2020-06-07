//!
//! # networkmanager
//!
//! A [NetworkManager](https://wiki.gnome.org/Projects/NetworkManager) API library using the [D-Bus message bus system](https://www.freedesktop.org/wiki/Software/dbus/)
//!
//! ## Getting started
//!
//! Some hints on how to use this library:
//!
//! 1. Look at the examples under `examples/`.
//! 2. Look at the `NetworkManager` struct.
//!

mod api;

pub use api::device;
pub use api::errors::Error;
pub use api::networkmanager::NetworkManager;
pub use api::types;
