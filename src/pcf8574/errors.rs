
use core::fmt;

#[derive(Debug)]
pub enum Pcf8574Error<E: fmt::Debug> {
    I2cError(E),
    NoDeviceFound,
}

impl<E: fmt::Debug> fmt::Display for Pcf8574Error<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pcf8574Error::I2cError(e) => write!(f, "I2C error: {:?}", e),
            Pcf8574Error::NoDeviceFound => write!(f, "No device found on the I2C bus"),
        }
    }
}