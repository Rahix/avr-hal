# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Note**: As this project is still very early in its lifecycle, we don't have
proper releases yet.  Instead, the CHANGELOG will document changes over time so
people already using the crates have a reference what is changing upstream.

## [2021-02-01 - 2021-02-07][2021-05]
### Changed
- Large refactor of the USART implementation ([#116]).  The user-facing API is
  _mostly_ the same though there might be some small details which have changed.
  Especially the user-facing interrupt methods look different now.

### Fixed
- Removed a bashism in the runner scripts ([#126]).

[#116]: https://github.com/Rahix/avr-hal/pull/116
[#126]: https://github.com/Rahix/avr-hal/pull/126


## [2020-11-30 - 2021-01-31][2021-01]
### Added
- A method to reconfigure an SPI peripheral ([#112]).

### Fixed
- Fixed I2C implementation for ATmega328PB ([#107]).
- Fixed SPI implementation for ATmega328PB ([#111]).

[#107]: https://github.com/Rahix/avr-hal/pull/107
[#111]: https://github.com/Rahix/avr-hal/pull/111
[#112]: https://github.com/Rahix/avr-hal/pull/112


## [2020-11-23 - 2020-11-29][2020-48]
### Added
- An example implementation of the Arduino `millis()` function:
  [`uno-millis.rs`][uno-millis].  There is also a blog post with a code
  walkthrough: <https://blog.rahix.de/005-avr-hal-millis>

### Changed
- `I2c` was renamed to `I2cMaster`.  There is a (deprecated) alias for the old
  name but where possible, the new one should be used ([#102]).

[uno-millis]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-uno/examples/uno-millis.rs
[#102]: https://github.com/Rahix/avr-hal/pull/102


## [2020-11-16 - 2020-11-22][2020-47]
### Changed
- The SPI driver now wraps the chip-select (CS) pin to enforce proper usage
  ([#103]).  Downstream code must change slightly to use the new API (example
  for Arduino Uno):
  ```diff
  -    let mut cs_pin = pins.d10.into_output(&mut pins.ddr);

  -    let mut spi = spi::Spi::new(
  +    let (mut spi, mut cs_pin) = spi::Spi::new(
           dp.SPI,
           pins.d13.into_output(&mut pins.ddr),
           pins.d11.into_output(&mut pins.ddr),
           pins.d12.into_pull_up_input(&mut pins.ddr),
  +        pins.d10.into_output(&mut pins.ddr),
           spi::Settings::default(),
       );
  ```

### Fixed
- Fixed a soundness issue in the SPI driver.  The hardware requires proper setup
  of the CS pin which was not guaranteed.  An API change was made to enforce the
  necessary invariant ([#103]).

[#103]: https://github.com/Rahix/avr-hal/pull/103


## [2020-11-09 - 2020-11-15][2020-46]
### Added
- `arduino-mega2560`: A `usart` module with type aliases for all other USART
  peripherals apart from `Usart0` ([#100]).

### Changed
- The `avr-hal-generic::serial` module was renamed to `avr-hal-generic::usart`
  for consistency ([`493546530eb8`]).

[#100]: https://github.com/Rahix/avr-hal/pull/100
[`493546530eb8`]: https://github.com/Rahix/avr-hal/commit/493546530eb87d47ae1eee507e9309be590688d4


## [2020-11-02 - 2020-11-08][2020-45]
### Added
- Serial/USART: The `Baudrate` type for more precise control over the baudrate
  selection ([#88]).

### Changed
- The constructor for the `Serial`/USART driver must now be called with
  a `Baudrate` value, not an integer ([#88]).  In practice, the easiest way to
  do this is using the `.into_baudrate()` conversion method.  As an example, the
  needed change might look like this:
  ```diff
   let mut serial = arduino_uno::Serial::new(
       dp.USART0,
       pins.d0,
       pins.d1.into_output(&mut pins.ddr),
  -    57600,
  +    57600.into_baudrate(),
   );
  ```

[#88]: https://github.com/Rahix/avr-hal/pull/88


## [2020-10-26 - 2020-11-01][2020-44]
### Added
- Support for `ATmega328PB` ([#96]).

### Changed
- `atmega328p-hal`: You must now select a chip via either the `atmega328p` or
  `atmega328pb` features.  Selecting no or more than one chip will lead to
  a compile-error.  As an example:
  ```toml
  [dependencies.atmega328p-hal]
  git = "https://github.com/Rahix/avr-hal.git"
  rev = "<latest git commit hash>"
  features = ["atmega328p"]
  ```
- In HAL crates, the `avr-hal-generic` crate is no longer renamed to `avr-hal`
  as this will just lead to confusion and problems down the line ([#89]).
- In HAL crates, the peripheral access crate (submodule of `avr-device`) is
  imported as `pac` everywhere instead of using the actual device name.  This
  will make it easier to support multiple devices in a single HAL in the future
  ([#89]).
- The reexported modules in board crates were cleaned up:  The HAL crate is now
  reexported as `hal` and the PAC crates as `pac` ([#89]).

[#89]: https://github.com/Rahix/avr-hal/pull/89
[#96]: https://github.com/Rahix/avr-hal/pull/96


## [2020-10-12 - 2020-10-18][2020-42]
### Added
- The runner scripts now have better suport for MacOS out of the box ([#87]).

[#87]: https://github.com/Rahix/avr-hal/pull/87


## [2020-10-05 - 2020-10-11][2020-41]
### Added
- Added a script for automatically synchronizing target specs with upstream
  ([#84]).

### Changed
- Moved all target specs (like `avr-atmega32u4.json`) to a single directory
  named `avr-specs/` ([#82]).
- Updated various settings in all target specs for better compatibility and
  general cleanup ([#85]).

### Fixed
- Fixed a number of issues in the `sparkfun-pro-micro` board crate ([#74]).

[#74]: https://github.com/Rahix/avr-hal/pull/74
[#82]: https://github.com/Rahix/avr-hal/pull/82
[#84]: https://github.com/Rahix/avr-hal/pull/84
[#85]: https://github.com/Rahix/avr-hal/pull/85


## [2020-09-14 - 2020-09-20][2020-38]
### Changed
- The `Pins::new()` for `arduino-leonardo` now also consumes the `PORTF`
  peripheral and exposes its pins as `a0` - `a5` ([#73]).

### Fixed
- Fixed a nightly regression which broke our target specs ([#72]).

[#73]: https://github.com/Rahix/avr-hal/pull/73
[#72]: https://github.com/Rahix/avr-hal/pull/72


## [2020-09-07 - 2020-09-13][2020-37]
### Added
- Added support for Arduino Nano (in `arduino-uno` board crate) ([#69]).
- Added support for `ADC6` and `ADC7` pins in `atmega328p-hal` ([#69]).

### Fixed
- Reduced the overhead from `delay_us(u32)` ([#68]).

[#68]: https://github.com/Rahix/avr-hal/pull/68
[#69]: https://github.com/Rahix/avr-hal/pull/69


## [2020-08-31 - 2020-09-06][2020-36]
### Added
- Support for Sparkfun's Pro Micro board ([#62]).
- SPI driver now implements the `blocking::spi::{Transfer, Write}` traits
  ([#66]).

### Fixed
- Fixed not resetting `U2X` bit in `USART` driver which leads to wrong baudrates
  in some situations (reported in [#67], fixed in [`7caed3a995e2`]).
- Fixed I2C/TWI driver not resetting all bits during initialization
  ([`3116e9ad5441`]).

[#62]: https://github.com/Rahix/avr-hal/pull/62
[#66]: https://github.com/Rahix/avr-hal/pull/66
[#67]: https://github.com/Rahix/avr-hal/pull/67
[`7caed3a995e2`]: https://github.com/Rahix/avr-hal/commit/7caed3a995e22f107b87e69f53679b0b4a3eb758
[`3116e9ad5441`]: https://github.com/Rahix/avr-hal/commit/3116e9ad544120fffd55c60fcca58b51e61f934b


## 2019-05-11 - 2020-08-31
Please look at the git log for changes before this point :)




[2021-05]: https://github.com/Rahix/avr-hal/compare/master@%7B2021-01-31%7D...master@%7B2021-02-07%7D
[2021-01]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-11-29%7D...master@%7B2021-01-31%7D
[2020-48]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-11-22%7D...master@%7B2020-11-29%7D
[2020-47]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-11-15%7D...master@%7B2020-11-22%7D
[2020-46]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-11-08%7D...master@%7B2020-11-15%7D
[2020-45]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-11-01%7D...master@%7B2020-11-08%7D
[2020-44]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-10-25%7D...master@%7B2020-11-01%7D
[2020-42]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-10-11%7D...master@%7B2020-10-18%7D
[2020-41]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-10-04%7D...master@%7B2020-10-11%7D
[2020-38]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-09-13%7D...master@%7B2020-09-20%7D
[2020-37]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-09-06%7D...master@%7B2020-09-13%7D
[2020-36]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-08-30%7D...master@%7B2020-09-06%7D
