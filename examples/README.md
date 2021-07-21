`avr-hal` examples
==================
The subdirectories here contain various examples which demonstrate how to write
firmware using `avr-hal`.  Please note that often examples for a different
board can be easily ported to other hardware, so if you can't find something
for your board, look for examples with other hardware as well.

All examples are ready to use if you have the respective board available.  Just
switch to the subdirectory and run an example via cargo.  For example:

```bash
cd examples/arduino-uno

# Build and run it on a connected board
cargo run --bin uno-blink
```

You need to install [`ravedude`](https://crates.io/crates/ravedude) with `cargo install ravedude` to make
this work.

The Examples
------------

A list with a brief description of the examples. Not all boards provide all examples but many of the examples are easyly portable. If you ported and tested an example you can file a pull request on github.

### Blink

The Blink example is the simplest it just blinks one LED that is already on the board.

### ADC

The Analog Digital Converter displays the Input and Ground Voltages. Then it periodically prints a list of Values corresponding to the analog pins (A0-A7). Hereby if that pin is connected to ground (`GND`) the value will be 0. If the pin is connected to `VIN` or 5V the value will be 1023. The voltages in between those two are linearly mapped.


### I2cdetect

Detecting the inter device protocol i2c. This protocoll is used for a two way data transmission with only two cables.

### Millis

Count the milliseconds since bootup of the device. On a serial character this program answers with the ascii ordinal of the character and the time since elapsed since the bootup of the device.

### Panic

This demonstrates how a custom panic handler can be set in place. With such a handler a LED and/or a serial signal can be emitted in case of any panic in the code.

### USART

The Universal Synchronous and Asynchronous serial Receiver and Transmitter (USART) is a highly flexible serial communication device. This example is the basic demonstration of a serial console. You can send characters and the arduino answers with the characters sent as ascii ordinal.

### Watchdog

On most boards a watchdog chip is installed. This chip can reset the arduino in case it is not responding.

> Attention: On some arduinos the bootloader does not reset the watchdog setting. Meaning the arduino is resetted again during bootup. This results in a rapid flashing arduino that can only be programmed again after beeing unplugged and plugged again. Test the watchdog timer with rather high timeouts like 4 or 8 seconds that way there is enough time to reprogram the arduino (and disable the watchdog again).

Sensor examples
---------------

### Ultrasonic sensor (HC-SR04)

Using an ultrasonic sensor to estimate distances to objects.

### Light sensor (LDR)

Using a light defined resistor to measure the amount of light and react to different brightnesses.