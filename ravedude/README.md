ravedude
========
`ravedude` is a CLI utility to make Rust development for AVR microcontrollers
super smooth.  It's a wrapper around `avrdude` and provides easy access to the
target's serial console, similar to the Arduino IDE.

`ravedude` is meant to be used as a cargo "runner".  This allows you to just use
`cargo run` for building, deploying, and running your AVR code!  For example, in
a project for Arduino Uno, place the following into your `.cargo/config.toml`
(**not in `Cargo.toml`**):

```toml
[target.'cfg(target_arch = "avr")']
runner = "ravedude uno --open-console --baudrate 57600"
```

## Installation
On Linux systems, you'll need pkg-config and libudev development files
installed:

- *Archlinux*: `pacman -S systemd pkgconf`
- *Ubuntu/Debian*: `apt install libudev-dev pkg-config`
- *Fedora*: `dnf install systemd-devel pkgconf-pkg-config`

Next, install the latest version from crates.io with the following command:

```bash
cargo install ravedude
```

Finally, add *ravedude* as a runner to your project like shown above!

## License
*ravedude* is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
