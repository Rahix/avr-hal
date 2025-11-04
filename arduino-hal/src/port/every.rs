pub use atxmega_hal::port::{mode, Pin};

avr_hal_generic::renamed_pins! {
    /// Pins of the **Arduino Nano Every**.
    ///
    /// This struct is best initialized via the [`arduino_hal::pins!()`][crate::pins] macro.
    pub struct Pins {
        /// `A0`
        ///
        /// * AIN3 (ADC input pin 3)
        /// * INT27: External Interrupt
        pub a0: atxmega_hal::port::PD3 = pd3,
        /// `A1`
        ///
        /// * AIN2 (ADC input pin 2)
        /// * INT26: External Interrupt
        pub a1: atxmega_hal::port::PD2 = pd2,
        /// `A2`
        ///
        /// * AIN1 (ADC input channel 2)
        /// * INT25: External Interrupt
        pub a2: atxmega_hal::port::PD1 = pd1,
        /// `A3`
        ///
        /// * AIN0 (ADC input channel 0)
        /// * INT24: External Interrupt
        pub a3: atxmega_hal::port::PD0 = pd0,
        /// `A4`
        ///
        /// * AIN12 (ADC input channel 12)
        /// * SDA (2-wire serial bus data input/output line)
        /// * INT2: External Interrupt
        pub a4: atxmega_hal::port::PF2 = pf2,
        /// `A5`
        ///
        /// * AIN13 (ADC input channel 13)
        /// * SCL (2-wire serial bus clock line)
        /// * INT3: External Interrupt
        pub a5: atxmega_hal::port::PF3 = pf3,
        /// `A6`
        ///
        /// * AIN4 (ADC input channel 5)
        /// * INT28: External Interrupt
        pub a6: atxmega_hal::port::PD4 = pd4,
        /// `A7`
        ///
        /// * AIN5 (ADC input channel 5)
        /// * INT29: External Interrupt
        pub a7: atxmega_hal::port::PD5 = pd5,
        /// `D0` / `RX`
        ///
        /// * RXD (USART input pin)
        /// * INT20: External Interrupt
        pub d0: atxmega_hal::port::PC5 = pc5,
        /// `D1` / `TX`
        ///
        /// * TXD (USART output pin)
        /// * INT21: External Interrupt
        pub d1: atxmega_hal::port::PC4 = pc4,
        /// `D2`
        ///
        /// * INT0: External Interrupt
        pub d2: atxmega_hal::port::PA0 = pa0,
        /// `D3`
        ///
        /// * AIN15 (analog comparator positive input)
        /// * INT45: External Interrupt
        pub d3: atxmega_hal::port::PF5 = pf5,
        /// `D4`
        ///
        /// * INT22: External Interrupt
        pub d4: atxmega_hal::port::PC6 = pc6,
        /// `D5`
        ///
        /// * **PWM**:
        /// * INT22: External Interrupt
        pub d5: atxmega_hal::port::PB2 = pb2,
        /// `D6`
        ///
        /// * **PWM**
        /// * AIN14 (analog comparator positive input)
        /// * INT44: External Interrupt
        pub d6: atxmega_hal::port::PF4 = pf4,
        /// `D7`
        ///
        /// * INT1: External Interrupt
        pub d7: atxmega_hal::port::PA1 = pa1,
        /// `D8`
        ///
        /// * INT35: External Interrupt
        pub d8: atxmega_hal::port::PE3 = pe3,
        /// `D9`
        ///
        /// * **PWM**
        /// * INT9: External Interrupt
        pub d9: atxmega_hal::port::PB0 = pb0,
        /// `D10`
        ///
        /// * **PWM**
        /// * INT10: External Interrupt
        pub d10: atxmega_hal::port::PB1 = pb1,
        /// `D11`
        ///
        /// * **PWM**
        /// * INT32: External Interrupt
        pub d11: atxmega_hal::port::PE0 = pe0,
        /// `D12`
        ///
        /// * INT33: External Interrupt
        pub d12: atxmega_hal::port::PE1 = pe1,
        /// `D13`
        ///
        /// * SCK (SPI bus master clock input)
        /// * INT34: External Interrupt
        /// * L LED on Arduino Uno
        pub d13: atxmega_hal::port::PE2 = pe2,

        pub rx : atxmega_hal::port::PB5= pb5,
        pub tx : atxmega_hal::port::PB4 = pb4,
    }

    impl Pins {
        type Pin = Pin;
        type McuPins = atxmega_hal::Pins;
    }
}
