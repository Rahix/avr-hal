ravedude [![crates.io page](https://img.shields.io/crates/v/ravedude.svg)](https://crates.io/crates/ravedude)
========
`ravedude` is a CLI utility to make Rust development for AVR microcontrollers
super smooth.  It's a wrapper around `avrdude` and provides easy access to the
target's serial console, similar to the Arduino IDE.

`ravedude` is meant to be used as a cargo "runner".  This allows you to just use
`cargo run` for building, deploying, and running your AVR code!

if you get an `Error: no matching serial port found, use -P or set RAVEDUDE_PORT in your environment` , 
run `cargo run` with set environment variable or adjust `runner = "ravedude {X} -cb {X} -P /dev/ttyUSB{X}"` inside `.cargo/config.toml` (replace {X} with your respective values)

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

(alternatively, if you're using NixOS + Flakes, you can install `ravedude` by
adding `inputs.ravedude.url = "github:Rahix/avr-hal?dir=ravedude";` and then
`environment.systemPackages = [ ravedude.defaultPackage."${system}" ];`.)

Now you need to add *ravedude* to your project.  For example in a project for
Arduino Uno, place the following into your `.cargo/config.toml` (**not in
`Cargo.toml`**):

```toml
[target.'cfg(target_arch = "avr")']
runner = "ravedude uno --open-console --baudrate 57600"
```

And that's all, now just call `cargo run` and watch it do its magic:

<pre style="background-color: #191919; color: #FFF"><font color="#A1EFE4"><b>avr-hal/examples/arduino-uno</b></font> on <font color="#AE81FF"><b>ravedude</b></font> via <font color="#F92672"><b>v1.51.0-nightly </b></font>
<font color="#A6E22E"><b>❯</b></font> cargo run --bin uno-i2cdetect
<font color="#A6E22E"><b>   Compiling</b></font> arduino-uno-examples v0.0.0 (avr-hal/examples/arduino-uno)
<font color="#A6E22E"><b>    Finished</b></font> dev [optimized + debuginfo] target(s) in 1.26s
<font color="#A6E22E"><b>     Running</b></font> `ravedude uno -cb 57600 avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf`
<font color="#A6E22E"><b>       Board</b></font> Arduino Uno
<font color="#A6E22E"><b> Programming</b></font> avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf <font color="#66D9EF"><b>=&gt;</b></font> /dev/ttyACM0

avrdude: AVR device initialized and ready to accept instructions

Reading | ################################################## | 100% 0.00s

avrdude: Device signature = 0x1e950f (probably m328p)
avrdude: erasing chip
avrdude: reading input file &quot;avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf&quot;
avrdude: writing flash (1654 bytes):

Writing | ################################################## | 100% 0.27s

avrdude: 1654 bytes of flash written
avrdude: verifying flash memory against avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf:
avrdude: load data flash data from input file avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf:
avrdude: input file avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf contains 1654 bytes
avrdude: reading on-chip flash data:

Reading | ################################################## | 100% 0.21s

avrdude: verifying ...
avrdude: 1654 bytes of flash verified

avrdude: safemode: Fuses OK (E:00, H:00, L:00)

avrdude done.  Thank you.

<font color="#A6E22E"><b>  Programmed</b></font> avr-hal/target/avr-atmega328p/debug/uno-i2cdetect.elf
<font color="#A6E22E"><b>     Console</b></font> /dev/ttyACM0 at 57600 baud

Write direction test:
-    0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
00:       -- -- -- -- -- -- -- -- -- -- -- -- -- --
10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
30: -- -- -- -- -- -- -- -- 38 39 -- -- -- -- -- --
40: -- -- -- -- -- -- -- -- 48 -- -- -- -- -- -- --
50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
70: -- -- -- -- -- -- -- --

Read direction test:
-    0  1  2  3  4  5  6  7  8  9  a  b  c  d  e  f
00:       -- -- -- -- -- -- -- -- -- -- -- -- -- --
10: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
20: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
30: -- -- -- -- -- -- -- -- 38 39 -- -- -- -- -- --
40: -- -- -- -- -- -- -- -- 48 -- -- -- -- -- -- --
50: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
60: -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
70: -- -- -- -- -- -- -- --
</pre>

## License
*ravedude* is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
