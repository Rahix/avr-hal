avr-hal
=======
`embedded-hal` implementations for AVR microcontrollers.  Based on the register definitions from [`avr-device`](https://github.com/Rahix/avr-device).

## Quickstart
Go into `./boards/arduino-leonardo`, and run the following commands:
```bash
# Tell rustc where the target description can be found
# (it is in this directory)
export RUST_TARGET_PATH="$(pwd)"

# Set the toolchain to your avr toolchain.  The name might
# be different, depending on how you installed avr-rust.
export RUSTUP_TOOLCHAIN=avr

# xargo needs a path to the rust sources to build libcore.
export XARGO_RUST_SRC="/path/to/avr-rust/src"

# Now you are ready to build your first avr blink example!
xargo build --target avr-atmega32u4 --example leonardo-blink --release

# Finally, convert it into a .hex file that you can flash using avr-dude
../../mkhex.sh --release leonardo-blink

ls -l ../../target/leonardo-blink.hex
```

## Starting your own project
You need at least the following:
* A target description for the chip you are using.  In most cases, just copy the one from this repo.
* The `Xargo.toml` file, as found in this repo.
* The Cargo profiles, as found in `Cargo.toml` of this repo.
* Add the board-support-crate for your hardware as a dependency, and also include `panic-halt` to provide a panic implementation.

In the end, your `Cargo.toml` should contain the following:
```toml
[package]
...

[dependencies]
panic-halt = "0.2.0"

[dependencies.arduino-leonardo]
path = "/path/to/avr-hal/boards/arduino-leonardo"

[profile.dev]
panic = "abort"
codegen-units = 1
incremental = false
lto = true

[profile.release]
panic = "abort"
codegen-units = 1
debug = false
lto = true
```

## Structure
This repository contains the following components:
* A generic crate containing implementations that can be used chip-independently and macros to create chip-dependent instances of peripheral abstractions.  This crate is named [`avr-hal-generic`](./avr-hal-generic).
* HAL crates for each chip in `chips/`.  These make use of `avr-hal-generic` to create chip-specific definitions.
* Board Support Crates for popular hardware in `boards/`.  They, for the most part, just re-export functionality from the chip-HAL, with the names that are printed on the PCB.

## Status
The following peripherals are supported in `avr-hal-generic`:
- [x] A spinning delay implementation
- [x] `PORTx` peripherals as digital IO (v2)
- [x] A TWI based I2C implementation
- [X] SPI primary-mode implementation

### HAL Status
The chip-HAL crates currently support the following peripherals:
* [`atmega328p-hal`](./chips/atmega328p-hal)
  - [x] Spinning Delay
  - [x] `PORTB`, `PORTC`, `PORTD` as digital IO (v2)
  - [x] `USART0` for serial communication
  - [x] I2C using `TWI`
  - [x] SPI
* [`atmega32u4-hal`](./chips/atmega32u4-hal)
  - [x] Spinning Delay
  - [x] `PORTB`, `PORTC`, `PORTD`, `PORTE`, `PORTF` as digital IO (v2)
  - [x] `USART1` for serial communication
  - [x] I2C using `TWI`
  - [x] SPI
* [`attiny85-hal`](./chips/attiny85-hal)
  - [x] Spinning Delay
  - [x] `PORTB` as digital IO (v2)

### Supported Hardware
In `boards/` there are crates for the following hardware.  Please note that this project is in no way affiliated with any of the vendors.

* [Arduino Leonardo](./boards/arduino-leonardo)
  - [Website](https://www.arduino.cc/en/Main/Arduino_BoardLeonardo)
  - Support for basic digital IO and SPI
* [Arduino Uno](./boards/arduino-uno)
  - [Website](https://store.arduino.cc/usa/arduino-uno-rev3)
  - Support for basic digital IO and SPI
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
