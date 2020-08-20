use atmega168_hal::port::PortExt;

avr_hal_generic::impl_board_pins! {
    #[port_defs]
    use atmega168_hal::port;

    /// Generic DDR that works for all ports
    pub struct DDR {
        portb: crate::atmega168::PORTB,
        portc: crate::atmega168::PORTC,
        portd: crate::atmega168::PORTD,
    }

    /// Reexport of the Diecimila's pins, with the names they have on the board
    pub struct Pins {
        /// `A0`
        ///
        /// * ADC0 (ADC input channel 0)
        /// * PCINT8 (pin change interrupt 8)
        pub a0: portc::pc0::PC0,
        /// `A1`
        ///
        /// * ADC1 (ADC input channel 1)
        /// * PCINT9 (pin change interrupt 9)
        pub a1: portc::pc1::PC1,
        /// `A2`
        ///
        /// * ADC2 (ADC input channel 2)
        /// * PCINT10 (pin change interrupt 10)
        pub a2: portc::pc2::PC2,
        /// `A3`
        ///
        /// * ADC3 (ADC input channel 3)
        /// * PCINT11 (pin change interrupt 11)
        pub a3: portc::pc3::PC3,
        /// `A4`
        ///
        /// * ADC4 (ADC input channel 4)
        /// * SDA (2-wire serial bus data input/output line)
        /// * PCINT12 (pin change interrupt 12)
        pub a4: portc::pc4::PC4,
        /// `A5`
        ///
        /// ADC5 (ADC input channel 5)
        /// SCL (2-wire serial bus clock line)
        /// PCINT13 (pin change interrupt 13)
        pub a5: portc::pc5::PC5,

        /// `D0` / `RX`
        ///
        /// * RXD (USART input pin)
        /// * PCINT16 (pin change interrupt 16)
        pub d0: portd::pd0::PD0,
        /// `D1` / `TX`
        ///
        /// * TXD (USART output pin)
        /// * PCINT17 (pin change interrupt 17)
        pub d1: portd::pd1::PD1,
        /// `D2`
        ///
        /// * INT0 (external interrupt 0 input)
        /// * PCINT18 (pin change interrupt 18)
        pub d2: portd::pd2::PD2,
        /// `D3`
        ///
        /// * **PWM**: [atmega168_hal::timer::Timer3Pwm]
        /// * INT1 (external interrupt 1 input)
        /// * OC2B (Timer/Counter2 output compare match B output)
        /// * PCINT19 (pin change interrupt 19)
        pub d3: portd::pd3::PD3,
        /// `D4`
        ///
        /// * XCK (USART external clock input/output)
        /// * T0 (Timer/Counter 0 external counter input)
        /// * PCINT20 (pin change interrupt 20)
        pub d4: portd::pd4::PD4,
        /// `D5`
        ///
        /// * **PWM**: [atmega168_hal::timer::Timer3Pwm]
        /// * T1 (Timer/Counter 1 external counter input)
        /// * OC0B (Timer/Counter0 output compare match B output)
        /// * PCINT21 (pin change interrupt 21)
        pub d5: portd::pd5::PD5,
        /// `D6`
        ///
        /// * **PWM**: [atmega168_hal::timer::Timer3Pwm]
        /// * AIN0 (analog comparator positive input)
        /// * OC0A (Timer/Counter0 output compare match A output)
        /// * PCINT22 (pin change interrupt 22)
        pub d6: portd::pd6::PD6,
        /// `D7`
        ///
        /// * AIN1 (analog comparator negative input)
        /// * PCINT23 (pin change interrupt 23)
        pub d7: portd::pd7::PD7,
        /// `D8`
        ///
        /// * ICP1 (Timer/Counter1 input capture input)
        /// * CLKO (divided system clock output)
        /// * PCINT0 (pin change interrupt 0)
        pub d8: portb::pb0::PB0,
        /// `D9`
        ///
        /// * **PWM**: [atmega168_hal::timer::Timer3Pwm]
        /// * OC1A (Timer/Counter1 output compare match A output)
        /// * PCINT1 (pin change interrupt 1)
        pub d9: portb::pb1::PB1,
        /// `D10`
        ///
        /// * **PWM**: [atmega168_hal::timer::Timer3Pwm]
        /// * SS (SPI bus master slave select)
        /// * OC1B (Timer/Counter1 output compare match B output)
        /// * PCINT2 (pin change interrupt 2)
        pub d10: portb::pb2::PB2,
        /// `D11`
        ///
        /// * **PWM**: [atmega168_hal::timer::Timer3Pwm]
        /// * MOSI (SPI bus master/slave input)
        /// * OC2A (Timer/Counter2 output compare match A output)
        /// * PCINT3 (pin change interrupt 3)
        pub d11: portb::pb3::PB3,
        /// `D12`
        ///
        /// * MISO (SPI bus master input/slave output)
        /// * PCINT4 (pin change interrupt 4)
        pub d12: portb::pb4::PB4,
        /// `D13`
        ///
        /// * SCK (SPI bus master clock input)
        /// * PCINT5 (pin change interrupt 5)
        /// * L LED on Arduino Uno
        pub d13: portb::pb5::PB5,
    }
}
