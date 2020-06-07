# networkmanager-rs
[![Crates.io](https://img.shields.io/crates/v/networkmanager.svg)](https://crates.io/crates/networkmanager)
[![docs.rs](https://docs.rs/networkmanager/badge.svg)](https://docs.rs/networkmanager)
[![CI](https://github.com/he4d/networkmanager-rs/workflows/Continuous%20Integration/badge.svg)](https://github.com/he4d/networkmanager-rs/actions)

A [NetworkManager](https://wiki.gnome.org/Projects/NetworkManager) API library using the [D-Bus message bus system](https://www.freedesktop.org/wiki/Software/dbus/)

## Status

This project is still under development. Currently implemented parts can be found in the docs.

- NetworkManager D-Bus API >= v1.24.2

## Prerequisites

* ### Debian and its derivatives (e.g. Ubuntu)
  * network-manager
  * libdbus-1-dev
  * pkg-config
* ### Fedora
  * NetworkManager
  * dbus-devel
  * pkg-config

## Usage

```rust
use networkmanager::devices::Any;
use networkmanager::types::DeviceType;
use networkmanager::{Error, NetworkManager};

fn main() -> Result<(), Error> {
    let nm = NetworkManager::new()?;

    let enp0s2 = nm.get_device_by_ip_iface("enp0s2")?;

    for dev in nm.get_devices()? {
        println!("Is autoconnected: {:?}", dev.autoconnect()?);
        println!("Device Type: {:?}", dev.device_type()?);
        match dev.device_type()? {
            DeviceType::WiFi => {
                use networkmanager::devices::Wireless;
                println!("Access Point: {:?}", dev.access_points()?);
            }
            DeviceType::Ethernet => {
                use networkmanager::devices::Wired;
                println!("Speed: {:?}", dev.speed()?);
                println!("Permanent Hardware Address: {:?}", dev.perm_hw_address()?);
                println!("S390 Subchannels: {:?}", dev.s390_subchannels()?);
                println!("Carrier: {:?}", dev.carrier()?);
            }
            _ => {}
        }
    }
    Ok(())
}
```

Also, check the examples directory

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions. 
