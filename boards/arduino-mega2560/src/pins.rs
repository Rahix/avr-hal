use atmega2560_hal::port::PortExt;

avr_hal_generic::impl_board_pins! {
    #[port_defs]
    use atmega2560_hal::port;

    /// Generic DDR that works for all ports
    pub struct DDR {
        porta: crate::atmega2560::PORTA,
        portb: crate::atmega2560::PORTB,
        portc: crate::atmega2560::PORTC,
        portd: crate::atmega2560::PORTD,
        porte: crate::atmega2560::PORTE,
        portf: crate::atmega2560::PORTF,
        portg: crate::atmega2560::PORTG,
        porth: crate::atmega2560::PORTH,
        portj: crate::atmega2560::PORTJ,
        portk: crate::atmega2560::PORTK,
        portl: crate::atmega2560::PORTL,
    }

    /// Reexport of the Mega 2560's pins, with the names they have on the board
    pub struct Pins {
        /// `D0` / `RX0`
        ///
        /// * `RXD0` (USART0)
        /// * `PCINT8`: External Interrupt (Pin Change)
        pub d0: porte::pe0::PE0,
        /// `D1` / `TX0`
        ///
        /// * `TXD0` (USART0)
        pub d1: porte::pe1::PE1,
        /// `D2`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer3Pwm]
        /// * `OC3B`: Output Compare Channel `B` for Timer/Counter3
        /// * `INT4`: External Interrupt
        pub d2: porte::pe4::PE4,
        /// `D3`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer3Pwm]
        /// * `OC3C`: Output Compare Channel `C` for Timer/Counter3
        /// * `INT5`: External Interrupt
        pub d3: porte::pe5::PE5,
        /// `D4`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer0Pwm]
        /// * `OC0B`: Output Compare Channel `B` for Timer/Counter0
        pub d4: portg::pg5::PG5,
        /// `D5`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer3Pwm]
        /// * `OC3A`: Output Compare Channel `A` for Timer/Counter3
        /// * `AIN1`: Analog Comparator Negative Input (Not Implemented)
        pub d5: porte::pe3::PE3,
        /// `D6`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer4Pwm]
        /// * `OC4A`: Output Compare Channel `A` for Timer/Counter4
        pub d6: porth::ph3::PH3,
        /// `D7`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer4Pwm]
        /// * `OC4B`: Output Compare Channel `B` for Timer/Counter4
        pub d7: porth::ph4::PH4,
        /// `D8`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer4Pwm]
        /// * `OC4C`: Output Compare Channel `C` for Timer/Counter4
        pub d8: porth::ph5::PH5,
        /// `D9`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer2Pwm]
        /// * `OC2B`: Output Compare Channel `B` for Timer/Counter2
        pub d9: porth::ph6::PH6,
        /// `D10`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer2Pwm]
        /// * `OC2B`: Output Compare Channel `B` for Timer/Counter2
        /// * `PCINT4`: External Interrupt (Pin Change)
        pub d10: portb::pb4::PB4,
        /// `D11`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer1Pwm]
        /// * `OC1A`: Output Compare Channel `A` for Timer/Counter1
        /// * `PCINT5`: External Interrupt (Pin Change)
        pub d11: portb::pb5::PB5,
        /// `D12`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer1Pwm]
        /// * `OC1B`: Output Compare Channel `B` for Timer/Counter1
        /// * `PCINT6`: External Interrupt (Pin Change)
        pub d12: portb::pb6::PB6,
        /// `D13`
        ///
        /// * Onboard LED
        /// * **PWM**: [atmega2560_hal::timer::Timer0Pwm]
        /// * **PWM**: [atmega2560_hal::timer::Timer1Pwm]
        /// * `OC0A`: Output Compare Channel `A` for Timer/Counter0
        /// * `OC1C`: Output Compare Channel `C` for Timer/Counter1
        /// * `PCINT7`: External Interrupt (Pin Change)
        pub d13: portb::pb7::PB7,
        /// `D14` / `TX3`
        ///
        /// * `TXD3` (USART3)
        /// * `PCINT10`: External Interrupt (Pin Change)
        pub d14: portj::pj1::PJ1,
        /// `D15` / `RX3`
        ///
        /// * `RXD3` (USART3)
        /// * `PCINT9`: External Interrupt (Pin Change)
        pub d15: portj::pj0::PJ0,
        /// `D16` / `TX2`
        ///
        /// * `TXD2` (USART2)
        pub d16: porth::ph1::PH1,
        /// `D17` / `RX2`
        ///
        /// * `RXD2` (USART2)
        pub d17: porth::ph0::PH0,
        /// `D18` / `TX1`
        ///
        /// * `TXD1` (USART1)
        /// * `INT3`: External Interrupt
        pub d18: portd::pd3::PD3,
        /// `D19` / `RX1`
        ///
        /// * `RXD1` (USART1)
        /// * `INT2`: External Interrupt
        pub d19: portd::pd2::PD2,
        /// `D20` / `SDA`
        ///
        /// * `SDA`: i2c/twi data
        /// * `INT1`: External Interrupt
        pub d20: portd::pd1::PD1,
        /// `D21` / `SCL`
        ///
        /// * `SCL`: i2c/twi clock
        /// * `INT0`: External Interrupt
        pub d21: portd::pd0::PD0,
        /// `D22`
        ///
        /// * `AD0`: External memory interface
        pub d22: porta::pa0::PA0,
        /// `D23`
        ///
        /// * `AD1`: External memory interface
        pub d23: porta::pa1::PA1,
        /// `D24`
        ///
        /// * `AD2`: External memory interface
        pub d24: porta::pa2::PA2,
        /// `D25`
        ///
        /// * `AD3`: External memory interface
        pub d25: porta::pa3::PA3,
        /// `D26`
        ///
        /// * `AD4`: External memory interface
        pub d26: porta::pa4::PA4,
        /// `D27`
        ///
        /// * `AD5`: External memory interface
        pub d27: porta::pa5::PA5,
        /// `D28`
        ///
        /// * `AD6`: External memory interface
        pub d28: porta::pa6::PA6,
        /// `D29`
        ///
        /// * `AD7`: External memory interface
        pub d29: porta::pa7::PA7,
        /// `D30`
        ///
        /// * `AD15`: External memory interface
        pub d30: portc::pc7::PC7,
        /// `D31`
        ///
        /// * `AD14`: External memory interface
        pub d31: portc::pc6::PC6,
        /// `D32`
        ///
        /// * `AD13`: External memory interface
        pub d32: portc::pc5::PC5,
        /// `D33`
        ///
        /// * `AD12`: External memory interface
        pub d33: portc::pc4::PC4,
        /// `D34`
        ///
        /// * `AD11`: External memory interface
        pub d34: portc::pc3::PC3,
        /// `D35`
        ///
        /// * `AD10`: External memory interface
        pub d35: portc::pc2::PC2,
        /// `D36`
        ///
        /// * `AD9`: External memory interface
        pub d36: portc::pc1::PC1,
        /// `D37`
        ///
        /// * `AD8`: External memory interface
        pub d37: portc::pc0::PC0,
        /// `D38`
        ///
        /// * `T0`: Clock Input for Timer/Counter0
        pub d38: portd::pd7::PD7,
        /// `D39`
        ///
        /// * `ALE`: External memory Address Latch Enable
        pub d39: portg::pg2::PG2,
        /// `D40`
        ///
        /// `RD`: External memory Read Strobe
        pub d40: portg::pg1::PG1,
        /// `D41`
        ///
        /// `WR`: External memory Write Strobe
        pub d41: portg::pg0::PG0,
        /// `D42`
        pub d42: portl::pl7::PL7,
        /// `D43`
        pub d43: portl::pl6::PL6,
        /// `D44`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer5Pwm]
        /// * `OC5C`: Output Compare Channel `C` for Timer/Counter5
        pub d44: portl::pl5::PL5,
        /// `D45`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer5Pwm]
        /// * `OC5B`: Output Compare Channel `B` for Timer/Counter5
        pub d45: portl::pl4::PL4,
        /// `D46`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer5Pwm]
        /// * `OC5A`: Output Compare Channel `A` for Timer/Counter5
        pub d46: portl::pl3::PL3,
        /// `D47`
        ///
        /// * `T5`: Clock Input for Timer/Counter5
        pub d47: portl::pl2::PL2,
        /// `D48`
        ///
        /// * `ICP5`: Input Capture Trigger for Timer/Counter5
        pub d48: portl::pl1::PL1,
        /// `D49`
        ///
        /// * `ICP4`: Input Capture Trigger for Timer/Counter4
        pub d49: portl::pl0::PL0,
        /// `D50`
        ///
        /// * `MISO`: SPI bus Master In/Slave Out
        /// * `PCINT3`: External Interrupt (Pin Change)
        pub d50: portb::pb3::PB3,
        /// `D51`
        ///
        /// * `MOSI`: SPI bus Master Out/Slave In
        /// * `PCINT2`: External Interrupt (Pin Change)
        pub d51: portb::pb2::PB2,
        /// `D52`
        ///
        /// * `SCK`: SPI bus Serial Clock
        /// * `PCINT1`: External Interrupt (Pin Change)
        pub d52: portb::pb1::PB1,
        /// `D53`
        ///
        /// * `SS`: SPI bus Slave Select
        /// * `PCINT0`: External Interrupt (Pin Change)
        pub d53: portb::pb0::PB0,
        /// `A0`
        ///
        /// * `ADC0`: A/D converter input 0
        pub a0: portf::pf0::PF0,
        /// `A1`
        ///
        /// * `ADC1`: A/D converter input 1
        pub a1: portf::pf1::PF1,
        /// `A2`
        ///
        /// * `ADC2`: A/D converter input 2
        pub a2: portf::pf2::PF2,
        /// `A3`
        ///
        /// * `ADC3`: A/D converter input 3
        pub a3: portf::pf3::PF3,
        /// `A4`
        ///
        /// * `ADC4`: A/D converter input 4
        /// * `TCK`: JTAG test clock
        pub a4: portf::pf4::PF4,
        /// `A5`
        ///
        /// * `ADC5`: A/D converter input 5
        /// * `TMS`: JTAG test mode select
        pub a5: portf::pf5::PF5,
        /// `A6`
        ///
        /// * `ADC6`: A/D converter input 6
        /// * `TDO`: JTAG test data output
        pub a6: portf::pf6::PF6,
        /// `A7`
        ///
        /// * `ADC7`: A/D converter input 7
        /// * `TDI`: JTAG test data input
        pub a7: portf::pf7::PF7,
        /// `A8`
        ///
        /// * `ADC8`: A/D converter input 8
        /// * `PCINT16`: External Interrupt (Pin Change)
        pub a8: portk::pk0::PK0,
        /// `A9`
        ///
        /// * `ADC9`: A/D converter input 9
        /// * `PCINT17`: External Interrupt (Pin Change)
        pub a9: portk::pk1::PK1,
        /// `A10`
        ///
        /// * `ADC10`: A/D converter input 10
        /// * `PCINT18`: External Interrupt (Pin Change)
        pub a10: portk::pk2::PK2,
        /// `A11`
        ///
        /// * `ADC11`: A/D converter input 11
        /// * `PCINT19`: External Interrupt (Pin Change)
        pub a11: portk::pk3::PK3,
        /// `A12`
        ///
        /// * `ADC12`: A/D converter input 12
        /// * `PCINT20`: External Interrupt (Pin Change)
        pub a12: portk::pk4::PK4,
        /// `A13`
        ///
        /// * `ADC13`: A/D converter input 13
        /// * `PCINT21`: External Interrupt (Pin Change)
        pub a13: portk::pk5::PK5,
        /// `A14`
        ///
        /// * `ADC14`: A/D converter input 14
        /// * `PCINT22`: External Interrupt (Pin Change)
        pub a14: portk::pk6::PK6,
        /// `A15`
        ///
        /// * `ADC15`: A/D converter input 15
        /// * `PCINT23`: External Interrupt (Pin Change)
        pub a15: portk::pk7::PK7,
    }
}
