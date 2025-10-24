#![no_std]
#![no_main]

use atmega_hal::delay::Delay;
use atmega_hal::usart::{Baudrate, Usart};
use embedded_hal::delay::DelayNs;
use panic_halt as _;

// Define core clock in the root crate
type CoreClock = atmega_hal::clock::MHz8;
// Use it as follows in the rest of the project
type Adc = atmega_hal::adc::Adc<crate::CoreClock>;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();

    // double set in 4 cycles to disable JTAG and make shared ADC pins usable
    dp.JTAG.mcucr.write(|w| w.jtd().set_bit());
    dp.JTAG.mcucr.write(|w| w.jtd().set_bit());

    let pins = atmega_hal::pins!(dp);

    let mut delay = Delay::<crate::CoreClock>::new();

    // set up serial interface for text output
    let mut serial = Usart::new(
        dp.USART0,
        pins.pe0,
        pins.pe1.into_output(),
        Baudrate::<crate::CoreClock>::new(57600),
    );

    let mut adc = Adc::new(dp.ADC, Default::default());

    // To store multiple channels in an array, we use the `into_channel()` method.
    let channels: [atmega_hal::adc::Channel; 5] = [
        pins.pf0.into_analog_input(&mut adc).into_channel(),
        pins.pf1.into_analog_input(&mut adc).into_channel(),
        pins.pf2.into_analog_input(&mut adc).into_channel(),
        pins.pf3.into_analog_input(&mut adc).into_channel(),
        pins.pf4.into_analog_input(&mut adc).into_channel(),
    ];

    loop {
        for (i, ch) in channels.iter().enumerate() {
            let v = adc.read_blocking(ch);
            ufmt::uwrite!(&mut serial, "PF{}: {} ", i, v).unwrap();
        }

        ufmt::uwriteln!(&mut serial, "\r").unwrap();
        delay.delay_ms(1000);
    }
}
