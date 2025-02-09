use crate::pcf8574::{Commands, Pcf8574Error};
use core::fmt;
use core::{
    error::Error,
    fmt::{Display, Write},
};
use esp_hal::i2c::master::I2c;
use esp_hal::delay::Delay;
use esp_println::println;
use heapless::String;
use core::borrow::BorrowMut;
use embedded_hal::i2c::I2c as HalI2c;

pub struct Pcf8574<I2C, E> {
    i2c: I2C,
    address: u8,
    delay: Delay,
    _error: core::marker::PhantomData<E>,
}

impl<E: fmt::Debug> Error for Pcf8574Error<E> {}

impl<I2C, E> Pcf8574<I2C, E>
where
I2C: HalI2c<Error = E> ,
    E: fmt::Debug,
{
    pub fn new(i2c: &I2C) -> Result<Self, Pcf8574Error<E>> {
        Ok(Self {
            i2c,
            address: 0x27,
            delay: Delay::new(),
            _error: core::marker::PhantomData,
        })
    }

    ///function searches for addresses and sets found address as the device address
    pub fn search_for_address(&mut self) {
        let mut device_address = 0x27;
        for address in 0x00..0x78 {
            if self.i2c.borrow_mut().write(address, &[0]).is_ok() {
                println!("Device found at address: 0x{:X}", address);
                device_address = address;
                break;
            } else {
                println!("No device found!")
            }
        }
        self.address = device_address;
    }

    fn send_byte(&mut self, byte: u8, rs: bool) -> Result<(), Pcf8574Error<E>> {
        let rs_bit = if rs { 0x01 } else { 0x00 }; // RS = 1 data, RS = 0 komento
        let high_nibble = (byte & 0xF0) | rs_bit | 0x08; // 0x08 pitää taustavalon päällä
        let low_nibble = ((byte << 4) & 0xF0) | rs_bit | 0x08;

        self.set_enable(high_nibble)?;
        self.set_enable(low_nibble)?;
        Ok(())
    }

    /// Send enable signal to the display via PCF8574
    fn set_enable(&mut self, data: u8) -> Result<(), Pcf8574Error<E>> {
        self.i2c.borrow_mut()
            .write(self.address, &[data | 0x04])
            .map_err(Pcf8574Error::I2cError)?; // E=1
        self.delay.delay_millis(5);
        self.i2c.borrow_mut()
            .write(self.address, &[data & !0x04])
            .map_err(Pcf8574Error::I2cError)?; // E=0
        Ok(())
    }

    /// Send command to the LCD
    fn send_command(&mut self, cmd: u8) -> Result<(), Pcf8574Error<E>> {
        self.send_byte(cmd, false)
    }

    /// Send charat
    fn send_char(&mut self, ch: char) -> Result<(), Pcf8574Error<E>> {
        self.send_byte(ch as u8, true)
    }

    /// Write message on the LCD
    pub fn write<T>(&mut self, message: T) -> Result<(), Pcf8574Error<E>>
    where
        T: Display,
    {
        let mut buffer = heapless::String::<32>::new();
        write!(&mut buffer, "{}", message).map_err(|_| Pcf8574Error::MessageFormatError)?;

        for ch in buffer.chars() {
            self.send_char(ch)?;
        }
        Ok(())
    }

    ///function sets command for pcf8574. Command is given as an enum
    pub fn set_command(&mut self, command: Commands) -> Result<(), Pcf8574Error<E>> {
        self.send_command(command as u8).unwrap();
        Ok(())
    }

    /// Initislize the display
    pub fn initialize_lcd(&mut self) -> Result<(), Pcf8574Error<E>> {
        self.send_command(0x03)?; // Init-sekvenssi
        self.send_command(0x03)?;
        self.send_command(0x03)?;
        self.send_command(0x02)?; // 4-bit mode
        self.send_command(0x28)?; // Function set: 4-bit, 2 lines, 5x8 font
        self.send_command(0x0C)?; // Display on, cursor off
        self.send_command(0x06)?; // Entry mode set
        self.send_command(0x01)?; // Clear screen
        Ok(())
    }
}
