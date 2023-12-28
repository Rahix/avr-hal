#![no_std]
#![no_main]

use panic_halt as _;

use embedded_hal_v0::serial::Read;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").unwrap();

    loop {
        // Read a byte from the serial connection default
        let b = nb::block!(serial.read()).unwrap();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).unwrap();
    }
}
