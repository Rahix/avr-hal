`avr-modern` examples for attiny402, attiny1614, attiny3224, avr128db28
================================
The subdirectories here contain various examples which demonstrate how to write
firmware using `avrmodern-hal`.  Please note that often examples for a different
chip can be easily ported to others.

Currently only port and uart functionality have been ported from atmega-hal.

Note that Ubuntu has an old avr-gcc compiler suite with incomplete support for modern AVR chips.

Install Arduino and https://github.com/SpenceKonde/megaTinyCore and use that compiler and the UPDI flash program.

Add this to your `.profile`:
```
PATH="$HOME/.arduino15/packages/arduino/tools/avr-gcc/7.3.0-atmel3.6.1-arduino7/bin:$PATH"

```

`attiny402-flash.sh` is an example how to flash.

Adapt `.cargo/config.toml` and uncomment the chip you are using before running `cargo build --release`.

The current rust compiler does not support `target-cpu` for all modern AVR chips, but that is not really a problem.
You can pick any modern chip, but note that you should use `attiny402` for chips with 8kB or less flash memory (these have a 2 byte vector table) and `attiny1614` for bigger flash memories. The amount of flash should be specified with a linker argument (which overrules the CRT file), so for the AVR128DB28 we can use:

```
rustflags = ["-C", "target-cpu=attiny1614", "-C", "link-args=-Wl,--defsym=__TEXT_REGION_LENGTH__=131072"]
```

For programs up to 32kB we need to set the flash mapping:

```
dp.CPU.ccp().write(|w| w.ccp().ioreg()); // remove protection
dp.NVMCTRL.ctrlb().write(|w| w.flmap().bits(0)); // Set the memory flash mapping for AVR128DB28
```

For programs bigger than 32Kb the following works for AVR128DB28
```
rustflags = ["-C", "target-cpu=atxmega128a3", "-C", "link-args=-Wl,--defsym=__DATA_REGION_ORIGIN__=16384", "-C", "link-args=-Wl,--defsym=__DATA_REGION_LENGTH__=16384", "-C", "link-args=-Wl,--defsym=__stack=32767" ]
```
Note that this will copy readonly objects in flash into the 16kB RAM of the AVR128DB28, so you lose some RAM.
For programs less than 32kB the `target-cpu=attiny1614` line specified above combined with the flash mapping, might be better, but in general 16kB RAM will be sufficient.
