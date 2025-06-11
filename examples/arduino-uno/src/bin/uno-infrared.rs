/*!
 * Example for receiving IR signals from a remote control using an [Irdroino] shield
 * and the [Infrared] crate.
 *
 * [Infrared]: https://github.com/jkristell/infrared
 * [Irdroino]: https://github.com/irdroid/irdroino
 *
 * Connections
 * -----------
 *   - `d7`: Yellow led
 *   - `d6`: Blue led
 *   - `d5`: Rx Button
 *   - `d4`: Tx Button
 *   - `d3`: Infrared tx
 *   - `d2`: Infrared rx
 */

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::cell::Cell;
use panic_halt as _;

use arduino_hal::{
    hal::port::{PD2, PD6, PD7},
    pac::tc0::tccr0b::CS0_A,
    port::mode::{Floating, Input, Output},
    port::Pin,
    prelude::*,
};
use avr_device::interrupt::Mutex;

use infrared::{protocol::nec::NecCommand, protocol::*, Receiver};

type IrPin = Pin<Input<Floating>, PD2>;
type IrProto = Nec;
type IrCmd = NecCommand;

static CLOCK: Clock = Clock::new();
static mut RECEIVER: Option<Receiver<IrProto, IrPin>> = None;
static mut LED: Option<Pin<Output, PD7>> = None;
static mut ERROR_LED: Option<Pin<Output, PD6>> = None;

static CMD: Mutex<Cell<Option<IrCmd>>> = Mutex::new(Cell::new(None));

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Monotonic clock to keep track of the time
    CLOCK.start(dp.TC0);

    let mut uno_led = pins.d13.into_output();
    let mut irdroino_led1 = pins.d7.into_output();
    let mut irdroino_led2 = pins.d6.into_output();

    uno_led.set_low();
    irdroino_led1.set_low();
    irdroino_led2.set_low();

    // Enable group 2 (PORTD)
    dp.EXINT.pcicr().write(|w| unsafe { w.bits(0b100) });

    // Enable pin change interrupts on PCINT18 which is pin PD2 (= d2)
    dp.EXINT.pcmsk2().write(|w| w.set(0b100));

    let ir = Receiver::with_pin(Clock::FREQ, pins.d2);

    unsafe {
        RECEIVER.replace(ir);
        LED.replace(irdroino_led1);
        ERROR_LED.replace(irdroino_led2);
    }

    // Enable interrupts globally
    unsafe { avr_device::interrupt::enable() };

    ufmt::uwriteln!(&mut serial, "Hello from Arduino and Irdroino!\r").unwrap_infallible();

    loop {
        if let Some(cmd) = avr_device::interrupt::free(|cs| CMD.borrow(cs).take()) {
            ufmt::uwriteln!(
                &mut serial,
                "Cmd: Adress: {}, Command: {}, repeat: {}\r",
                cmd.addr,
                cmd.cmd,
                cmd.repeat
            )
            .unwrap_infallible();
        }

        arduino_hal::delay_ms(100);
    }
}

#[avr_device::interrupt(atmega328p)]
fn PCINT2() {
    let recv = unsafe { RECEIVER.as_mut().unwrap() };
    let led = unsafe { LED.as_mut().unwrap() };
    let error_led = unsafe { ERROR_LED.as_mut().unwrap() };

    let now = CLOCK.now();

    match recv.event_instant(now) {
        Ok(Some(cmd)) => {
            avr_device::interrupt::free(|cs| {
                let cell = CMD.borrow(cs);
                cell.set(Some(cmd));
            });
            error_led.set_low();
        }
        Ok(None) => (),
        Err(_) => error_led.set_high(),
    }

    led.toggle();
}

#[avr_device::interrupt(atmega328p)]
fn TIMER0_COMPA() {
    CLOCK.tick();
}

struct Clock {
    cntr: Mutex<Cell<u32>>,
}

impl Clock {
    const FREQ: u32 = 20_000;
    const PRESCALER: CS0_A = CS0_A::PRESCALE_8;
    const TOP: u8 = 99;

    pub const fn new() -> Clock {
        Clock {
            cntr: Mutex::new(Cell::new(0)),
        }
    }

    pub fn start(&self, tc0: arduino_hal::pac::TC0) {
        // Configure the timer for the above interval (in CTC mode)
        tc0.tccr0a().write(|w| w.wgm0().ctc());
        tc0.ocr0a().write(|w| w.set(Self::TOP));
        tc0.tccr0b().write(|w| w.cs0().variant(Self::PRESCALER));

        // Enable interrupt
        tc0.timsk0().write(|w| w.ocie0a().set_bit());
    }

    pub fn now(&self) -> u32 {
        avr_device::interrupt::free(|cs| self.cntr.borrow(cs).get())
    }

    pub fn tick(&self) {
        avr_device::interrupt::free(|cs| {
            let c = self.cntr.borrow(cs);

            let v = c.get();
            c.set(v.wrapping_add(1));
        });
    }
}
