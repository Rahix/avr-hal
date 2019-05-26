avr-hal
=======
`embedded-hal` implementations for AVR microcontrollers.  Based on the register definitions from [`avr-device`](https://github.com/Rahix/avr-device).

## Structure
This repository contains the following components:
* A generic crate containing implementations that can be used chip-independently and macros to create chip-dependent instances of peripheral abstractions.  This crate is named [`avr-hal-generic`](./avr-hal-generic).
* HAL crates for each chip in `chips/`.  These make use of `avr-hal-generic` to create chip-specific definitions.
* Board Support Crates for popular hardware in `boards/`.  They, for the most part, just re-export functionality from the chip-HAL, with the names that are printed on the PCB.

## Status
The following peripherals are supported in `avr-hal-generic`:
- [x] A spinning delay implementation
- [x] `PORTx` peripherals as digital IO (v2)

### HAL Status
The chip-HAL crates currently support the following peripherals:
* [`atmega32u4-hal`](./chips/atmega32u4-hal)
  - [x] Spinning Delay
  - [x] `PORTB`, `PORTC`, `PORTD`, `PORTE`, `PORTF` as digital IO (v2)
* [`attiny85-hal`](./chips/attiny85-hal)
  - [x] Spinning Delay
  - [x] `PORTB` as digital IO (v2)

### Supported Hardware
In `boards/` there are crates for the following hardware.  Please note that this project is in no way affiliated with any of the vendors.

* [Arduino Leonardo](./boards/arduino-leonardo)
  - [Website](https://www.arduino.cc/en/Main/Arduino_BoardLeonardo)
  - Support for basic digital IO
* [Adafruit Trinket (3V3 or 5V)](./boards/trinket) (**not** PRO!)
  - [Website](https://learn.adafruit.com/introducing-trinket)
  - Support for basic digital IO

## Disclaimer
This project is not affiliated with either Microchip (former Atmel) nor any of the Vendors that created the boards supported in this repository.

## License
*avr-hal* is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
