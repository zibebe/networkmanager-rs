use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    DBus(dbus::Error),
    #[error("unsupported method")]
    UnsupportedMethod,
    #[error("unsupported device")]
    UnsupportedDevice,
    #[error("unsupported type")]
    UnsupportedType,
}

impl From<dbus::Error> for Error {
    fn from(error: dbus::Error) -> Self {
        Error::DBus(error)
    }
}
