# Changelog
All notable changes to *ravedude* will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Support for the *Nano 168* (that's Arduino Nano clones with an ATmega168).

### Fixed
- Fixed `ravedude` terminating with an error on CTRL+C ([#225]).

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


[Unreleased]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.3...HEAD
[0.1.3]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.2...ravedude-0.1.3
[0.1.2]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.1...ravedude-0.1.2
[0.1.1]: https://github.com/rahix/avr-hal/compare/ravedude-0.1.0...ravedude-0.1.1
[0.1.0]: https://github.com/rahix/avr-hal/releases/tag/ravedude-0.1.0
