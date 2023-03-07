pub use attiny_hal::port::{mode, Pin, PinOps, PinMode};

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins of the **Digispark (model A)** .
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
    pub struct Pins from attiny_hal::Pins {
        /// `P0`
        ///
        /// * PWM: [attiny_hal::timer::Timer0Pwm]
        /// * MOSI (SPI bus master/slave input)
        /// * SDA (2-wire serial bus data input/output line)
        /// * PCINT0 (pin change interrupt 0)
        pub p0: attiny_hal::port::PB0 = pb0,
        /// `P1`
        ///
        /// * PWM: [attiny_hal::timer::Timer0Pwm]
        /// * MISO (SPI bus master input/slave output)
        /// * PCINT1 (pin change interrupt 1)
        pub p1: attiny_hal::port::PB1 = pb1,
        /// `P2`
        ///
        /// * ADC1 (ADC input channel 1)
        /// * SCK (SPI bus master clock input)
        /// * SCL (2-wire serial bus clock line)
        /// * PCINT2 (pin change interrupt 2)
        pub p2: attiny_hal::port::PB2 = pb2,
        /// `P3`
        ///
        /// * ADC3 (ADC input channel 3)
        /// * USB+ (USB data + pin)
        /// * PCINT3 (pin change interrupt 3)
        pub p3: attiny_hal::port::PB3 = pb3,
        /// `P4`
        ///
        /// * PWM: [attiny_hal::timer::Timer1Pwm]
        /// * ADC2 (ADC input channel 2)
        /// * USB- (USB data - pin)
        /// * PCINT4 (pin change interrupt 4)
        pub p4: attiny_hal::port::PB4 = pb4,
        /// `P5`
        ///
        /// * ADC0 (ADC input channel 0)
        /// * PCINT5 (pin change interrupt 5)
        pub p5: attiny_hal::port::PB5 = pb5,
    }
}
