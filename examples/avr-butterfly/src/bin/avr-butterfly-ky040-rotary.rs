//! Rotary encoder KY040 test code for AVR Butterfly
//!
//! Modifies display counter with rotating encoder.  
//! Pressing button displays PRESS.
//!
//! KY040 needs to be connected with CLK to PB6, DT to PB7 and SW to PB4.  
//! VCC and GND corresponding to AVR Butterfly.
//!

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use atmega_hal::delay::Delay;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use avr_butterfly_lcd::Lcd;

#[allow(dead_code)]
mod avr_butterfly_lcd;

// Define core clock. This can be used in the rest of the project.
type CoreClock = atmega_hal::clock::MHz8;

const PRESS: &[u8] = b"PRESS";
const PRESS_OFFSET: usize = 6 - PRESS.len();

static CTRL_CHANGE: AtomicBool = AtomicBool::new(false);
static CTRL_PRESS: AtomicBool = AtomicBool::new(false);
static CTRL_VALUE: AtomicU8 = AtomicU8::new(127);

/// Convert u8 number to ascii digits
fn u8_to_ascii(mut n: u8, buf: &mut [u8; 3]) -> usize {
    if n < 10 {
        buf[0] = b'0' + n;
        return 1;
    }

    let mut i = 0;
    let mut tmp = [0u8; 3];
    while n > 0 {
        let digit = n % 10;
        tmp[i] = b'0' + digit;
        n /= 10;
        i += 1;
    }
    for j in 0..i {
        buf[j] = tmp[i - 1 - j];
    }

    i
}

/// Update LCD with current `CTRL_VALUE`
fn update_lcd(lcd: &mut Lcd) {
    let val = CTRL_VALUE.load(Ordering::Relaxed);
    let mut ascii_buf = [0u8; 3];
    let n = u8_to_ascii(val, &mut ascii_buf);
    let display_val = &ascii_buf[..n];
    lcd.lcd_sync_enable(false);
    // 1 for len of PRESS
    for i in 1..(6 - n) {
        lcd.lcd_write(0, i as u8);
    }
    for (i, &char_val) in display_val.iter().enumerate() {
        lcd.lcd_write(char_val, (6 - n + i) as u8);
    }
    lcd.lcd_sync_enable(true);
}

#[avr_device::entry]
fn main() -> ! {
    let mut delay = Delay::<crate::CoreClock>::new();

    let dp = atmega_hal::Peripherals::take().unwrap();

    let pins = atmega_hal::pins!(dp);
    let _clk = pins.pb6.into_pull_up_input();
    let _dt = pins.pb7.into_pull_up_input();
    let _sw = pins.pb4.into_pull_up_input();

    // enable irq for pb4 (pcint12), pb6 (pcint14), pb7 (pcint15)
    dp.EXINT.pcmsk1.write(|w| w.pcint().bits(0b1101_0000));
    // enable pcie0 for above pin irqs
    dp.EXINT.eimsk.write(|w| w.pcie1().set_bit());

    let lcd = Lcd::lcd_init(dp.LCD, Some(dp.TC2)).unwrap();
    unsafe {
        avr_device::interrupt::enable(); // LCD uses start of frame interrupt
    }

    update_lcd(lcd);

    loop {
        if CTRL_CHANGE.load(Ordering::Relaxed) {
            update_lcd(lcd);
            CTRL_CHANGE.store(false, Ordering::Relaxed);
        } else if CTRL_PRESS.load(Ordering::Relaxed) {
            lcd.lcd_sync_enable(false);
            for (i, &char_val) in PRESS.iter().enumerate() {
                lcd.lcd_write(char_val, (PRESS_OFFSET + i) as u8);
            }
            lcd.lcd_sync_enable(true);
        }

        delay.delay_ms(10); // should be ok for demo
    }
}

/// previous sample storage for `PCINT1()`
static mut SAMPLE_PREV: u8 = 0b11;

/// Irq handler for pb4,6,7, so button/rotate CW-CCW
#[avr_device::interrupt(atmega169pa)]
fn PCINT1() {
    // safety: in irq it is ok to use like this to check pin state
    let dp = unsafe { atmega_hal::Peripherals::steal() };
    let pb = dp.PORTB.pinb.read();

    if pb.pb4().bit_is_clear() {
        CTRL_PRESS.store(true, Ordering::Relaxed);
        return;
    } else if pb.pb4().bit_is_set() && CTRL_PRESS.load(Ordering::Relaxed) {
        CTRL_PRESS.store(false, Ordering::Relaxed);
        CTRL_CHANGE.store(true, Ordering::Relaxed);
        return;
    }

    let sample = ((pb.pb7().bit() as u8) << 1) | pb.pb6().bit() as u8;

    unsafe {
        // 1st part of bitfield is used to count rotary step parts
        // 2nd part, the 2 lsb is the previous sample
        // 3rd part is current sample
        #[allow(clippy::unusual_byte_groupings)]
        match (SAMPLE_PREV & 0b111_111_00, SAMPLE_PREV & 0b11, sample) {
            //
            // CW 11 -> 10 -> 00 -> 01 -> 11
            (0b1_00, 0b10, 0b00) => {
                SAMPLE_PREV |= 0b10_00;
            }
            (0b11_00, 0b00, 0b01) => {
                SAMPLE_PREV |= 0b100_00;
            }
            (0b111_00, 0b01, 0b11) => {
                let val = CTRL_VALUE.load(Ordering::Relaxed);
                if val < u8::MAX {
                    CTRL_VALUE.store(val + 1, Ordering::Relaxed);
                    CTRL_CHANGE.store(true, Ordering::Relaxed);
                }
            }
            // here CW order begins because of wildcard match
            (_, 0b11, 0b10) => SAMPLE_PREV = 0b1_11,
            //
            // CCW 11 -> 01 -> 00 -> 10 -> 11
            (0b1_000_00, 0b01, 0b00) => {
                SAMPLE_PREV |= 0b10_000_00;
            }
            (0b11_000_00, 0b00, 0b10) => {
                SAMPLE_PREV |= 0b100_000_00;
            }
            (0b111_000_00, 0b10, 0b11) => {
                let val = CTRL_VALUE.load(Ordering::Relaxed);
                if val > u8::MIN {
                    CTRL_VALUE.store(val - 1, Ordering::Relaxed);
                    CTRL_CHANGE.store(true, Ordering::Relaxed);
                }
            }
            // here CCW order begins because of wildcard match
            (_, 0b11, 0b01) => SAMPLE_PREV = 0b1_000_11,
            _ => {}
        }

        SAMPLE_PREV = (SAMPLE_PREV & !0b11) | sample;
    }
}
