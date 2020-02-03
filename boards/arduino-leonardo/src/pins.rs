use atmega32u4_hal::port::PortExt;

avr_hal_generic::impl_board_pins! {
    #[port_defs]
    use atmega32u4_hal::port;

    /// Generic DDR that works for all ports
    pub struct DDR {
        portb: crate::atmega32u4::PORTB,
        portc: crate::atmega32u4::PORTC,
        portd: crate::atmega32u4::PORTD,
        porte: crate::atmega32u4::PORTE,
        // Not used yet
        // portf: crate::atmega32u4::PORTF,
    }

    /// Reexport of the Leonardo's pins, with the names they have on the board
    pub struct Pins {
        /// `D0` / `RX`
        ///
        /// * `RX` (UART)
        /// * `INT2`: External Interrupt
        pub d0: portd::pd2::PD2,
        /// `D1` / `TX`
        ///
        /// * `TX` (UART)
        /// * `INT3`: External Interrupt
        pub d1: portd::pd3::PD3,
        /// `D2` / `SDA`
        ///
        /// * `SDA`: i2c/twi data
        /// * `INT1`: External Interrupt
        pub d2: portd::pd1::PD1,
        /// `D3` / `SCL`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer0Pwm]
        /// * `SCL`: i2c/twi clock
        /// * `INT0`: External Interrupt
        /// * `OC0B`: Output Compare Channel `B` for Timer/Counter0
        pub d3: portd::pd0::PD0,
        /// `D4`
        pub d4: portd::pd4::PD4,
        /// `D5`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer3Pwm]
        /// * `OC3A`: Output Compare Channel `A` for Timer/Counter3
        /// * `#OC4A`: Inverted Output Compare Channel `A` for Timer/Counter4 (Not implemented)
        pub d5: portc::pc6::PC6,
        /// `D6`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer4Pwm]
        /// * `OC4D`: Output Compare Channel `D` for Timer/Counter4
        pub d6: portd::pd7::PD7,
        /// `D7`
        ///
        /// * `INT6`: External Interrupt
        pub d7: porte::pe6::PE6,
        /// `D8`
        pub d8: portb::pb4::PB4,
        /// `D9`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer1Pwm]
        /// * `OC1A`: Output Compare Channel `A` for Timer/Counter1
        /// * `#OC4B`: Inverted Output Compare Channel `B` for Timer/Counter4 (Not implemented)
        pub d9: portb::pb5::PB5,
        /// `D10`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer1Pwm]
        /// * `OC1B`: Output Compare Channel `B` for Timer/Counter1
        /// * `OC4B`: Output Compare Channel `B` for Timer/Counter4 (Not implemented)
        pub d10: portb::pb6::PB6,
        /// `D11`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer0Pwm]
        /// * `OC0A`: Output Compare Channel `B` for Timer/Counter0
        /// * `OC1C`: Output Compare Channel `C` for Timer/Counter1
        pub d11: portb::pb7::PB7,
        /// `D12`
        ///
        /// * `#OC4D`: Inverted Output Compare Channel `D` for Timer/Counter4 (Not implemented)
        pub d12: portd::pd6::PD6,
        /// `D13` / `LED_BUILTIN`
        ///
        /// * Onboard LED
        /// * **PWM**: [atmega32u4_hal::timer::Timer4Pwm]
        /// * `OC4A`: Output Compare Channel `A` for Timer/Counter4
        pub d13: portc::pc7::PC7,
        /// `RX`
        ///
        /// Led for indicating inbound data.  Also the CS pin.
        pub led_rx: portb::pb0::PB0,
        /// `TX`
        ///
        /// Led for indicating outbound data
        pub led_tx: portd::pd5::PD5,
        /// `SCLK`
        ///
        /// ICSP SCLK pin
        pub icsp9: portb::pb1::PB1,
        /// `MOSI`
        ///
        /// ICSP MOSI pin
        pub icsp10: portb::pb2::PB2,
        /// `MISO`
        ///
        /// ICSP MISO pin
        pub icsp11: portb::pb3::PB3,
    }
}
