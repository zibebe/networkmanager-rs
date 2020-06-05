mod gen;

pub mod device;
pub mod networkmanager;

const DBUS_TIMEOUT_MS: u64 = 5000;

#[derive(Debug)]
pub enum Error {
    DBus(dbus::Error),
}

impl From<dbus::Error> for Error {
    fn from(error: dbus::Error) -> Self {
        Error::DBus(error)
    }
}
