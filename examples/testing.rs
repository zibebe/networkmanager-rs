use networkmanager::device::{Any, Wired, Wireless};
use networkmanager::types::DeviceType;
use networkmanager::{Error, NetworkManager};

fn main() -> Result<(), Error> {
    let nm = NetworkManager::new()?;

    let enp0s2 = nm.get_device_by_ip_iface("enp0s2")?;

    println!("Device enp0s2: {:?}", enp0s2);

    for dev in nm.get_devices()? {
        println!("Is autoconnected: {:?}", dev.autoconnect()?);
        println!("Device Type: {:?}", dev.device_type()?);
        match dev.device_type()? {
            DeviceType::WiFi => {
                println!("Access Point {:?}", dev.access_points()?);
            }
            DeviceType::Ethernet => {
                println!("Speed {:?}", dev.speed()?);
            }
            _ => {}
        }
    }
    Ok(())
}
