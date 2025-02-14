# PCF8574

## Overview
This is a driver for PCF8574 written in Rust programming language and tested using ESP32 microcontroller. The PCF8574 chip is used to control a 16 x 2 LCD display.

### Datasheets:
LCD display
> https://www.etechnophiles.com/wp-content/uploads/2023/02/16-X2-LCD-Datasheet.pdf

PCF8574
> https://www.ti.com/lit/ds/symlink/pcf8574.pdf?ts=1627006546204

## Build instructions

### ESP32-based boards
- cargo build --release --features esp32
- cargo build --release --features esp32s2
- cargo build --release --features esp32s3
- cargo build --release --features esp32c3

### Raspberry Pi
- cargo build --release --features rp-pico  
- cargo build --release --features rpi 

## Supported platforms

| Platform           | Status      |  
|--------------------|------------|  
| ESP32C3           | ✅ Tested  |  
| ESP32 / ESP32S2 / ESP32S3 | ⚠ Not tested |  
| Raspberry Pi Pico | ⚠ Not tested |  
| Raspberry Pi 4/5  | ⚠ Not tested |
