pub use attiny_hal::port::{mode, Pin, PinOps, PinMode};

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    pub struct Pins from attiny_hal::Pins {
     /// `#0`: `PB0`, `DI`(SPI), `SDA`(I2C)
     pub d0: attiny_hal::port::PB0 = pb0,
     /// `#1`: `PB1`, `DO`(SPI), Builtin LED
     pub d1: attiny_hal::port::PB1 = pb1,
     /// `#2`: `PB2`, `SCK`(SPI), `SCL`(I2C)
     pub d2: attiny_hal::port::PB2 = pb2,
     /// `#3`: `PB3`
     pub d3: attiny_hal::port::PB3 = pb3,
     /// `#4`: `PB4`
     pub d4: attiny_hal::port::PB4 = pb4,
     /// `#5`: `PB5`
     pub d5: attiny_hal::port::PB5 = pb5,
    }
}
