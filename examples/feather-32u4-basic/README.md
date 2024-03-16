# Flashing

```sh
$ avrdude -v -p m32u4 -cavr109 -P /dev/ttyACM0 -D -Uflash:w:target/avr-atmega32u4/release/sby-buoy-feather.elf:e
```
