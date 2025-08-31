ravedude [![crates.io page](https://img.shields.io/crates/v/ravedude.svg)](https://crates.io/crates/ravedude)
========
`ravedude` is a CLI utility to make Rust development for AVR microcontrollers
super smooth.  It's a wrapper around `avrdude` and provides easy access to the
target's serial console, similar to the Arduino IDE.

`ravedude` is meant to be used as a cargo "runner".  This allows you to just use
`cargo run` for building, deploying, and running your AVR code!

## Installation
On Linux systems, you'll need pkg-config and libudev development files
installed:

- *Archlinux*: `pacman -S systemd pkgconf`
- *Ubuntu/Debian*: `apt install libudev-dev pkg-config`
- *Fedora*: `dnf install systemd-devel pkgconf-pkg-config`

Next, install the latest version from crates.io with the following command:

```bash
cargo +stable install --locked ravedude
```

Alternatively, if you're using Nix (the package manager) + Flakes, you can
install `ravedude` by adding `inputs.ravedude.url =
"github:Rahix/avr-hal?dir=ravedude";` and use the package
`ravedude.packages."${system}".default`.

## Usage
Now you need to add *ravedude* to your project.  For example in a project for
Arduino Uno, place the following into your `.cargo/config.toml` (**not in
`Cargo.toml`**):

```toml
[target.'cfg(target_arch = "avr")']
runner = "ravedude"
```

Then, create a `Ravedude.toml` file next to your `Cargo.toml`:

```toml
[general]
board = "uno"

# ravedude should open a serial console after flashing
open-console = true
serial-baudrate = 57600
```

For more info about the configuratiom, please check [`Ravedude.toml`
Format](#ravedudetoml-format) below.

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

## `Ravedude.toml` Format
For off-the-shelf AVR boards that are already supported by ravedude, configuration is very
simple.  Just two lines in `Ravedude.toml` are all that is necessary:

```toml
[general]
board = "<board-name-here>"
```

Depending on your project, you may want to add any of the following additional options:

```toml
[general]
# if auto-detection is not working, you can hard-code a specific port here
# (the port can also be passed via the RAVEDUDE_PORT environment variable)
port = "/dev/ttyACM0"

# ravedude should open a serial console after flashing
open-console = true

# console output mode. Can be ascii, hex, dec or bin
output-mode = "ascii"

# Print a newline after this byte
# not used with output_mode ascii
# hex (0x) and bin (0b) notations are supported.
# matching chars/bytes are NOT removed
# to add newlines after \n (in non-ascii mode), use \n, 0x0a or 0b00001010
# newline-on = '\n'

# Print a newline after n bytes
# not used with output_mode ascii
# defaults to 16 for hex and dec and 8 for bin
# if dividable by 4, bytes will be grouped to 4
# newline-after = 16

# baudrate for the serial console (this is **not** the avrdude flashing baudrate)
serial-baudrate = 57600

# time to wait for the board to be reset (in milliseconds).  this skips the manual prompt for resetting the board.
reset-delay = 2000
```

#### Custom Boards
For boards that are not yet part of _ravedude_, you can specify all relevant options yourself
in `Ravedude.toml`.  It works like this:

```toml
[general]
# port = ...
# open-console = true
# serial-baudrate = 57600

[board]
name = "Custom Arduino Uno"

[board.reset]
# The board automatically resets when attempting to flash
automatic = true

[board.avrdude]
# avrdude configuration
programmer = "arduino"
partno = "atmega328p"
baudrate = -1
do-chip-erase = true
```

For reference, take a look at [`boards.toml`](https://github.com/Rahix/avr-hal/blob/main/ravedude/src/boards.toml).

## License
*ravedude* is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
