#![no_std]
#![no_main]

use atmega_hal::Eeprom;
use atmega_hal::delay::Delay;
use atmega_hal::usart::{Baudrate, Usart};
use embedded_hal::delay::DelayNs;
use panic_halt as _;

// Define core clock in the root crate
type CoreClock = atmega_hal::clock::MHz8;

const BOOT_COUNT_OFFSET: u16 = 0;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut delay = Delay::<crate::CoreClock>::new();

    // set up serial interface for text output
    let mut serial = Usart::new(
        dp.USART0,
        pins.pe0,
        pins.pe1.into_output(),
        Baudrate::<crate::CoreClock>::new(57600),
    );

    let mut eeprom = Eeprom::new(dp.EEPROM);

    let eeprom_capacity = eeprom.capacity();
    ufmt::uwriteln!(&mut serial, "eeprom capacity is: {}", eeprom_capacity).unwrap();

    let mut boot_count = eeprom.read_byte(BOOT_COUNT_OFFSET);
    boot_count = boot_count.wrapping_add(1);
    eeprom.write_byte(BOOT_COUNT_OFFSET, boot_count);

    ufmt::uwriteln!(&mut serial, "\rBoot count: {}\r", boot_count).unwrap();

    loop {
        delay.delay_ms(1000);
    }
}
