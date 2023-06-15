pub use atmega_hal::port::{mode, Pin, PinOps, PinMode};

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins of the **SparkFun ProMicro**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
    pub struct Pins from atmega_hal::Pins {
        /// `RX`
        ///
        /// `RX` (UART)
        pub rx: atmega_hal::port::PD2 = pd2,
        /// `TX`
        ///
        /// `TX` (UART)
        pub tx: atmega_hal::port::PD3 = pd3,
        /// `D2` / `SDA`
        ///
        /// `SDA`: i2c/twi data
        pub d2: atmega_hal::port::PD1 = pd1,
        /// `D3` / `SCL`
        ///
        /// `SCL`: i2c/twi clock
        pub d3: atmega_hal::port::PD0 = pd0,
        /// `D4`
        pub d4: atmega_hal::port::PD4 = pd4,
        /// `D5`
        pub d5: atmega_hal::port::PC6 = pc6,
        /// `D6`
        pub d6: atmega_hal::port::PD7 = pd7,
        /// `D7`
        pub d7: atmega_hal::port::PE6 = pe6,
        /// `D8`
        pub d8: atmega_hal::port::PB4 = pb4,
        /// `D9`
        pub d9: atmega_hal::port::PB5 = pb5,
        /// `D10`
        pub d10: atmega_hal::port::PB6 = pb6,
        /// `LED_RX`
        ///
        /// Led for indicating inbound data (yellow).  Also the CS pin for SPI.
        pub led_rx: atmega_hal::port::PB0 = pb0,
        /// `LED_TX`
        ///
        /// Led for indicating outbound data (green).
        pub led_tx: atmega_hal::port::PD5 = pd5,
        /// `D15`, `SCK`
        ///
        /// ICSP SCLK pin
        pub d15: atmega_hal::port::PB1 = pb1,
        /// `D14`, `MISO`
        ///
        /// ICSP MISO pin
        pub d16: atmega_hal::port::PB3 = pb3,
        /// `D16`, `MOSI`
        ///
        /// ICSP MOSI pin
        pub d14: atmega_hal::port::PB2 = pb2,
        /// `A0`
        ///
        /// `ADC7` channel
        pub a0: atmega_hal::port::PF7 = pf7,
        /// `A1`
        ///
        /// `ADC6` channel
        pub a1: atmega_hal::port::PF6 = pf6,
        /// `A2`
        ///
        /// `ADC5` channel
        pub a2: atmega_hal::port::PF5 = pf5,
        /// `A3`
        ///
        /// `ADC4` channel
        pub a3: atmega_hal::port::PF4 = pf4,
    }
}
