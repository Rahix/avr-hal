#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

///
/// We use the 16Mhz oscillator (fuse setting) for the attiny.
/// Startup divider is 6, init_clock() changes it to 2, resulting in a 8Mhz system clock.
///
pub fn init_clock(dp: &arduino_hal::Peripherals) {
    dp.CPU.ccp.write(|w| w.ccp().ioreg()); // remove protection
    dp.CLKCTRL
        .mclkctrlb
        .write(|w| w.pen().set_bit().pdiv()._2x()); // change frequency divider from 6 to 2, so we get 16/2 = 8 Mhz
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    init_clock(&dp);

    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 115200);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap_infallible();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).unwrap_infallible();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap_infallible();
    }
}
