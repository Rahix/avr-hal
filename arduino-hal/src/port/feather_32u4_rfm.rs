pub use atmega_hal::port::mode;
pub use atmega_hal::port::Pin;

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins of the **Adafruit Feather 32u4 RFM69 and RFM9* **.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
    /// [Reference](https://github.com/adafruit/Adafruit-Feather-32u4-Basic-Proto-PCB/blob/master/Adafruit%20Feather%2032u4%20Basic%20Proto%20Pinout.pdf)
    pub struct Pins from atmega_hal::Pins {
        /// `A0`
        ///
        /// * ADC7 (ADC input channel 7)
        /// * TDI
        pub a0: atmega_hal::port::PF7 = pf7,
        /// `A1`
        ///
        /// * ADC6 (ADC input channel 6)
        /// * TDO
        pub a1: atmega_hal::port::PF6 = pf6,
        /// `A2`
        ///
        /// * ADC5 (ADC input channel 5)
        /// * TMS
        pub a2: atmega_hal::port::PF5 = pf5,
        /// `A3`
        ///
        /// * ADC4 (ADC input channel 4)
        /// * TCK
        pub a3: atmega_hal::port::PF4 = pf4,
        /// `A4`
        ///
        /// * ADC1 (ADC input channel 1)
        pub a4: atmega_hal::port::PF1 = pf1,
        /// `A5`
        ///
        /// * ADC0 (ADC input channel 0)
        pub a5: atmega_hal::port::PF0 = pf0,

        /// `SCK`
        ///  (Used by radio module too)
        ///  * PCINT1
        pub sck: atmega_hal::port::PB1 = pb1,

        /// `MOSI`
        ///  (Used by radio module too)
        /// * PCINT2
        pub mosi: atmega_hal::port::PB2 = pb2,

        /// `MISO`
        ///  (Used by radio module too)
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
        /// * OC4A
        pub d5: atmega_hal::port::PC6 = pc6,

        /// `D6`
        ///
        /// * OC4D
        /// * ADC10
        pub d6: atmega_hal::port::PD7 = pd7,

        /// `D9`
        ///
        /// * OC1A
        /// * !OC4B
        /// * PCINT5
        /// * ADC12
        pub d9: atmega_hal::port::PB5 = pb5,

        /// `D10`
        ///
        /// * OC1B
        /// * OC4B
        /// * PCINT6
        /// * ADC13
        pub d10: atmega_hal::port::PB6 = pb6,

        /// `D11`
        ///
        /// * OC0A
        /// * OC1C
        /// * PCINT7
        pub d11: atmega_hal::port::PB7 = pb7,

        /// `D12`
        ///
        /// * !OC4D
        /// * ADC9
        pub d12: atmega_hal::port::PD6 = pd6,

        /// `D13`
        ///
        /// * OC4A
        pub d13: atmega_hal::port::PC7 = pc7,
    
        /// `D8`
        /// For radio module
        /// 
        /// * PCINT4
        /// * CS
        /// * ADC11
        pub d8: atmega_hal::port::PB4 = pb4,

        /// `D7`
        /// For radio module
        /// 
        /// * INT6
        /// * IRQ
        /// * AIN0
        pub d7: atmega_hal::port::PE6 = pe6,

        /// `D4`
        /// For radio module
        /// 
        /// * ICP1
        /// * RST
        /// * ADC8
        pub d4: atmega_hal::port::PD4 = pd4,


    }
}