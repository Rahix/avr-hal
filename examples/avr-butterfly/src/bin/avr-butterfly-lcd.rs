//! LCD test code for AVR Butterfly
//!
//! Uses all available segments and displays all available and supportable ascii chars on
//! all LCD digits.
//!

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_hal::delay::Delay;
use embedded_hal::delay::DelayNs;
use panic_halt as _;

use avr_butterfly_lcd::{LCD_MAP, Lcd, SpecialSeg};

mod avr_butterfly_lcd;

// Define core clock. This can be used in the rest of the project.
type CoreClock = atmega_hal::clock::MHz8;

const DIGITS_AVRHAL: &[u8; 6] = b"AVRHAL";

#[avr_device::entry]
fn main() -> ! {
    let mut delay = Delay::<crate::CoreClock>::new();

    let dp = atmega_hal::Peripherals::take().unwrap();

    // let lcd = Lcd::lcd_init(dp.LCD, None).unwrap();
    let lcd = Lcd::lcd_init(dp.LCD, Some(dp.TC2)).unwrap();

    unsafe {
        avr_device::interrupt::enable(); // LCD uses start of frame interrupt
    }

    for n in 0..6 {
        for m in 0..n {
            lcd.lcd_write(b'-', m);
        }
        lcd.lcd_write(b'>', n);
        delay.delay_ms(500);
    }

    for (d, n) in (0..5).rev().enumerate() {
        for m in 0..n {
            lcd.lcd_write(b'-', m);
        }
        lcd.lcd_write(b'>', n);

        for i in 0..=(d as u8) {
            lcd.lcd_write(DIGITS_AVRHAL[i as usize], i + n + 1);
        }

        delay.delay_ms(500);
    }

    lcd.lcd_sync_enable(false);
    for (digit_nr, &char_val) in DIGITS_AVRHAL.iter().enumerate() {
        lcd.lcd_write(char_val, digit_nr as u8);
    }
    lcd.lcd_sync_enable(true);
    delay.delay_ms(1000);

    lcd.lcd_sync_enable(false);
    for n in 0..6 {
        lcd.lcd_write(32, n);
    }
    lcd.lcd_sync_enable(true);

    let seg_list = [
        SpecialSeg::N1,
        SpecialSeg::N2,
        SpecialSeg::N4,
        SpecialSeg::N5,
        SpecialSeg::N9,
        SpecialSeg::N10,
        SpecialSeg::S1,
        SpecialSeg::S2,
        SpecialSeg::S3,
        SpecialSeg::S4,
        SpecialSeg::S9,
        SpecialSeg::S10,
    ];

    for seg in seg_list {
        while !lcd.lcd_is_sync() {
            delay.delay_ms(100);
        }
        lcd.lcd_special_seg(seg, true);
        delay.delay_ms(1000);
    }
    delay.delay_ms(1000);

    for seg in seg_list {
        while !lcd.lcd_is_sync() {
            delay.delay_ms(100);
        }
        lcd.lcd_special_seg(seg, false);
        delay.delay_ms(500);
    }

    while !lcd.lcd_is_sync() {
        delay.delay_ms(100);
    }

    loop {
        for char_val in 32..(LCD_MAP.len() + 32) {
            let char_val = char_val as u8;
            for digit_nr in 0..6 {
                lcd.lcd_write(char_val, digit_nr);
                delay.delay_ms(500);
            }
        }

        lcd.lcd_sync_enable(false);
        for (digit_nr, &char_val) in DIGITS_AVRHAL.iter().enumerate() {
            lcd.lcd_write(char_val, digit_nr as u8);
        }
        lcd.lcd_sync_enable(true);
        delay.delay_ms(3000);
        lcd.lcd_enable(false);
        delay.delay_ms(2000);
        lcd.lcd_enable(true);
    }
}
