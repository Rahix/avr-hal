avr-hal ![Continuous Integration](https://github.com/Rahix/avr-hal/workflows/Continuous%20Integration/badge.svg) [![arduino-hal docs](https://img.shields.io/badge/docs-arduino--hal-4d76ae)][arduino-hal docs] [![atmega-hal docs](https://img.shields.io/badge/docs-atmega--hal-4d76ae)][atmega-hal docs] [![attiny-hal docs](https://img.shields.io/badge/docs-attiny--hal-4d76ae)][attiny-hal docs]
=======
Hardware Abstraction Layer for AVR microcontrollers and common boards (for example Arduino).  Based on the [`avr-device`](https://github.com/Rahix/avr-device) crate.

## Quickstart
You need a nightly Rust compiler for compiling Rust code for AVR.  The correct version will be installed automatically due to the `rust-toolchain.toml` file.

Install dependencies:

- Ubuntu
  ```bash
  sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential
  ```
- Macos  
  ```bash
  xcode-select --install # if you haven't already done so
  brew tap osx-cross/avr
  brew install avr-gcc avrdude
  ```
- Windows

  Install [Scoop](https://scoop.sh/) using Powershell
  ```PowerShell
  Set-ExecutionPolicy RemoteSigned -Scope CurrentUser # Needed to run a remote script the first time
  irm get.scoop.sh | iex
  ```
  Install avr-gcc and avrdude
  ```
  scoop install avr-gcc
  scoop install avrdude
  ```
  See [Setting up environment](https://github.com/Rahix/avr-hal/wiki/Setting-up-environment) for more information.
  
Next, install ["ravedude"](./ravedude), a tool which seamlessly integrates flashing your board into the usual cargo workflow:

```bash
cargo +stable install ravedude
```

Go into `./examples/arduino-uno` (or the directory for whatever board you want), and run the following commands:

```bash
cd examples/arduino-uno

# Build and run it on a connected board
cargo run --bin uno-blink
```

## Starting your own project
The best way to start your own project is via the [`avr-hal-template`](https://github.com/Rahix/avr-hal-template) which you can easily use with [`cargo-generate`](https://github.com/cargo-generate/cargo-generate):

```bash
cargo install cargo-generate
cargo generate --git https://github.com/Rahix/avr-hal-template.git
```

## Repository Structure
The `avr-hal` repository is a workspace containing all components making up the HAL.  Here is an overview:

### `arduino-hal` [![arduino-hal docs](https://img.shields.io/badge/docs-git-4d76ae)][arduino-hal docs]
`arduino-hal` is the batteries-included HAL for all Arduino & similar boards.  This is what you probably want to use for your projects.  It is intentionally built to abstract away the differences between boards as much as possible.

### `examples/*`
The [examples directory](./examples) contains lots of examples for common hardware.  Do note that not all examples were ported to all boards, but there is a good chance that you can still use the code.  Currently, the [Arduino Uno](./examples/arduino-uno/) crate contains the most examples.

### `mcu/atmega-hal` [![atmega-hal docs](https://img.shields.io/badge/docs-git-4d76ae)][atmega-hal docs] , `mcu/attiny-hal` [![attiny-hal docs](https://img.shields.io/badge/docs-git-4d76ae)][attiny-hal docs]
HAL crates for AVR microcontroller families.  If you have a custom board, you'll want to work with these crates.  Please check their documentation for a list of supported MCUs.

### `avr-hal-generic` [![avr-hal-generic docs](https://img.shields.io/badge/docs-git-4d76ae)][avr-hal-generic docs]
This is a generic crate containing most of the HAL implementations in the form of macros which are instanciated in each HAL crate for the specific MCUs.  If you intend to write drivers that work with any AVR chip, targeting `avr-hal-generic` is probably the best route.

### `avr-specs/`
The `avr-specs/` directory contains rustc target definitions for all supported microcontrollers.  You will need these for compiling rust code for AVR.  The [`avr-hal-template`](https://github.com/Rahix/avr-hal-template) already includes them for convenience.

### [`ravedude`](./ravedude) [![crates.io page](https://img.shields.io/crates/v/ravedude.svg)](https://crates.io/crates/ravedude)
`ravedude` is a utility for seamlessly integrating avrdude and a serial console into the cargo workflow.  With a bit of configuration (check its [README](./ravedude/README.md)!) you can then upload your code to your board and view its output over the serial console by just using `cargo run` as you would normally.

[avr-hal-generic docs]: https://rahix.github.io/avr-hal/avr_hal_generic/index.html
[arduino-hal docs]: https://rahix.github.io/avr-hal/arduino_hal/index.html
[atmega-hal docs]: https://rahix.github.io/avr-hal/atmega_hal/index.html
[attiny-hal docs]: https://rahix.github.io/avr-hal/attiny_hal/index.html

## Disclaimer
This project is not affiliated with either Microchip (former Atmel) nor any of the Vendors that created the boards supported in this repository.

## License
*avr-hal* is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
