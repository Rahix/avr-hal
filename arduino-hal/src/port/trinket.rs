pub use attiny_hal::port::mode;
pub use attiny_hal::port::Pin;

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    pub struct Pins from attiny_hal::Pins {
     /// `#0`: `PB0`, `DI`(SPI), `SDA`(I2C)
     pub d0: portb::pb0::PB0 = pb0,
     /// `#1`: `PB1`, `DO`(SPI), Builtin LED
     pub d1: portb::pb1::PB1 = pb1,
     /// `#2`: `PB2`, `SCK`(SPI), `SCL`(I2C)
     pub d2: portb::pb2::PB2 = pb2,
     /// `#3`: `PB3`
     pub d3: portb::pb3::PB3 = pb3,
     /// `#4`: `PB4`
     pub d4: portb::pb4::PB4 = pb4,
    }
}
