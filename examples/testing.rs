use networkmanager::{Error, NetworkManager};

fn main() -> Result<(), Error> {
    let nm = NetworkManager::new()?;

    let devs = nm.get_devices()?;
    for dev in devs.iter() {
        println!("Is autoconnected: {:?}", dev.autoconnect()?);
        println!("Device Type: {:?}", dev.device_type()?);
    }
    Ok(())
}
