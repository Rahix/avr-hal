# Changelog
All notable changes to *ravedude* will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]


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


[Unreleased]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.5...HEAD
[0.1.5]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.4...ravedude-0.1.5
[0.1.4]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.3...ravedude-0.1.4
[0.1.3]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.2...ravedude-0.1.3
[0.1.2]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.1...ravedude-0.1.2
[0.1.1]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.0...ravedude-0.1.1
[0.1.0]: https://github.com/rahix/avr-hal/releases/tag/ravedude-0.1.0
