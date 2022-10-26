# Flashing on Linux

Press the reset button on the Feather and then run the commands.
```sh
$ cargo build --bin feather-blink
$ avrdude -v -p m32u4 -cavr109 -P /dev/ttyACM0 -D -Uflash:w:../../target/avr-atmega32u4/debug/feather-blink.elf:e
```
You might need to use a different tty port (e.g. ttyACM1) or press the reset button multiple times to get it to download.
