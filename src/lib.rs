//!
//! # networkmanager
//!
//! `networkmanager` is an library for communication with the Linux NetworkManager over D-Bus.
//!
//! ## Getting started
//!
//! Some hints on how to use this library:
//!
//! 1. Look at the examples under `examples/`.
//! 2. Look at the `NetworkManager` struct.
//!

pub mod api;

pub use api::device::Device;
pub use api::errors::Error;
pub use api::networkmanager::NetworkManager;
pub use api::types::*;
