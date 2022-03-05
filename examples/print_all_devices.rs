use networkmanager::devices::{Any, Device, Wired, Wireless};
use networkmanager::{Error, NetworkManager};

use dbus::blocking::Connection;

fn main() -> Result<(), Error> {
    let dbus_connection = Connection::new_system()?;

    let nm = NetworkManager::new(&dbus_connection);

    for dev in nm.get_devices()? {
        match dev {
            Device::Ethernet(x) => {
                println!("Interface: {:?}", x.interface()?);
                println!("Is autoconnected: {:?}", x.autoconnect()?);
                println!("Speed: {:?}", x.speed()?);
                println!("S390 Subchannels: {:?}", x.s390_subchannels()?);
                println!("Carrier: {:?}", x.carrier()?);
                if let Ok(conf) = x.ip4_config() {
                    if let Ok(addr) = conf.addresses() {
                        println!("Adresses: {:?}", addr);
                    }
                    if let Ok(gw) = conf.gateway() {
                        println!("Gateway: {:?}", gw);
                    }
                };
                if let Ok(con) = x.active_connection() {
                    if let Ok(id) = con.id() {
                        println!("Connection id: {}", id);
                    }
                }
            }
            Device::WiFi(x) => {
                println!("Interface: {:?}", x.interface()?);
                println!("Bitrate: {:?}", x.bitrate()?);
                x.request_scan(std::collections::HashMap::new())?;
                for ap in x.get_all_access_points()? {
                    println!("SSID: {:?}", ap.ssid()?);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
