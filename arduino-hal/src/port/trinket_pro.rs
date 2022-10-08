pub use atmega_hal::port::{mode, Pin, PinOps, PinMode};

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins of the **Trinket Pro**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
    pub struct Pins from atmega_hal::Pins {
        /// `A0`
        ///
        /// * ADC0 (ADC input channel 0)
        /// * PCINT8 (pin change interrupt 8)
        pub a0: atmega_hal::port::PC0 = pc0,
        /// `A1`
        ///
        /// * ADC1 (ADC input channel 1)
        /// * PCINT9 (pin change interrupt 9)
        pub a1: atmega_hal::port::PC1 = pc1,
        /// `A2`
        ///
        /// * ADC2 (ADC input channel 2)
        /// * PCINT10 (pin change interrupt 10)
        pub a2: atmega_hal::port::PC2 = pc2,
        /// `A3`
        ///
        /// * ADC3 (ADC input channel 3)
        /// * PCINT11 (pin change interrupt 11)
        pub a3: atmega_hal::port::PC3 = pc3,
        /// `A4`
        ///
        /// * ADC4 (ADC input channel 4)
        /// * SDA (2-wire serial bus data input/output line)
        /// * PCINT12 (pin change interrupt 12)
        pub a4: atmega_hal::port::PC4 = pc4,
        /// `A5`
        ///
        /// ADC5 (ADC input channel 5)
        /// SCL (2-wire serial bus clock line)
        /// PCINT13 (pin change interrupt 13)
        pub a5: atmega_hal::port::PC5 = pc5,

        /// `D0` / `RX`
        ///
        /// * RXD (USART input pin)
        /// * PCINT16 (pin change interrupt 16)
        pub d0: atmega_hal::port::PD0 = pd0,
        /// `D1` / `TX`
        ///
        /// * TXD (USART output pin)
        /// * PCINT17 (pin change interrupt 17)
        pub d1: atmega_hal::port::PD1 = pd1,
        /// `D3`
        ///
        /// * **PWM**: [atmega328p_hal::timer::Timer3Pwm]
        /// * INT1 (external interrupt 1 input)
        /// * OC2B (Timer/Counter2 output compare match B output)
        /// * PCINT19 (pin change interrupt 19)
        pub d3: atmega_hal::port::PD3 = pd3,
        /// `D4`
        ///
        /// * XCK (USART external clock input/output)
        /// * T0 (Timer/Counter 0 external counter input)
        /// * PCINT20 (pin change interrupt 20)
        pub d4: atmega_hal::port::PD4 = pd4,
        /// `D5`
        ///
        /// * **PWM**: [atmega328p_hal::timer::Timer3Pwm]
        /// * T1 (Timer/Counter 1 external counter input)
        /// * OC0B (Timer/Counter0 output compare match B output)
        /// * PCINT21 (pin change interrupt 21)
        pub d5: atmega_hal::port::PD5 = pd5,
        /// `D6`
        ///
        /// * **PWM**: [atmega328p_hal::timer::Timer3Pwm]
        /// * AIN0 (analog comparator positive input)
        /// * OC0A (Timer/Counter0 output compare match A output)
        /// * PCINT22 (pin change interrupt 22)
        pub d6: atmega_hal::port::PD6 = pd6,
        /// `D8`
        ///
        /// * ICP1 (Timer/Counter1 input capture input)
        /// * CLKO (divided system clock output)
        /// * PCINT0 (pin change interrupt 0)
        pub d8: atmega_hal::port::PB0 = pb0,
        /// `D9`
        ///
        /// * **PWM**: [atmega328p_hal::timer::Timer3Pwm]
        /// * OC1A (Timer/Counter1 output compare match A output)
        /// * PCINT1 (pin change interrupt 1)
        pub d9: atmega_hal::port::PB1 = pb1,
        /// `D10`
        ///
        /// * **PWM**: [atmega328p_hal::timer::Timer3Pwm]
        /// * SS (SPI bus master slave select)
        /// * OC1B (Timer/Counter1 output compare match B output)
        /// * PCINT2 (pin change interrupt 2)
        pub d10: atmega_hal::port::PB2 = pb2,
        /// `D11`
        ///
        /// * **PWM**: [atmega328p_hal::timer::Timer3Pwm]
        /// * MOSI (SPI bus master/slave input)
        /// * OC2A (Timer/Counter2 output compare match A output)
        /// * PCINT3 (pin change interrupt 3)
        pub d11: atmega_hal::port::PB3 = pb3,
        /// `D12`
        ///
        /// * MISO (SPI bus master input/slave output)
        /// * PCINT4 (pin change interrupt 4)
        pub d12: atmega_hal::port::PB4 = pb4,
        /// `D13`
        ///
        /// * SCK (SPI bus master clock input)
        /// * PCINT5 (pin change interrupt 5)
        /// * L LED on Trinket Pro
        pub d13: atmega_hal::port::PB5 = pb5,
    }
}
