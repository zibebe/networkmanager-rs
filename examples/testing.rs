use networkmanager::{Error, NetworkManager};

fn main() -> Result<(), Error> {
    let nm = NetworkManager::new()?;

    let devs = nm.get_devices()?;

    for dev in devs {
        println!("Is autoconnected: {:?}", dev.autoconnect()?);
    }

    Ok(())
}
