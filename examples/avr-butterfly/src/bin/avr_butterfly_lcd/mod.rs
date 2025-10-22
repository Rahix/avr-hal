//! `LCD` driver for the ATmega169 LCD controller

use avr_device::atmega169pa::{LCD as Mega169_LCD, TC2};

macro_rules! seg {
    ( $($s:ident),* ) => {{
        0u16 $(| seg_bit!($s))*
    }};
}

macro_rules! seg_bit {
    (A) => {
        1 << 0
    };
    (S) => {
        1 << 1
    };
    (X) => {
        1 << 2
    };
    (K) => {
        1 << 3
    };
    (B) => {
        1 << 4
    };
    (H) => {
        1 << 5
    };
    (F) => {
        1 << 6
    };
    (J) => {
        1 << 7
    };
    (C) => {
        1 << 8
    };
    (G) => {
        1 << 9
    };
    (E) => {
        1 << 10
    };
    (L) => {
        1 << 11
    };
    (D) => {
        1 << 12
    };
    (N) => {
        1 << 13
    };
    (P) => {
        1 << 14
    };
    (M) => {
        1 << 15
    };
}

/// Mapping of LCD segments to supported ascii chars
///
/// The supported ascii chars start with space (32) and end with ~ (126).  
/// All other char codes from u8 range become replaced with an _ (95).
///
/// The segment arrangement is
///
/// <pre>
///   AAAAAAAAA
/// F H   J   K B
/// F  H  J  K  B
/// F   H J K   B
/// F    HJK    B
///   GGGG LLLL
/// E    PNM    C
/// E   P N M   C
/// E  P  N  M  C
/// E P   N   M C
///   DDDDDDDDD
/// </pre>
///
/// The LCD display on AVR butterfly board supports some additional fixed small symbols:  
/// - numbers 1, 2, 4, 5, 9, 10
/// - markers S1, S2, S3, S4, S9, S10
///
/// See [`SpecialSeg`].
///
/// # See also
///
/// [STK502 User Guide](https://usermanual.wiki/Atmel/AtmelStk502UsersManual500190.957703282.pdf)
/// and AVR App Note
/// [AVR065: LCD Driver for the STK502](https://ww1.microchip.com/downloads/aemDocuments/documents/OTH/ApplicationNotes/ApplicationNotes/doc2530.pdf)
///
pub const LCD_MAP: [u16; 95] = [
    0,                                  // space
    seg!(D, J, N),                      // !
    seg!(B, J),                         // "
    seg!(B, C, D, G, J, L, N),          // #
    seg!(A, C, D, F, G, L, J, N),       // $
    seg!(C, F, G, H, J, K, L, M, N, P), // %
    seg!(A, D, E, G, H, J, M),          // &
    seg!(J),                            // '
    seg!(K, M),                         // (
    seg!(H, P),                         // )
    seg!(G, H, J, K, L, M, N, P),       // *
    seg!(G, J, L, N),                   // +
    seg!(P),                            // ,
    seg!(G, L),                         // -
    seg!(P),                            // .
    seg!(K, P),                         // /
    //
    // digits
    seg!(A, B, C, D, E, F, K, P), // 0
    seg!(B, C, K),                // 1
    seg!(A, B, D, E, G, L),       // 2
    seg!(A, B, C, D, G, L),       // 3
    seg!(B, C, F, G, L),          // 4
    seg!(A, D, F, G, M),          // 5
    seg!(A, C, D, E, F, G, L),    // 6
    seg!(A, B, C, F),             // 7
    seg!(A, B, C, D, E, F, G, L), // 8
    seg!(A, B, C, D, F, G, L),    // 9
    //
    //
    seg!(G, P),                // :
    seg!(J, P),                // ;
    seg!(G, K, M),             // <
    seg!(G, L, D),             // =
    seg!(H, L, P),             // >
    seg!(A, B, D, L, N),       // ?
    seg!(A, B, D, E, F, J, L), // @
    //
    // uppercase letters A–Z
    seg!(A, B, C, E, F, G, L), // A
    seg!(A, B, C, D, J, L, N), // B
    seg!(A, D, E, F),          // C
    seg!(A, B, C, D, J, N),    // D
    seg!(A, D, E, F, G),       // E
    seg!(A, E, F, G),          // F
    seg!(A, C, D, E, F, L),    // G
    seg!(B, C, E, F, G, L),    // H
    seg!(A, D, J, N),          // I
    seg!(B, C, D, E),          // J
    seg!(E, F, G, K, M),       // K
    seg!(D, E, F),             // L
    seg!(B, C, E, F, H, K),    // M
    seg!(B, C, E, F, H, M),    // N
    seg!(A, B, C, D, E, F),    // O
    seg!(A, B, E, F, G, L),    // P
    seg!(A, B, C, D, E, F, M), // Q
    seg!(A, B, E, F, G, L, M), // R
    seg!(A, C, D, F, G, L),    // S
    seg!(A, J, N),             // T
    seg!(B, C, D, E, F),       // U
    seg!(E, F, K, P),          // V
    seg!(B, C, E, F, P, M),    // W
    seg!(H, K, M, P),          // X
    seg!(B, C, D, F, G, L),    // Y
    seg!(A, D, K, P),          // Z
    //
    //
    seg!(A, D, E, F), // [
    seg!(H, M),       // \
    seg!(A, B, C, D), // ]
    seg!(M, P),       // ^
    seg!(D),          // _
    seg!(H),          // `
    //
    // lowercase letters a–z
    seg!(D, E, G, N),    // a
    seg!(D, E, F, G, M), // b
    seg!(D, E, G, L),    // c
    seg!(B, C, D, L, P), // d
    seg!(D, E, G, P),    // e
    seg!(G, K, L, N),    // f
    seg!(B, C, D, K, L), // g
    seg!(E, F, G, N),    // h
    seg!(N),             // i
    seg!(B, C, D, P),    // j
    seg!(J, K, M, N),    // k
    seg!(E, F),          // l
    seg!(C, E, G, L, N), // m
    seg!(E, G, N),       // n
    seg!(C, D, E, G, L), // o
    seg!(A, E, F, G, K), // p
    seg!(A, B, C, H, L), // q
    seg!(E, G),          // r
    seg!(A, C, D, H, L), // s
    seg!(D, E, F, G),    // t
    seg!(C, D, E),       // u
    seg!(E, P),          // v
    seg!(C, E, M, P),    // w
    seg!(H, K, M, P),    // x
    seg!(B, C, D, H, L), // y
    seg!(D, G, P),       // z
    //
    //
    seg!(A, D, G, H, P), // {
    seg!(J, N),          // |
    seg!(A, D, K, L, M), // }
    seg!(G, K, L, P),    // ~
];

/// The LCD display on AVR butterfly board supports some additional fixed small symbols:  
/// - numbers 1, 2, 4, 5, 9, 10
/// - markers S1, S2, S3, S4, S9, S10
///
/// At least it is documented like this, but my physical display has additional bars above the two
/// right digits, and overall the mapping of these symbols is not working as expected.  
/// See the enum fields for what might be happening.
#[derive(Clone, Copy)]
pub enum SpecialSeg {
    N1,
    /// board enables here 2 and 9
    N2,
    /// board enables 4 and a bar above 2nd right most digit
    N4,
    N5,
    /// board enables 9 and S10
    N9,
    N10,
    S1,
    /// board enables S2 and S9
    S2,
    S3,
    S4,
    /// board enables S9 and a bar above the right most
    S9,
    S10,
}

static mut LCD_USED: bool = false;

static mut LCD: Lcd = Lcd {
    lcd: None,
    lcd_buf: LCDBuf {
        digits: [0u8; 6],
        n_digits: 0u8,
        s_digits: 0u8,
    },
    sync: true,
    sync_enable: true,
};

/// `LCD` driver for the ATmega169 LCD controller
pub struct Lcd {
    lcd: Option<Mega169_LCD>,
    lcd_buf: LCDBuf,
    sync: bool,
    sync_enable: bool,
}

impl Lcd {
    /// Enable and disable LCD
    pub fn lcd_enable(&mut self, enable: bool) {
        if let Some(ref lcd) = self.lcd {
            lcd.lcdcra.write(|w| {
                if enable {
                    w.lcden() // LCD enable
                        .set_bit()
                        .lcdie() // LCD SOF interrupt enable, global interrupts need to be enabled external
                        .set_bit()
                } else {
                    w.lcden() // LCD disable
                        .clear_bit()
                        .lcdie() // LCD SOF interrupt disable
                        .clear_bit()
                }
            });
        }
    }

    /// `true` when buffer has been used in irq and LCD is in sync
    pub fn lcd_is_sync(&self) -> bool {
        self.sync
    }

    /// Init LCD for usage
    ///
    /// Global interrupts need to be enabled for updating the LCD
    /// ```rust
    /// unsafe { avr_device::interrupt::enable(); }
    /// ```
    ///
    /// When providing the optional `tc2` async timer clock, it used with on-board 32khz crystal
    /// for efficient LCD frame timing.
    ///
    /// Without `tc2` the LCD is timed by sys clock.
    ///
    pub fn lcd_init(lcd: Mega169_LCD, tc2: Option<TC2>) -> Option<&'static mut Self> {
        if avr_device::interrupt::free(|_| {
            if unsafe { LCD_USED } {
                true
            } else {
                unsafe {
                    LCD_USED = true;
                }
                false
            }
        }) {
            return None;
        }

        if let Some(tc2) = tc2 {
            tc2.assr.write(|w| w.as2().set_bit());
            lcd.lcdcrb.write(|w| {
                w.lcdmux()
                    .bits(0b11) // 1/4 duty
                    .lcd2b()
                    .clear_bit() // 1/3 bias
                    .lcdpm()
                    .bits(0b111) // all LCD port pins
                    .lcdcs()
                    .set_bit() // clock souce TOSC1 pin (32khz osci)
            });
            lcd.lcdfrr.write(|w| w.lcdps().clklcd_16().lcdcd()._3()); // prescaler/divider => 16 / 4 => =64Hz
        } else {
            lcd.lcdcrb.write(|w| {
                w.lcdmux()
                    .bits(0b11) // 1/4 duty
                    .lcd2b()
                    .clear_bit() // 1/3 bias
                    .lcdpm()
                    .bits(0b111) // all LCD port pins        
            });
            lcd.lcdfrr.write(|w| w.lcdps().clklcd_4096().lcdcd()._3()); // prescaler/divider => 4096 / 4 => ~64Hz
        }

        lcd.lcdccr.write(|w| w.lcdcc()._3_00v()); // recommended contrast

        lcd.lcdcra.write(|w| {
            w.lcden() // LCD enable
                .set_bit()
                .lcdie() // LCD SOF interrupt enable, global interrupts need to be enabled external
                .set_bit()
        });

        unsafe {
            LCD.lcd = Some(lcd);
        }

        let p = &raw mut LCD;
        unsafe { p.as_mut() }
    }

    /// Enable and disable a [`SpecialSeg`]
    pub fn lcd_special_seg(&mut self, seg: SpecialSeg, enable: bool) {
        let (digits, bit) = match seg {
            SpecialSeg::N1 => (&mut self.lcd_buf.n_digits, 0u8),
            SpecialSeg::N2 => (&mut self.lcd_buf.n_digits, 1),
            SpecialSeg::N4 => (&mut self.lcd_buf.n_digits, 2),
            SpecialSeg::N5 => (&mut self.lcd_buf.n_digits, 3),
            SpecialSeg::N9 => (&mut self.lcd_buf.n_digits, 4),
            SpecialSeg::N10 => (&mut self.lcd_buf.n_digits, 5),
            SpecialSeg::S1 => (&mut self.lcd_buf.s_digits, 0),
            SpecialSeg::S2 => (&mut self.lcd_buf.s_digits, 1),
            SpecialSeg::S3 => (&mut self.lcd_buf.s_digits, 2),
            SpecialSeg::S4 => (&mut self.lcd_buf.s_digits, 3),
            SpecialSeg::S9 => (&mut self.lcd_buf.s_digits, 4),
            SpecialSeg::S10 => (&mut self.lcd_buf.s_digits, 5),
        };

        Self::update_special(&mut self.sync, digits, bit, enable);
    }

    /// Enable and disable refresh sync the LCD from buffer
    pub fn lcd_sync_enable(&mut self, enable: bool) {
        self.sync_enable = enable;
    }

    /// Write supported ascii char `char_val` to LCD digit `digit_nr`
    ///
    /// This actual writes to buffer and LCD is synced in interrupt handler
    ///
    pub fn lcd_write(&mut self, char_val: u8, digit_nr: u8) {
        if self.lcd_buf.digits[digit_nr as usize] != char_val {
            self.lcd_buf.digits[digit_nr as usize] = char_val;
            self.sync = false;
        }
    }

    /// Set LCD register to modify display
    ///
    /// `digit_nr` is the number of digit starting from left with `0`.  
    /// The AVR Butterfly doesn't support the 1st digit on LCD display,
    /// so `digit_nr` is the 2nd physical segment area.
    ///
    /// `digit` is the segment control code for the requested signs.
    ///
    /// `n_digits` is a bitfield for special number segments
    ///
    /// `s_digits` is a bitfield for special s segments
    ///
    fn set_lcd_digit(lcd: &Mega169_LCD, digit_nr: u8, digit: u16, n_digits: u8, s_digits: u8) {
        match digit_nr {
            0 => {
                let mut digit_odd = lcd.lcddr0.read().bits() & 0xF0;
                let special = ((n_digits & 0x1) << 2) | ((s_digits & 0x1) << 1);
                lcd.lcddr0
                    .write(|w| w.bits((digit & 0xF) as u8 | special | digit_odd));
                digit_odd = lcd.lcddr5.read().bits() & 0xF0;
                lcd.lcddr5
                    .write(|w| w.bits((digit >> 4 & 0xF) as u8 | digit_odd));
                digit_odd = lcd.lcddr10.read().bits() & 0xF0;
                lcd.lcddr10
                    .write(|w| w.bits((digit >> 8 & 0xF) as u8 | digit_odd));
                digit_odd = lcd.lcddr15.read().bits() & 0xF0;
                lcd.lcddr15
                    .write(|w| w.bits((digit >> 12 & 0xF) as u8 | digit_odd));
            }
            1 => {
                let mut digit_even = lcd.lcddr0.read().bits() & 0xF;
                let special = ((n_digits & 0x2) << 5) | ((s_digits & 0x2) << 4);
                lcd.lcddr0
                    .write(|w| w.bits(((digit & 0xF) << 4) as u8 | special | digit_even));
                digit_even = lcd.lcddr5.read().bits() & 0xF;
                lcd.lcddr5
                    .write(|w| w.bits((digit & 0xF0) as u8 | digit_even));
                digit_even = lcd.lcddr10.read().bits() & 0xF;
                lcd.lcddr10
                    .write(|w| w.bits(((digit & 0xF00) >> 4) as u8 | digit_even));
                digit_even = lcd.lcddr15.read().bits() & 0xF;
                lcd.lcddr15
                    .write(|w| w.bits(((digit & 0xF000) >> 8) as u8 | digit_even));
            }

            2 => {
                let mut digit_odd = lcd.lcddr1.read().bits() & 0xF0;
                let special = ((n_digits & 0x4) >> 1) | (s_digits & 0x4);
                lcd.lcddr1
                    .write(|w| w.bits((digit & 0xF) as u8 | special | digit_odd));
                digit_odd = lcd.lcddr6.read().bits() & 0xF0;
                lcd.lcddr6
                    .write(|w| w.bits((digit >> 4 & 0xF) as u8 | digit_odd));
                digit_odd = lcd.lcddr11.read().bits() & 0xF0;
                lcd.lcddr11
                    .write(|w| w.bits((digit >> 8 & 0xF) as u8 | digit_odd));
                digit_odd = lcd.lcddr16.read().bits() & 0xF0;
                lcd.lcddr16
                    .write(|w| w.bits((digit >> 12 & 0xF) as u8 | digit_odd));
            }
            3 => {
                let mut digit_even = lcd.lcddr1.read().bits() & 0xF;
                let special = ((n_digits & 0x8) << 2) | ((s_digits & 0x8) << 3);
                lcd.lcddr1
                    .write(|w| w.bits(((digit & 0xF) << 4) as u8 | special | digit_even));
                digit_even = lcd.lcddr6.read().bits() & 0xF;
                lcd.lcddr6
                    .write(|w| w.bits((digit & 0xF0) as u8 | digit_even));
                digit_even = lcd.lcddr11.read().bits() & 0xF;
                lcd.lcddr11
                    .write(|w| w.bits(((digit & 0xF00) >> 4) as u8 | digit_even));

                digit_even = lcd.lcddr16.read().bits() & 0xF;
                lcd.lcddr16
                    .write(|w| w.bits(((digit & 0xF000) >> 8) as u8 | digit_even));
            }

            4 => {
                let mut digit_odd = lcd.lcddr2.read().bits() & 0xF0;
                let special = ((n_digits & 0x16) >> 2) | ((s_digits & 0x16) >> 3);
                lcd.lcddr2
                    .write(|w| w.bits((digit & 0xF) as u8 | special | digit_odd));
                digit_odd = lcd.lcddr7.read().bits() & 0xF0;
                lcd.lcddr7
                    .write(|w| w.bits((digit >> 4 & 0xF) as u8 | digit_odd));

                digit_odd = lcd.lcddr12.read().bits() & 0xF0;
                lcd.lcddr12
                    .write(|w| w.bits((digit >> 8 & 0xF) as u8 | digit_odd));

                digit_odd = lcd.lcddr17.read().bits() & 0xF0;
                lcd.lcddr17
                    .write(|w| w.bits((digit >> 12 & 0xF) as u8 | digit_odd));
            }
            5 => {
                let mut digit_even = lcd.lcddr2.read().bits() & 0xF;
                let special = ((n_digits & 0x32) << 1) | (s_digits & 0x32);
                lcd.lcddr2
                    .write(|w| w.bits(((digit & 0xF) << 4) as u8 | special | digit_even));
                digit_even = lcd.lcddr7.read().bits() & 0xF;
                lcd.lcddr7
                    .write(|w| w.bits((digit & 0xF0) as u8 | digit_even));
                digit_even = lcd.lcddr12.read().bits() & 0xF;
                lcd.lcddr12
                    .write(|w| w.bits(((digit & 0xF00) >> 4) as u8 | digit_even));
                digit_even = lcd.lcddr17.read().bits() & 0xF;
                lcd.lcddr17
                    .write(|w| w.bits(((digit & 0xF000) >> 8) as u8 | digit_even));
            }
            _ => assert!(digit_nr < 6),
        }
    }

    /// Set digit `digit_nr` to an ascii representation of ascii `char_val`
    ///
    /// Unsupported values are replaced with `_`.
    ///
    fn set_lcd_digit_char(
        lcd: &Mega169_LCD,
        digit_nr: u8,
        char_val: u8,
        n_digits: u8,
        s_digits: u8,
    ) {
        let char_val = if (32..=126).contains(&char_val) {
            char_val
        } else if char_val == 0 {
            b' '
        } else {
            b'_'
        };

        let digit = LCD_MAP[char_val as usize - 32];

        Self::set_lcd_digit(lcd, digit_nr, digit, n_digits, s_digits);
    }

    /// Set corresponding bit in `digits` bitfield
    fn update_special(sync: &mut bool, digits: &mut u8, bit: u8, enable: bool) {
        if *digits & (1u8 << bit) != enable as u8 {
            *sync = false;
            if enable {
                *digits |= 1u8 << bit;
            } else {
                *digits &= !(1u8 << bit);
            }
        }
    }
}

struct LCDBuf {
    digits: [u8; 6],
    n_digits: u8,
    s_digits: u8,
}

/// Updates the `LCDDRn` registers from changed buffered data and when `sync_enable` is true
#[avr_device::interrupt(atmega169pa)]
fn LCD() {
    if unsafe { LCD.sync || !LCD.sync_enable } {
        return;
    }
    let lcd = unsafe { (&raw const LCD.lcd) };
    if let Some(lcd) = unsafe { lcd.as_ref() }.unwrap() {
        let buf = unsafe { (&raw const LCD.lcd_buf).as_ref() }.unwrap();
        for (digit_nr, &char_val) in buf.digits.iter().enumerate() {
            Lcd::set_lcd_digit_char(lcd, digit_nr as u8, char_val, buf.n_digits, buf.s_digits);
        }
        unsafe {
            LCD.sync = true;
        }
    }
}
