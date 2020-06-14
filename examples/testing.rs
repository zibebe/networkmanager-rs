use networkmanager::devices::{Any, Device, Wired, Wireless};
use networkmanager::{Error, NetworkManager};

use dbus::blocking::Connection;

fn main() -> Result<(), Error> {
    let dbus_connection = Connection::new_system()?;

    let nm = NetworkManager::new(&dbus_connection);

    for dev in nm.get_devices()? {
        match dev {
            Device::Ethernet(x) => {
                println!("Is autoconnected: {:?}", x.autoconnect()?);
                println!("Speed: {:?}", x.speed()?);
                println!("S390 Subchannels: {:?}", x.s390_subchannels()?);
                println!("Carrier: {:?}", x.carrier()?);
                let conf = x.ip4_config()?;
                println!("Gateway: {:?}", conf.gateway()?);
                let con = x.active_connection()?;
                println!("Connection id: {}", con.id()?);
            }
            Device::WiFi(x) => {
                println!("Bitrate: {:?}", x.bitrate()?);
                x.request_scan(std::collections::HashMap::new())?;
                for ap in x.get_all_access_points()? {
                    println!("SSID: {:?}", ap.ssid()?);
                }
            }
            _ => {}
        }
    }

    let eth0 = nm.get_device_by_ip_iface("eth0")?;
    match eth0 {
        Device::Ethernet(x) => {
            // NetworkManager >= 1.24
            // println!("Hardware Address: {:?}", Any::hw_address(&x)?);

            // NetworkManager < 1.24
            // println!("Hardware Address: {:?}", Wired::hw_address(&x)?);

            println!("Speed: {:?}", x.speed()?);
        }
        _ => {}
    }

    Ok(())
}
