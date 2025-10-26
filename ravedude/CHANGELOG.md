# Changelog
All notable changes to *ravedude* will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


## [0.2.2] - 2025-10-26
### Added
- Added a `general.console-port` option with a corresponding
  `-C`/`--console-port` CLI option to set a console port that's separate from
  the programming port ([#683]).
- Support for the AVR `butterfly` board ([#685]).
- Support for the _SparkFun Pro Mini 3v3_ (`promini-3v3`) board ([#687]).

[#683]: https://github.com/Rahix/avr-hal/pull/683
[#685]: https://github.com/Rahix/avr-hal/pull/685
[#687]: https://github.com/Rahix/avr-hal/pull/687


## [0.2.1] - 2025-08-31
### Added
- Added a `general.output-mode` setting in `Ravedude.toml` ([#549]).

  This setting can be used to set a numeric output mode for the serial console
  instead of the default ascii representation.  One of `ascii`, `hex`, `dec`,
  or `bin` can be chosen.

  Additionaly either `general.newline-after = N` can be set to print a newline
  after N bytes, or `general.newline-on = "\n"` to print a newline when a
  specific byte is received.

  For example:
  ```toml
  [general]
  output-mode = "hex"
  newline-after = 4
  ```

[#549]: https://github.com/Rahix/avr-hal/pull/549


## [0.2.0] - 2025-04-22
### Changed
- **BREAKING** ravedude is now configured using a `Ravedude.toml` configuration
  file.  This is more flexible because boards are no longer hard-coded into the
  internals of the tool.  Custom boards can now be defined and settings for
  off-the-shelf boards can be adjusted.

  For documentation, please see [`Ravedude.toml` Format](https://github.com/Rahix/avr-hal/blob/main/ravedude/README.md#ravedudetoml-format).

  This redesign was implemented in [#522].
- Updated to clap 3.0 ([#631]).

### Added
- Added support for the Mighty Core ATMEGA1284P board ([#607]).
- Ravedude now gives descriptive errors when one of the dependency tools is not
  installed ([#649]).

### Fixed
- Fixed trinket board definition ([#626]).

[#522]: https://github.com/Rahix/avr-hal/pull/522
[#607]: https://github.com/Rahix/avr-hal/pull/607
[#626]: https://github.com/Rahix/avr-hal/pull/626
[#631]: https://github.com/Rahix/avr-hal/pull/631
[#649]: https://github.com/Rahix/avr-hal/pull/649


## [0.1.8] - 2024-03-15
### Added
- Added the `--debug-avrdude` option to print the `avrdude` invocation used to
  flash the binary.

### Fixed
- Fixed incompatibility with `avrdude` versions 7.0 to 7.2 ([#513]).  All
  `avrdude` versions should work again with this release.
- Fixed a crash on Windows when attempting to print non-ascii bytes that were
  received on the serial console ([#516]).

[#513]: https://github.com/Rahix/avr-hal/pull/513
[#516]: https://github.com/Rahix/avr-hal/pull/516


## [0.1.7] - 2024-02-24
### Added
- Added support for using `ravedude` with `avrdude` version 7.x ([#508]).

### Fixed
- Added missing signature for ATmega328PB ([#462]).

[#462]: https://github.com/Rahix/avr-hal/pull/462
[#508]: https://github.com/Rahix/avr-hal/pull/508


## [0.1.6] - 2023-11-06
### Added
- Support for *Arduino Mega 1280* ([#362]).
- Support for *Arduino Duemilanove* ([#404]).
- Support for *SparkFun ProMini 5V* ([#435]).

### Fixed
- Fixed serial console not working on Windows ([#433]).

[#362]: https://github.com/Rahix/avr-hal/pull/362
[#404]: https://github.com/Rahix/avr-hal/pull/404
[#433]: https://github.com/Rahix/avr-hal/pull/433
[#435]: https://github.com/Rahix/avr-hal/pull/435


## [0.1.5] - 2022-09-18
### Added
- Added a `--reset-delay` option as an alternative to interactively waiting
  for the user to confirm board reset ([#275]).
- Added support for newer Arduino Nano boards which have a different baudrate setting.
  The board is called `nano-new` ([#247]).

### Fixed
- Fixed bug in avrdude version number parsing code which didn't allow version
  numbers with more than two components ([#260]).
- Fixed ATtiny85 avrdude config ([#274]).

[#247]: https://github.com/Rahix/avr-hal/pull/247
[#260]: https://github.com/Rahix/avr-hal/pull/260
[#274]: https://github.com/Rahix/avr-hal/pull/274
[#275]: https://github.com/Rahix/avr-hal/pull/275


## [0.1.4] - 2022-02-08
### Added
- Support for the *Nano 168* (that's Arduino Nano clones with an ATmega168).
- Support for the *Adafruit Trinket* ([#179]).
- Added support for the Arduino Leonardo auto-reset method even though
  `avr-hal` does not yet support it ([#207]).
- Added a check whether the `avrdude` version is recent enough to support all
  features we need ([#221]).

### Fixed
- Fixed `ravedude` terminating with an error on CTRL+C ([#225]).

[#179]: https://github.com/Rahix/avr-hal/pull/179
[#207]: https://github.com/Rahix/avr-hal/pull/207
[#221]: https://github.com/Rahix/avr-hal/pull/221
[#225]: https://github.com/Rahix/avr-hal/pull/225


## [0.1.3] - 2021-07-11
### Added
- Support for *Arduino Mega 2560*.
- Support for *Arduino Diecimila* (untested).
- Support for the *SparkFun ProMicro*.
- Support for *Arduino Micro*.
- Support for the *Trinket Pro*.


## [0.1.2] - 2021-03-21
### Removed
- The `--no-program` flag was removed, just don't pass a binary if you want to
  skip flashing.

### Fixed
- Fixed support for *Arduino Nano*.


## [0.1.1] - 2021-03-13
### Fixed
- Fixed broken metadata.


## [0.1.0] - 2021-03-06
Initial, miminal version.


[Unreleased]: https://github.com/rahix/avr-hal/compare/ravedude-0.2.2...HEAD
[0.2.2]: https://github.com/rahix/avr-hal/compare/ravedude-0.2.1...ravedude-0.2.2
[0.2.1]: https://github.com/rahix/avr-hal/compare/ravedude-0.2.0...ravedude-0.2.1
[0.2.0]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.8...ravedude-0.2.0
[0.1.8]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.7...ravedude-0.1.8
[0.1.7]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.6...ravedude-0.1.7
[0.1.6]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.5...ravedude-0.1.6
[0.1.5]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.4...ravedude-0.1.5
[0.1.4]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.3...ravedude-0.1.4
[0.1.3]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.2...ravedude-0.1.3
[0.1.2]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.1...ravedude-0.1.2
[0.1.1]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.0...ravedude-0.1.1
[0.1.0]: https://github.com/rahix/avr-hal/releases/tag/ravedude-0.1.0
