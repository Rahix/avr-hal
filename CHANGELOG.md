# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Note**: As this project is still very early in its lifecycle, we don't have
proper releases yet.  Instead, the CHANGELOG will document changes over time so
people already using the crates have a reference what is changing upstream.

## [2020-10-26 - 2020-11-01][2020-44]
### Changed
- In HAL crates, the `avr-hal-generic` crate is no longer renamed to `avr-hal`
  as this will just lead to confusion and problems down the line ([#89]).
- In HAL crates, the peripheral access crate (submodule of `avr-device`) is
  imported as `pac` everywhere instead of using the actual device name.  This
  will make it easier to support multiple devices in a single HAL in the future
  ([#89]).
- The reexported modules in board crates were cleaned up:  The HAL crate is now
  reexported as `hal` and the PAC crates as `pac` ([#89]).

[#89]: https://github.com/Rahix/avr-hal/pull/89


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




[2020-44]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-10-25%7D...master@%7B2020-11-01%7D
[2020-42]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-10-11%7D...master@%7B2020-10-18%7D
[2020-41]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-10-04%7D...master@%7B2020-10-11%7D
[2020-38]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-09-13%7D...master@%7B2020-09-20%7D
[2020-37]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-09-06%7D...master@%7B2020-09-13%7D
[2020-36]: https://github.com/Rahix/avr-hal/compare/master@%7B2020-08-30%7D...master@%7B2020-09-06%7D
