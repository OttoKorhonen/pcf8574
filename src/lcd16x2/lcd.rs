use core::error::Error;
use core::fmt;
use embedded_hal::i2c::I2c;
use esp_hal::delay::Delay;
use esp_println::println;

pub enum Commands {
    DisplayAndCursorOn = 0x0F,
    ClearScreen = 0x01,
    ReturnCursorAtStart = 0x02,
    ShiftCursorLeft = 0x04,
    ShiftCursorRight = 0x06,
    ShiftDisplayRight = 0x05,
    ShiftDisplayLeft = 0x07,
    DisplayOnCursorBlinking = 0x0E,
    ForceCursorAtStart = 0x80,
    StartFromSecondLine = 0xC0,
    Form5x7Matrix = 0x38,
    SetCursorFirstLineThirdPosition = 0x83,
    ActivateSecondLine = 0x3C,
    DisplayAndCursorOff = 0x08,
    SetCursorAtSecondLineFirstPosition = 0xC1,
    DisplayOnWithNoVisibleCursor = 0x0C,
    SetCursorAtSecondLineSecondPosition = 0xC2
}

pub enum Byte {
    Target,
    Data,
}

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

// #[derive(Debug)]
pub struct Pcf8574<I2C, E> {
    i2c: I2C,
    address: u8,
    delay: Delay,
    _error: core::marker::PhantomData<E>,
}

impl<E: fmt::Debug> Error for Pcf8574Error<E> {}

impl<I2C: I2c, E> Pcf8574<I2C, E>
where
    I2C: I2c<Error = E>,
    E: fmt::Debug,
{
    pub fn new(i2c: I2C, delay: Delay) -> Result<Self, Pcf8574Error<E>> {
        Ok(Self {
            i2c,
            address: 0x27,
            delay: delay,
            _error: core::marker::PhantomData,
        })
    }

    pub fn search_for_address(&mut self) {
        let mut device_address = 0;
        for address in 0x00..0x78 {
            if self.i2c.write(address, &[0]).is_ok() {
                println!("Device found at address: 0x{:X}", address);
                device_address = address;
            }else{
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

    /// Lähettää "Enable"-pulssin PCF8574:n kautta LCD:lle
    fn set_enable(&mut self, data: u8) -> Result<(), Pcf8574Error<E>> {
        self.i2c.write(self.address, &[data | 0x04]).map_err(Pcf8574Error::I2cError)?; // E=1
        self.delay.delay_millis(5);
        self.i2c.write(self.address, &[data & !0x04]).map_err(Pcf8574Error::I2cError)?; // E=0
        Ok(())
    }

    /// Lähettää komennon LCD:lle
    pub fn send_command(&mut self, cmd: u8) -> Result<(), Pcf8574Error<E>> {
        self.send_byte(cmd, false)
    }

    /// Lähettää merkin LCD:lle
    pub fn send_char(&mut self, ch: char) -> Result<(), Pcf8574Error<E>> {
        self.send_byte(ch as u8, true)
    }

    /// Kirjoittaa merkkijonon LCD:lle
    pub fn write(&mut self, text: &str) -> Result<(), Pcf8574Error<E>> {
        for ch in text.chars() {
            self.send_char(ch)?;
        }
        Ok(())
    }

    /// Alustaa LCD:n käyttöön
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
