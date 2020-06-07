use networkmanager::devices::{Any, Wired, Wireless};
use networkmanager::types::DeviceType;
use networkmanager::{Error, NetworkManager};

fn main() -> Result<(), Error> {
    let nm = NetworkManager::new()?;

    let enp0s2 = nm.get_device_by_ip_iface("enp0s2")?;

    for dev in nm.get_devices()? {
        println!("Is autoconnected: {:?}", Any::autoconnect(&dev)?);
        println!("Device Type: {:?}", Any::device_type(&dev)?);
        match Any::device_type(&dev)? {
            DeviceType::WiFi => {
                println!("Access Point: {:?}", Wireless::access_points(&dev)?);
            }
            DeviceType::Ethernet => {
                println!("Speed: {:?}", dev.speed()?);
                println!(
                    "Permanent Hardware Address: {:?}",
                    Wired::perm_hw_address(&dev)?
                );
                println!("S390 Subchannels: {:?}", Wired::s390_subchannels(&dev)?);
                println!("Carrier: {:?}", Wired::carrier(&dev)?);
            }
            _ => {}
        }
    }
    Ok(())
}
