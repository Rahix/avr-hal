use embedded_hal::blocking::delay::{DelayMs, DelayUs};

pub type Delay = avr_hal_generic::delay::Delay<crate::DefaultClock>;

pub fn delay_ms(ms: u16) {
    Delay::new().delay_ms(ms)
}

pub fn delay_us(us: u32) {
    Delay::new().delay_us(us)
}
