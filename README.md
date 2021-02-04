avr-hal [![Build Status](https://travis-ci.com/Rahix/avr-hal.svg?branch=master)](https://travis-ci.com/Rahix/avr-hal)
=======
`embedded-hal` implementations for AVR microcontrollers.  Based on the register definitions from [`avr-device`](https://github.com/Rahix/avr-device).

- [Quickstart](#quickstart)
- [Starting your own project](#starting-your-own-project)
- [Repository Structure](#repository-structure)
  - [Supported MCUs](#supported-mcus)
  - [Supported Boards](#supported-boards)

## Quickstart
You need a nightly Rust compiler for compiling Rust code for AVR.  **Note**: Due to a regression, versions after `nightly-2021-01-07` are currently broken (see [#124](https://github.com/Rahix/avr-hal/issues/124)).  Please use that version of the compiler for now.  You can install it using

```bash
rustup toolchain install nightly-2021-01-07
```

Go into `./boards/arduino-uno` (or the directory for whatever board you want), and run the following commands:

```bash
cd boards/arduino-uno

# Now you are ready to build your first avr blink example!
cargo +nightly-2021-01-07 build --example uno-blink

# For some boards, you can even run it directly (this will attempt to flash it
# onto a connected board):
cargo +nightly-2021-01-07 run --example uno-blink

# For others, you can find the binary file in
ls ../../target/avr-atmega328p/debug/examples/uno-blink.elf
# and e.g. create an ihex file using
avr-objcopy -S -j .text -j .data -O ihex uno-blink.elf uno-blink.hex
```

## Starting your own project
This is a step-by-step guide for creating a new project targeting Arduino Uno (`ATmega328P`).  You can of course apply the same steps for any other microcontroller.

1. Start by creating a new project:
   ```bash
   cargo new --bin avr-example
   cd avr-example
   ```
2. If you're using rustup, you probably want to set an override for this directory, to use the nightly toolchain:
   ```bash
   rustup override set nightly
   ```
3. Copy the target description for your MCU from [`avr-specs/`](./avr-specs) (e.g. [`avr-atmega328p.json`](avr-specs/avr-atmega328p.json)) into your project.
4. Create a file `.cargo/config.toml` with the following content:
   ```toml
   [build]
   target = "avr-atmega328p.json"

   [unstable]
   build-std = ["core"]
   ```
5. Fill `Cargo.toml` with these additional directives:
   ```toml
   [dependencies]
   # A panic handler is needed.  This is a crate with the most basic one.
   # The `leonardo-panic` example shows a more elaborate version.
   panic-halt = "0.2.0"

   [dependencies.arduino-uno]
   git = "https://github.com/rahix/avr-hal"
   rev = "<insert latest git-commit hash here>"
   # ^- Pin the dependency to a specific version.  You should use the latest
   # commit hash from the avr-hal master branch.  You can find it here:
   #
   #    https://github.com/rahix/avr-hal/commits/master

   # Configure the build for minimal size
   [profile.dev]
   panic = "abort"
   lto = true
   opt-level = "s"

   [profile.release]
   panic = "abort"
   codegen-units = 1
   debug = true
   lto = true
   opt-level = "s"
   ```
   **Note**: If you at some point want to update to a newer version of `avr-hal`, you just need to put a later commit hash into the `rev =` field.  For any breaking changes which might require you to fix something in your code, read the [CHANGELOG](https://github.com/Rahix/avr-hal/blob/master/CHANGELOG.md).
6. Start your project with this basic template:
   ```rust
   #![no_std]
   #![no_main]

   // Pull in the panic handler from panic-halt
   extern crate panic_halt;

   use arduino_uno::prelude::*;

   #[arduino_uno::entry]
   fn main() -> ! {
       let dp = arduino_uno::Peripherals::take().unwrap();

       unimplemented!()
   }
   ```
7. Build with these commands (make sure you're using _nightly_ rust!):
   ```bash
   cargo build
   # or
   cargo build --release
   ```
   and find your binary in `target/avr-atmega328p/debug/` (or `target/avr-atmega328p/release`).

8. (**Optional**): To make development as smooth as possible, you can configure a cargo runner for your board.  This repository contains a few example scripts (e.g. [`uno-runner.sh`][uno-runner], [`leonardo-runner.sh`][leonardo-runner]) which you can copy into your project.  You'll then need to add the following section to your `.cargo/config.toml`:
   ```toml
   [target.'cfg(target_arch = "avr")']
   runner = "./uno-runner.sh"
   ```
   And that's it, now you can build an run your project in a single command!
   ```bash
   cargo run
   # or
   cargo run --release
   ```

[leonardo-runner]: ./boards/arduino-leonardo/leonardo-runner.sh
[uno-runner]: ./boards/arduino-uno/uno-runner.sh

## Repository Structure
This repository contains the following components:
* A generic crate containing implementations that can be used chip-independently and macros to create chip-dependent instances of peripheral abstractions.  This crate is named [`avr-hal-generic`](./avr-hal-generic).
* HAL crates for each MCU in `chips/`.  These make use of `avr-hal-generic` to create chip-specific definitions.
* Board Support Crates for popular hardware in `boards/`.  They, for the most part, just re-export functionality from the chip-HAL, with the names that are printed on the PCB.

### Supported MCUs
The following HAL crates currently exist.  Take a look at the docs for more details on what's supported.

* [`atmega168-hal`](./chips/atmega168-hal) - [Crate Documentation](https://rahix.github.io/avr-hal/atmega168_hal)
  - [x] ADC
  - [x] Digital IO
  - [x] I2C using `TWI`
  - [x] PWM
  - [x] SPI
  - [x] Spinning Delay
  - [x] USART Serial
* [`atmega2560-hal`](./chips/atmega2560-hal) - [Crate Documentation](https://rahix.github.io/avr-hal/atmega2560_hal)
  - [x] ADC (no differential channels yet)
  - [x] Digital IO
  - [x] I2C using `TWI`
  - [x] SPI
  - [x] Spinning Delay
  - [x] USART Serial
* [`atmega328p-hal`](./chips/atmega328p-hal) - [Crate Documentation](https://rahix.github.io/avr-hal/atmega328p_hal)
  - [x] ADC
  - [x] Digital IO
  - [x] I2C using `TWI`
  - [x] SPI
  - [x] Spinning Delay
  - [x] USART Serial
* [`atmega32u4-hal`](./chips/atmega32u4-hal) - [Crate Documentation](https://rahix.github.io/avr-hal/atmega32u4_hal)
  - [x] ADC (no differential channels yet)
  - [x] Digital IO
  - [x] I2C using `TWI`
  - [x] SPI
  - [x] Spinning Delay
  - [x] USART Serial
* [`attiny85-hal`](./chips/attiny85-hal) - [Crate Documentation](https://rahix.github.io/avr-hal/attiny85_hal)
  - [x] Digital IO
  - [x] Spinning Delay
* [`attiny88-hal`](./chips/attiny88-hal) - [Crate Documentation](https://rahix.github.io/avr-hal/attiny88_hal)
  - [x] Digital IO
  - [x] I2C using `TWI`
  - [x] SPI
  - [x] Spinning Delay

### Supported Boards
In `boards/` there are crates for the following hardware.  Please note that this project is in no way affiliated with any of the vendors.

Each board crate comes with a few examples showing how to use them.  For more details, follow the links to the documentation.

* [Arduino diecimila](./boards/arduino-diecimila) - [Crate Documentation](https://rahix.github.io/avr-hal/arduino_diecimila)
  - [Website](https://www.arduino.cc/en/Main/Arduino_BoardDiecimila)
* [Arduino Leonardo](./boards/arduino-leonardo) - [Crate Documentation](https://rahix.github.io/avr-hal/arduino_leonardo)
  - [Website](https://www.arduino.cc/en/Main/Arduino_BoardLeonardo)
* [Arduino Uno](./boards/arduino-uno) - [Crate Documentation](https://rahix.github.io/avr-hal/arduino_uno)
  - [Website](https://store.arduino.cc/usa/arduino-uno-rev3)
* [Arduino Nano](./boards/arduino-uno) (via `arduino-uno` crate) - [Crate Documentation](https://rahix.github.io/avr-hal/arduino_uno)
  - [Website](https://store.arduino.cc/arduino-nano)
* [Arduino Mega 2560](./boards/arduino-mega2560) - [Crate Documentation](https://rahix.github.io/avr-hal/arduino_mega2560)
  - [Website](http://arduino.cc/en/Main/ArduinoBoardMega2560)
* [Adafruit Trinket (3V3 or 5V)](./boards/trinket) (**not** PRO!) - [Crate Documentation](https://rahix.github.io/avr-hal/trinket)
  - [Website](https://learn.adafruit.com/introducing-trinket)
* [BigAVR 6](./boards/bigavr6) - [Crate Documentation](https://rahix.github.io/avr-hal/bigavr6)

## Disclaimer
This project is not affiliated with either Microchip (former Atmel) nor any of the Vendors that created the boards supported in this repository.

## License
*avr-hal* is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
