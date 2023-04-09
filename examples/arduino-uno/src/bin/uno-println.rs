/*!
 * This is an example of how one can define a println!() macro which can be called anywhere to
 * write data to the console.
 *
 * Keep in mind that this will enter a critical section while printing so no interrupts can be
 * served in the meantime.
 */
#![no_std]
#![no_main]

use panic_halt as _;

use avr_device::interrupt;
use core::cell::RefCell;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

macro_rules! print {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

macro_rules! println {
    ($($t:tt)*) => {
        interrupt::free(
            |cs| {
                if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}

fn put_console(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}

fn subfunction() {
    println!("We can also call println!() in a subfunction!");
}

fn demo_print_without_ln() {
    for i in 0..10 {
        print!("{} ", i);
    }
    println!("numbers!");
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    put_console(serial);

    println!("Hello from main!");
    subfunction();
    demo_print_without_ln();

    loop {}
}
