pub use atmega_hal::port::mode;
pub use atmega_hal::port::Pin;

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins of the [**Adafruit Feather 32u4 Basic Proto**](https://www.adafruit.com/product/2771).
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][pins] macro.
    pub struct Pins from atmega_hal::Pins {
        /// `A0`
        ///
        /// * ADC0 (ADC input channel 0)
        /// * TDI
        pub a0: atmega_hal::port::PF7 = pf7,
        /// `A1`
        ///
        /// * ADC1 (ADC input channel 1)
        /// * TDO
        pub a1: atmega_hal::port::PF6 = pf6,
        /// `A2`
        ///
        /// * ADC2 (ADC input channel 2)
        /// * TMS
        pub a2: atmega_hal::port::PF5 = pf5,
        /// `A3`
        ///
        /// * ADC3 (ADC input channel 3)
        /// * TCK
        pub a3: atmega_hal::port::PF4 = pf4,
        /// `A4`
        ///
        /// * ADC4 (ADC input channel 4)
        pub a4: atmega_hal::port::PF1 = pf1,
        /// `A5`
        ///
        /// * ADC5 (ADC input channel 5)
        pub a5: atmega_hal::port::PF0 = pf0,

        /// `SCK`
        ///
        ///  * PCINT1
        pub sck: atmega_hal::port::PB1 = pb1,

        /// `MOSI`
        ///
        /// * PCINT2
        pub mosi: atmega_hal::port::PB2 = pb2,

        /// `MISO`
        ///
        /// * PCINT3
        pub miso: atmega_hal::port::PB3 = pb3,

        /// `D0` / `RX`
        ///
        /// * RXD (USART input pin)
        /// * INT2
        pub d0: atmega_hal::port::PD2 = pd2,
        /// `D1` / `TX`
        ///
        /// * TXD (USART output pin)
        /// * INT3
        pub d1: atmega_hal::port::PD3 = pd3,

        /// `D2`
        ///
        /// * INT1 (external interrupt 1 input)
        /// * SDA
        pub d2: atmega_hal::port::PD1 = pd1,

        /// `D3`
        ///
        /// * INT0 (external interrupt 1 input)
        /// * OC0B
        /// * SCL
        pub d3: atmega_hal::port::PD0 = pd0,

        /// `D5`
        ///
        /// * OC3A
        /// * ~OC4A
        pub d5: atmega_hal::port::PC6 = pc6,

        /// `D6`
        ///
        /// * OC4D
        /// * T0
        /// * ADC10
        pub d6: atmega_hal::port::PD7 = pd7,

        /// `D9`
        ///
        /// * OC1A
        /// * ~OC4B
        /// * PCINT5
        /// * ADC12
        pub d9: atmega_hal::port::PB5 = pb5,

        /// `D10`
        ///
        /// * OC1B (Timer/Counter1 output compare match B output)
        /// * OC4B
        /// * PCINT6
        /// * ADC13
        pub d10: atmega_hal::port::PB6 = pb6,

        /// `D11`
        ///
        /// * OC1C
        /// * OC0A
        /// * PCINT7
        /// * RTS
        pub d11: atmega_hal::port::PB7 = pb7,

        /// `D12`
        ///
        /// * T1
        /// * ~OC4D
        /// * ADC9
        pub d12: atmega_hal::port::PD6 = pd6,

        /// `D13`
        ///
        /// * CLK0
        /// * OC4A
        /// * ICP3
        /// * L LED
        pub d13: atmega_hal::port::PC7 = pc7,
    }
}

