#![no_std]
#![no_main]

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

    // Digital pin A7 is a LED
    let mut led = pins.a7.into_output();

    loop {
        led.set_high();
        arduino_hal::delay_ms(10);
        led.set_low();
        arduino_hal::delay_ms(990);
    }
}
