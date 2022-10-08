pub use atmega_hal::port::{mode, Pin, PinOps, PinMode};

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins of the **Arduino Mega 2560**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
    pub struct Pins from atmega_hal::Pins {
        /// `D0` / `RX0`
        ///
        /// * `RXD0` (USART0)
        /// * `PCINT8`: External Interrupt (Pin Change)
        pub d0: atmega_hal::port::PE0 = pe0,
        /// `D1` / `TX0`
        ///
        /// * `TXD0` (USART0)
        pub d1: atmega_hal::port::PE1 = pe1,
        /// `D2`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer3Pwm]
        /// * `OC3B`: Output Compare Channel `B` for Timer/Counter3
        /// * `INT4`: External Interrupt
        pub d2: atmega_hal::port::PE4 = pe4,
        /// `D3`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer3Pwm]
        /// * `OC3C`: Output Compare Channel `C` for Timer/Counter3
        /// * `INT5`: External Interrupt
        pub d3: atmega_hal::port::PE5 = pe5,
        /// `D4`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer0Pwm]
        /// * `OC0B`: Output Compare Channel `B` for Timer/Counter0
        pub d4: atmega_hal::port::PG5 = pg5,
        /// `D5`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer3Pwm]
        /// * `OC3A`: Output Compare Channel `A` for Timer/Counter3
        /// * `AIN1`: Analog Comparator Negative Input (Not Implemented)
        pub d5: atmega_hal::port::PE3 = pe3,
        /// `D6`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer4Pwm]
        /// * `OC4A`: Output Compare Channel `A` for Timer/Counter4
        pub d6: atmega_hal::port::PH3 = ph3,
        /// `D7`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer4Pwm]
        /// * `OC4B`: Output Compare Channel `B` for Timer/Counter4
        pub d7: atmega_hal::port::PH4 = ph4,
        /// `D8`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer4Pwm]
        /// * `OC4C`: Output Compare Channel `C` for Timer/Counter4
        pub d8: atmega_hal::port::PH5 = ph5,
        /// `D9`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer2Pwm]
        /// * `OC2B`: Output Compare Channel `B` for Timer/Counter2
        pub d9: atmega_hal::port::PH6 = ph6,
        /// `D10`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer2Pwm]
        /// * `OC2B`: Output Compare Channel `B` for Timer/Counter2
        /// * `PCINT4`: External Interrupt (Pin Change)
        pub d10: atmega_hal::port::PB4 = pb4,
        /// `D11`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer1Pwm]
        /// * `OC1A`: Output Compare Channel `A` for Timer/Counter1
        /// * `PCINT5`: External Interrupt (Pin Change)
        pub d11: atmega_hal::port::PB5 = pb5,
        /// `D12`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer1Pwm]
        /// * `OC1B`: Output Compare Channel `B` for Timer/Counter1
        /// * `PCINT6`: External Interrupt (Pin Change)
        pub d12: atmega_hal::port::PB6 = pb6,
        /// `D13`
        ///
        /// * Onboard LED
        /// * **PWM**: [atmega2560_hal::timer::Timer0Pwm]
        /// * **PWM**: [atmega2560_hal::timer::Timer1Pwm]
        /// * `OC0A`: Output Compare Channel `A` for Timer/Counter0
        /// * `OC1C`: Output Compare Channel `C` for Timer/Counter1
        /// * `PCINT7`: External Interrupt (Pin Change)
        pub d13: atmega_hal::port::PB7 = pb7,
        /// `D14` / `TX3`
        ///
        /// * `TXD3` (USART3)
        /// * `PCINT10`: External Interrupt (Pin Change)
        pub d14: atmega_hal::port::PJ1 = pj1,
        /// `D15` / `RX3`
        ///
        /// * `RXD3` (USART3)
        /// * `PCINT9`: External Interrupt (Pin Change)
        pub d15: atmega_hal::port::PJ0 = pj0,
        /// `D16` / `TX2`
        ///
        /// * `TXD2` (USART2)
        pub d16: atmega_hal::port::PH1 = ph1,
        /// `D17` / `RX2`
        ///
        /// * `RXD2` (USART2)
        pub d17: atmega_hal::port::PH0 = ph0,
        /// `D18` / `TX1`
        ///
        /// * `TXD1` (USART1)
        /// * `INT3`: External Interrupt
        pub d18: atmega_hal::port::PD3 = pd3,
        /// `D19` / `RX1`
        ///
        /// * `RXD1` (USART1)
        /// * `INT2`: External Interrupt
        pub d19: atmega_hal::port::PD2 = pd2,
        /// `D20` / `SDA`
        ///
        /// * `SDA`: i2c/twi data
        /// * `INT1`: External Interrupt
        pub d20: atmega_hal::port::PD1 = pd1,
        /// `D21` / `SCL`
        ///
        /// * `SCL`: i2c/twi clock
        /// * `INT0`: External Interrupt
        pub d21: atmega_hal::port::PD0= pd0,
        /// `D22`
        ///
        /// * `AD0`: External memory interface
        pub d22: atmega_hal::port::PA0 = pa0,
        /// `D23`
        ///
        /// * `AD1`: External memory interface
        pub d23: atmega_hal::port::PA1 = pa1,
        /// `D24`
        ///
        /// * `AD2`: External memory interface
        pub d24: atmega_hal::port::PA2 = pa2,
        /// `D25`
        ///
        /// * `AD3`: External memory interface
        pub d25: atmega_hal::port::PA3 = pa3,
        /// `D26`
        ///
        /// * `AD4`: External memory interface
        pub d26: atmega_hal::port::PA4 = pa4,
        /// `D27`
        ///
        /// * `AD5`: External memory interface
        pub d27: atmega_hal::port::PA5 = pa5,
        /// `D28`
        ///
        /// * `AD6`: External memory interface
        pub d28: atmega_hal::port::PA6 = pa6,
        /// `D29`
        ///
        /// * `AD7`: External memory interface
        pub d29: atmega_hal::port::PA7 = pa7,
        /// `D30`
        ///
        /// * `AD15`: External memory interface
        pub d30: atmega_hal::port::PC7 = pc7,
        /// `D31`
        ///
        /// * `AD14`: External memory interface
        pub d31: atmega_hal::port::PC6 = pc6,
        /// `D32`
        ///
        /// * `AD13`: External memory interface
        pub d32: atmega_hal::port::PC5 = pc5,
        /// `D33`
        ///
        /// * `AD12`: External memory interface
        pub d33: atmega_hal::port::PC4 = pc4,
        /// `D34`
        ///
        /// * `AD11`: External memory interface
        pub d34: atmega_hal::port::PC3 = pc3,
        /// `D35`
        ///
        /// * `AD10`: External memory interface
        pub d35: atmega_hal::port::PC2 = pc2,
        /// `D36`
        ///
        /// * `AD9`: External memory interface
        pub d36: atmega_hal::port::PC1 = pc1,
        /// `D37`
        ///
        /// * `AD8`: External memory interface
        pub d37: atmega_hal::port::PC0 = pc0,
        /// `D38`
        ///
        /// * `T0`: Clock Input for Timer/Counter0
        pub d38: atmega_hal::port::PD7 = pd7,
        /// `D39`
        ///
        /// * `ALE`: External memory Address Latch Enable
        pub d39: atmega_hal::port::PG2 = pg2,
        /// `D40`
        ///
        /// `RD`: External memory Read Strobe
        pub d40: atmega_hal::port::PG1 = pg1,
        /// `D41`
        ///
        /// `WR`: External memory Write Strobe
        pub d41: atmega_hal::port::PG0 = pg0,
        /// `D42`
        pub d42: atmega_hal::port::PL7 = pl7,
        /// `D43`
        pub d43: atmega_hal::port::PL6 = pl6,
        /// `D44`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer5Pwm]
        /// * `OC5C`: Output Compare Channel `C` for Timer/Counter5
        pub d44: atmega_hal::port::PL5 = pl5,
        /// `D45`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer5Pwm]
        /// * `OC5B`: Output Compare Channel `B` for Timer/Counter5
        pub d45: atmega_hal::port::PL4 = pl4,
        /// `D46`
        ///
        /// * **PWM**: [atmega2560_hal::timer::Timer5Pwm]
        /// * `OC5A`: Output Compare Channel `A` for Timer/Counter5
        pub d46: atmega_hal::port::PL3 = pl3,
        /// `D47`
        ///
        /// * `T5`: Clock Input for Timer/Counter5
        pub d47: atmega_hal::port::PL2 = pl2,
        /// `D48`
        ///
        /// * `ICP5`: Input Capture Trigger for Timer/Counter5
        pub d48: atmega_hal::port::PL1 = pl1,
        /// `D49`
        ///
        /// * `ICP4`: Input Capture Trigger for Timer/Counter4
        pub d49: atmega_hal::port::PL0 = pl0,
        /// `D50`
        ///
        /// * `MISO`: SPI bus Master In/Slave Out
        /// * `PCINT3`: External Interrupt (Pin Change)
        pub d50: atmega_hal::port::PB3 = pb3,
        /// `D51`
        ///
        /// * `MOSI`: SPI bus Master Out/Slave In
        /// * `PCINT2`: External Interrupt (Pin Change)
        pub d51: atmega_hal::port::PB2 = pb2,
        /// `D52`
        ///
        /// * `SCK`: SPI bus Serial Clock
        /// * `PCINT1`: External Interrupt (Pin Change)
        pub d52: atmega_hal::port::PB1 = pb1,
        /// `D53`
        ///
        /// * `SS`: SPI bus Slave Select
        /// * `PCINT0`: External Interrupt (Pin Change)
        pub d53: atmega_hal::port::PB0 = pb0,
        /// `A0`
        ///
        /// * `ADC0`: A/D converter input 0
        pub a0: atmega_hal::port::PF0 = pf0,
        /// `A1`
        ///
        /// * `ADC1`: A/D converter input 1
        pub a1: atmega_hal::port::PF1 = pf1,
        /// `A2`
        ///
        /// * `ADC2`: A/D converter input 2
        pub a2: atmega_hal::port::PF2 = pf2,
        /// `A3`
        ///
        /// * `ADC3`: A/D converter input 3
        pub a3: atmega_hal::port::PF3 = pf3,
        /// `A4`
        ///
        /// * `ADC4`: A/D converter input 4
        /// * `TCK`: JTAG test clock
        pub a4: atmega_hal::port::PF4 = pf4,
        /// `A5`
        ///
        /// * `ADC5`: A/D converter input 5
        /// * `TMS`: JTAG test mode select
        pub a5: atmega_hal::port::PF5 = pf5,
        /// `A6`
        ///
        /// * `ADC6`: A/D converter input 6
        /// * `TDO`: JTAG test data output
        pub a6: atmega_hal::port::PF6 = pf6,
        /// `A7`
        ///
        /// * `ADC7`: A/D converter input 7
        /// * `TDI`: JTAG test data input
        pub a7: atmega_hal::port::PF7 = pf7,
        /// `A8`
        ///
        /// * `ADC8`: A/D converter input 8
        /// * `PCINT16`: External Interrupt (Pin Change)
        pub a8: atmega_hal::port::PK0 = pk0,
        /// `A9`
        ///
        /// * `ADC9`: A/D converter input 9
        /// * `PCINT17`: External Interrupt (Pin Change)
        pub a9: atmega_hal::port::PK1 = pk1,
        /// `A10`
        ///
        /// * `ADC10`: A/D converter input 10
        /// * `PCINT18`: External Interrupt (Pin Change)
        pub a10: atmega_hal::port::PK2 = pk2,
        /// `A11`
        ///
        /// * `ADC11`: A/D converter input 11
        /// * `PCINT19`: External Interrupt (Pin Change)
        pub a11: atmega_hal::port::PK3 = pk3,
        /// `A12`
        ///
        /// * `ADC12`: A/D converter input 12
        /// * `PCINT20`: External Interrupt (Pin Change)
        pub a12: atmega_hal::port::PK4 = pk4,
        /// `A13`
        ///
        /// * `ADC13`: A/D converter input 13
        /// * `PCINT21`: External Interrupt (Pin Change)
        pub a13: atmega_hal::port::PK5 = pk5,
        /// `A14`
        ///
        /// * `ADC14`: A/D converter input 14
        /// * `PCINT22`: External Interrupt (Pin Change)
        pub a14: atmega_hal::port::PK6 = pk6,
        /// `A15`
        ///
        /// * `ADC15`: A/D converter input 15
        /// * `PCINT23`: External Interrupt (Pin Change)
        pub a15: atmega_hal::port::PK7 = pk7,
    }
}
