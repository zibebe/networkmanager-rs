use networkmanager::{Error, NetworkManager};

use dbus::blocking::Connection as DBusConnection;

fn main() -> Result<(), Error> {
    let dbus_connection = DBusConnection::new_system()?;

    let nm = NetworkManager::new(&dbus_connection);

    println!("primary_connection.id: {}", nm.primary_connection()?.id()?);
    println!("primary_connection_type: {}", nm.primary_connection_type()?);

    for con in nm.get_active_connections()? {
        println!("{}:", con.id()?);
        println!("\tconnection: {:?}", con.connection()?);
        println!("\tuuid: {:?}", con.uuid()?);
        println!("\tvpn: {:?}", con.vpn()?);
        println!("\tdefault: {:?}", con.default()?);
        println!("\tdefault6: {:?}", con.default6()?);
        println!("\tstate: {:?}", con.state()?);

        if let Ok(dv4) = con.dhcp4_config() {
            println!("\tDHCP4 Config:");
            if let Ok(de) = dv4.options() {
                println!("\t\toptions: {:?}", de);
            } else {
                println!("\t\tEmpty");
            }
        }

        if let Ok(dv6) = con.dhcp6_config() {
            println!("\tDHCP6 Config:");
            if let Ok(de) = dv6.options() {
                println!("\t\toptions: {:?}", de);
            } else {
                println!("\t\tEmpty");
            }
        }

        println!("");
    }

    Ok(())
}
