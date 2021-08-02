pub use attiny_hal::port::mode;
pub use attiny_hal::port::Pin;



avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins of the **Digispark Pro**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][pins] macro.
    pub struct Pins from attiny_hal::Pins {
        /// `0` / `SDA`
        ///
        /// * SDA (2-wire serial bus data input/output line)
        /// * PWM
        pub p0: attiny_hal::port::PB0 = pb0,
        /// `1` / `LED`
        ///
        /// * PWM
        pub p1: attiny_hal::port::PB1 = pb1,
        /// `2`
        ///
        /// * SCL (2-wire serial bus clock line)
        /// * PWM
        pub p2: attiny_hal::port::PB2 = pb2,
        /// `3` / `A3`
        ///
        /// * PWM
        /// * USB D+
        /// * ADC3 (ADC input channel 3)
        pub p3: attiny_hal::port::PB6 = pb6,
        /// `4`
        ///
        /// * PWM
        /// * USB D-
        pub p4: attiny_hal::port::PB3 = pb3,
        /// `5` / `A5`
        ///
        /// * ADC5
        pub p5: attiny_hal::port::PA7 = pa7,
        /// `6`
        ///
        /// * ADC6 (ADC input channel 6)
        /// * RX
        pub p6: attiny_hal::port::PA0 = pa0,
        /// `7`
        ///
        /// * ADC7
        /// * TX
        pub p7: attiny_hal::port::PA1 = pa1,
        /// `8`
        ///
        /// * PWM
        /// * ADC8
        /// * MISO
        pub p8: attiny_hal::port::PA2 = pa2,
        /// `9`
        ///
        /// * ADC9
        pub p9: attiny_hal::port::PA3 = pa3,
        /// `10`
        ///
        /// * ADC10
        /// * MOSI
        pub p10: attiny_hal::port::PA4 = pa4,
        /// `11`
        ///
        /// * ADC11
        /// * SCK
        pub p11: attiny_hal::port::PA5 = pa5,
        /// `12`
        ///
        /// * ADC12
        /// * SS
        pub p12: attiny_hal::port::PA6 = pa6,
        /// `RST`
        ///
        pub rst: attiny_hal::port::PB7 = pb7,
    }
}
