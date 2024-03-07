/*!
 * Demonstration of writing to and reading from the serial console.
 */
#![no_std]
#![no_main]

use panic_halt as _;

use embedded_io::{Read, Write};

fn usart_handler(serial: &mut (impl Read + Write)) -> ! {
    serial.write_all("Hello from Arduino!\r\n".as_bytes()).unwrap();

    loop {
        let mut rx_buf: [u8; 16] = [0; 16];
        let len = serial.read(&mut rx_buf).unwrap();

        writeln!(serial, "Got {:?} (which is {} bytes long)", &rx_buf[..len], len).unwrap();
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    usart_handler(&mut serial);
}
