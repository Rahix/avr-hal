pub use attiny_hal::port::{mode, Pin, PinMode, PinOps};

avr_hal_generic::renamed_pins! {
    pub struct Pins {
        /// `#0`: `PB0`, `0`, `PCINT0`, `D1`(SPI), `SDA`(I2C), `MOSI`, `AREF`, `OC0A`, `AIN0`
        pub d0: attiny_hal::port::PB0 = pb0,
        /// `#1`: `PB1`, `1`, `PCINT1`, `D0`(SPI), `SDA`(I2C), `MISO`, `AREF`, `OC0B`, `OC1A`, `AIN1`
        pub d1: attiny_hal::port::PB1 = pb1,
        /// `#2`: `PB2`, `2/A1`, `PCINT2` `SCK`(SPI), `SCL`(I2C), `INT0`, `ADC1`
        pub d2: attiny_hal::port::PB2 = pb2,
        /// `#3`: `PB3`, `3/A3`, `PCINT3`, `ACD3`, `XTAL1`
        pub d3: attiny_hal::port::PB3 = pb3,
        /// `#4`: `PB4`, `4/A2`, `PCINT4`, `OCB1`, `ADC2`, `XTAL2`
        pub d4: attiny_hal::port::PB4 = pb4,
        /// `#5`: `PB5`, `5/A0`, `PCINT5`, `RESET`, `ADC0`
        pub d5: attiny_hal::port::PB5 = pb5,
    }

    impl Pins {
        type Pin = Pin;
        type McuPins = attiny_hal::Pins;
    }
}
