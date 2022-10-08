pub use atmega_hal::port::{mode, Pin, PinOps, PinMode};

avr_hal_generic::renamed_pins! {
    type Pin = Pin;

    /// Pins of the **Arduino Leonardo**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
    pub struct Pins from atmega_hal::Pins {
        /// `D0` / `RX`
        ///
        /// * `RX` (UART)
        /// * `INT2`: External Interrupt
        pub d0: atmega_hal::port::PD2 = pd2,
        /// `D1` / `TX`
        ///
        /// * `TX` (UART)
        /// * `INT3`: External Interrupt
        pub d1: atmega_hal::port::PD3 = pd3,
        /// `D2` / `SDA`
        ///
        /// * `SDA`: i2c/twi data
        /// * `INT1`: External Interrupt
        pub d2: atmega_hal::port::PD1 = pd1,
        /// `D3` / `SCL`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer0Pwm]
        /// * `SCL`: i2c/twi clock
        /// * `INT0`: External Interrupt
        /// * `OC0B`: Output Compare Channel `B` for Timer/Counter0
        pub d3: atmega_hal::port::PD0 = pd0,
        /// `D4`
        pub d4: atmega_hal::port::PD4 = pd4,
        /// `D5`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer3Pwm]
        /// * `OC3A`: Output Compare Channel `A` for Timer/Counter3
        /// * `#OC4A`: Inverted Output Compare Channel `A` for Timer/Counter4 (Not implemented)
        pub d5: atmega_hal::port::PC6 = pc6,
        /// `D6`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer4Pwm]
        /// * `OC4D`: Output Compare Channel `D` for Timer/Counter4
        pub d6: atmega_hal::port::PD7 = pd7,
        /// `D7`
        ///
        /// * `INT6`: External Interrupt
        pub d7: atmega_hal::port::PE6 = pe6,
        /// `D8`
        pub d8: atmega_hal::port::PB4 = pb4,
        /// `D9`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer1Pwm]
        /// * `OC1A`: Output Compare Channel `A` for Timer/Counter1
        /// * `#OC4B`: Inverted Output Compare Channel `B` for Timer/Counter4 (Not implemented)
        pub d9: atmega_hal::port::PB5 = pb5,
        /// `D10`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer1Pwm]
        /// * `OC1B`: Output Compare Channel `B` for Timer/Counter1
        /// * `OC4B`: Output Compare Channel `B` for Timer/Counter4 (Not implemented)
        pub d10: atmega_hal::port::PB6 = pb6,
        /// `D11`
        ///
        /// * **PWM**: [atmega32u4_hal::timer::Timer0Pwm]
        /// * `OC0A`: Output Compare Channel `B` for Timer/Counter0
        /// * `OC1C`: Output Compare Channel `C` for Timer/Counter1
        pub d11: atmega_hal::port::PB7 = pb7,
        /// `D12`
        ///
        /// * `#OC4D`: Inverted Output Compare Channel `D` for Timer/Counter4 (Not implemented)
        pub d12: atmega_hal::port::PD6 = pd6,
        /// `D13` / `LED_BUILTIN`
        ///
        /// * Onboard LED
        /// * **PWM**: [atmega32u4_hal::timer::Timer4Pwm]
        /// * `OC4A`: Output Compare Channel `A` for Timer/Counter4
        pub d13: atmega_hal::port::PC7 = pc7,
        /// `RX`
        ///
        /// Led for indicating inbound data.  Also the CS pin.
        pub led_rx: atmega_hal::port::PB0 = pb0,
        /// `TX`
        ///
        /// Led for indicating outbound data
        pub led_tx: atmega_hal::port::PD5 = pd5,
        /// `SCLK`
        ///
        /// ICSP SCLK pin
        pub sck: atmega_hal::port::PB1 = pb1,
        /// `MOSI`
        ///
        /// ICSP MOSI pin
        pub mosi: atmega_hal::port::PB2 = pb2,
        /// `MISO`
        ///
        /// ICSP MISO pin
        pub miso: atmega_hal::port::PB3 = pb3,
        /// `A0`
        ///
        /// * `ADC7` channel
        pub a0: atmega_hal::port::PF7 = pf7,
        /// `A1`
        ///
        /// * `ADC6` channel
        pub a1: atmega_hal::port::PF6 = pf6,
        /// `A2`
        ///
        /// * `ADC5` channel
        pub a2: atmega_hal::port::PF5 = pf5,
        /// `A3`
        ///
        /// * `ADC4` channel
        pub a3: atmega_hal::port::PF4 = pf4,
        /// `A4`
        ///
        /// * `ADC1` channel
        pub a4: atmega_hal::port::PF1 = pf1,
        /// `A5`
        ///
        /// * `ADC0` channel
        pub a5: atmega_hal::port::PF0 = pf0,
    }
}
