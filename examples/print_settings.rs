use dbus::blocking::Connection;
use networkmanager::{Error, Settings};

fn main() -> Result<(), Error> {
    let dbus_connection = Connection::new_system()?;
    let settings = Settings::new(&dbus_connection);

    println!("hostname: {:?}", settings.get_hostname()?);

    for con in settings.get_all_connections()? {
        println!("filename: {:?}", con.filename()?);
        println!("flags: {:?}", con.flags()?);
        println!("settings: {:?}", con.get_settings()?);
    }

    Ok(())
}
