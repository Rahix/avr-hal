#![no_std]
#![no_main]

// Proof of concept, all hacked together right now...
//
// Compile with the dirty avr-atmega4809.json target (which actually uses atxmega32a4 ...).
//
// Flashing works using the following command, based on the Arduino stuff (not verified anything
// else yet):
//
// stty -F /dev/ttyACM0 hup 1200 && \
//      echo 1>/dev/ttyACM0 && \
//      ~/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/bin/avrdude \
//          -C${HOME}/.arduino15/packages/arduino/tools/avrdude/6.3.0-arduino17/etc/avrdude.conf \
//          -v -patmega4809 -cjtag2updi -P/dev/ttyACM0 -b115200 -e -D \
//          -Uflash:w:../../target/avr-atmega4809/release/every-blink.elf:e

use panic_halt as _;

use embedded_hal::blocking::delay::DelayMs;

#[atxmega_hal::entry]
fn main() -> ! {
    let dp = atxmega_hal::Peripherals::take().unwrap();

    let mut led = dp.pins.pe2.into_output();

    let mut delay = atxmega_hal::delay::Delay::<atxmega_hal::clock::MHz16>::new();

    loop {
        led.toggle();
        delay.delay_ms(100u16);
    }
}
