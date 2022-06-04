/*!
 * Demonstration of using the time-keeping facilities. (Based on `uno-serial`)
 */
#![no_std]
#![no_main]
//
// Needed for the timer interrupt that is attached via `impl_timepiece`
#![feature(abi_avr_interrupt)]

use arduino_hal::impl_timepiece;
use arduino_hal::prelude::*;
use arduino_hal::time::embedded_time::duration::Microseconds;
use arduino_hal::time::embedded_time::duration::Seconds;
use arduino_hal::time::embedded_time::fixed_point::FixedPoint;
use arduino_hal::time::Chronometer;
use core::convert::TryFrom;
use panic_halt as _;

use embedded_hal::serial::Read;

// Prepare `Timer0` to be used for time-keeping.
// This will define the configuration and attach its timer interrupt.
impl_timepiece! {
    pub timepiece Foo {
        peripheral: Timer0,
        cpu_clock: arduino_hal::DefaultClock,
        millis: u32,
        micros: u32,
        resolution: arduino_hal::time::Resolution::MS_4,
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Take the timer peripheral (TC0 for Timer0) and wrap it in our `Foo` warpper
    let timepiece = Foo::new(dp.TC0);
    // Initialize the time-keeping facility
    let clock = Chronometer::new(timepiece);

    // Since the Chronometer relies on interrupts, we have to enable them
    // globally:
    unsafe {
        // SAFETY: This is not within `interrupt::free`
        avr_device::interrupt::enable();
    }

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();

        // Using the local `Chronometer` instance, it has up to microseconds
        // precision
        let time = clock.now().duration_since_epoch();
        let us = Microseconds::<u32>::try_from(time).unwrap();
        let us = us.integer(); // extract the integer, since `Microseconds` is not `uDisplay`

        // Alternatively, you can use the static clock (it is statically
        // accessible), which has just milliseconds precision with whatever
        // resolution you configured your timepiece (i.e. `Foo`)
        let time = Foo::CLOCK.now().duration_since_epoch();
        let seconds = Seconds::<u32>::try_from(time).unwrap();
        // extract the integer, since `Seconds` is not `uDisplay`
        let seconds = seconds.integer();

        ufmt::uwriteln!(
            &mut serial,
            "It is now {} s since boot up! ({} us)\r",
            seconds,
            us
        )
        .void_unwrap();
    }
}
