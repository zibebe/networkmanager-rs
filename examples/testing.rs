use networkmanager::devices::{Any, Device, Wired, Wireless};
use networkmanager::{DBusConnection, Error, NetworkManager};

fn main() -> Result<(), Error> {
    let dbus_connection = DBusConnection::new()?;

    let nm = NetworkManager::new(&dbus_connection);

    let enp0s2 = nm.get_device_by_ip_iface("enp0s2")?;
    match enp0s2 {
        Device::Ethernet(x) => {
            // NetworkManager >= 1.24
            // println!("Hardware Address: {:?}", Any::hw_address(&x)?);

            // NetworkManager < 1.24
            // println!("Hardware Address: {:?}", Wired::hw_address(&x)?);

            println!("Speed: {:?}", x.speed()?);
        }
        _ => {}
    }

    for dev in nm.get_devices()? {
        match dev {
            Device::Ethernet(x) => {
                println!("Is autoconnected: {:?}", x.autoconnect()?);
                println!("Speed: {:?}", x.speed()?);
                println!("S390 Subchannels: {:?}", x.s390_subchannels()?);
                println!("Carrier: {:?}", x.carrier()?);
            }
            Device::WiFi(x) => {
                println!("Access Point: {:?}", x.access_points()?);
            }
            _ => {}
        }
    }
    Ok(())
}
