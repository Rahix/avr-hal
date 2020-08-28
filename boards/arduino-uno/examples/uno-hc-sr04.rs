#![no_std]
#![no_main]

extern crate panic_halt;

use arduino_uno::prelude::*;


#[arduino_uno::entry]
fn main() -> ! {

    let dp = arduino_uno::Peripherals::take().unwrap();

    let mut delay = arduino_uno::Delay::new();

    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);

    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600,
    );

    let timer1 = dp.TC1;
    // Starting and initializing the timer with prescaling 64.
    // it gives one clock count every 4 µs.
    // since the clock register size is 16 bits, the timer is full every
    // 1/(16e6/64)*2^16 ≈ 260 ms
    timer1.tccr1b.write(|w| w.cs1().prescale_64());

    // Sensor used in this example: https://www.electroschematics.com/hc-sr04-datasheet/
    let mut trig = pins.d2.into_output(&mut pins.ddr);
    // Pins are floating input by default.
    let echo = pins.d3;

    'outer: loop {

        // the timer is reinitialized with value 0.
        timer1.tcnt1.write(|w| unsafe { w.bits(0) });

        // the trigger must be set to high under 10 µs as per the HC-SR04 datasheet
        trig.set_high().void_unwrap();
        delay.delay_us(10u16);
        trig.set_low().void_unwrap();

        while echo.is_low().void_unwrap() {
            // exiting the loop if the timer has reached 200 ms.
            // 0.2s/4µs = 50000
            if timer1.tcnt1.read().bits() >= 50000 {
                // jump to the beginning of the outer loop if no obstacle is detected
                ufmt::uwriteln!(&mut serial, "Nothing was detected and jump to outer loop.\r").void_unwrap();
                continue 'outer;
            }
        }
        // Restarting the timer
        timer1.tcnt1.write(|w| unsafe { w.bits(0) });

        // Wait for the echo to get low again
        while echo.is_high().void_unwrap() {}

        // 1 count == 4 us, so the value is multiplied by 4.
        // 1/58 ≈ (34000 ms/2)* 1µs
        let value = (timer1.tcnt1.read().bits() * 4) / 58;

        // Await 100 ms before sending the next trig
        // 0.1s/4µs = 50000
        while timer1.tcnt1.read().bits() < 25000 {}

        ufmt::uwriteln!(&mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();
    }
}
