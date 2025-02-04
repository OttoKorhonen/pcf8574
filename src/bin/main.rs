#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, OutputOpenDrain, Pull};
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::prelude::*;
use lcd_display::lcd16x2::Pcf8574;
use lcd_display::enums::Commands;

#[entry]
fn main() -> ! {
    let _peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    let sda = OutputOpenDrain::new(_peripherals.GPIO21, Level::High, Pull::Up);
    let scl = OutputOpenDrain::new(_peripherals.GPIO22, Level::High, Pull::Up);


    let mut i2c = I2c::new(
        _peripherals.I2C0,
        Config {
            frequency: 100_u32.kHz(),
            timeout: Some(1000),
        },
    )
    .with_sda(sda)
    .with_scl(scl);

    let delay = Delay::new();
    let mut lcd = Pcf8574::new(&mut i2c, delay).unwrap();

    esp_println::logger::init_logger_from_env();
    
    lcd.initialize_lcd().unwrap();
    lcd.write("Hello, Rust!").unwrap();
    
    loop {

        lcd.write("testi").unwrap();

        delay.delay(500.millis());

        lcd.set_command(Commands::SetCursorAtSecondLineFirstPosition).unwrap();
        delay.delay(500.millis());
        lcd.write("uusi viesti").unwrap();
    }
}
