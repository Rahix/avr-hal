/*!
  # A led blink application

  This is a simple circuit with a blinking led.

  # The hardware

    * a led
    * a 330 Ohm resistor
    * some patch cables/connections
    * The ports on the board (GND (JTAG/J402 Pin 2), PF4 (JTAG/J402 Pin 1), 3.3V (JTAG/J402 Pin 4))

  # The board layout

  Connect the resistor to Pin 1 of J402 socket. Connect led anode to other end of resistor, led cathode to Pin 2 of J402.
  Connect your 3.3V power source to Pin 4 (+) and Pin 2 (-) of J402 socket.
  So yes, led cathode and negative part of powersource connects both at Pin 2 of J402, maybe use a breadboard.
*/
#![no_std]
#![no_main]

use atmega_hal::delay::Delay;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

// Define core clock. This can be used in the rest of the project.
type CoreClock = atmega_hal::clock::MHz8;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();

    // double set in 4 cycles to disable JTAG and make shared ADC pins usable
    dp.JTAG.mcucr.write(|w| w.jtd().set_bit());
    dp.JTAG.mcucr.write(|w| w.jtd().set_bit());

    let pins = atmega_hal::pins!(dp);

    let mut delay = Delay::<crate::CoreClock>::new();

    let mut led = pins.pf4.into_output();

    loop {
        led.toggle();
        delay.delay_ms(1000);
    }
}
