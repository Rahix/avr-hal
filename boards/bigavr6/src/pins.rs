use atmega1280_hal::port::PortExt;

avr_hal_generic::impl_board_pins! {
    #[port_defs]
    use atmega1280_hal::port;

    /// Generic DDR that works for all ports
    pub struct DDR {
        portb: crate::atmega1280::PORTA,
        portb: crate::atmega1280::PORTB,
        portc: crate::atmega1280::PORTC,
        portd: crate::atmega1280::PORTD,
        porte: crate::atmega1280::PORTE,
        porte: crate::atmega1280::PORTF,
        porte: crate::atmega1280::PORTG,
        porte: crate::atmega1280::PORTH,
        porte: crate::atmega1280::PORTJ,
        porte: crate::atmega1280::PORTK,
        porte: crate::atmega1280::PORTL,
    }

    pub struct Pins {
        /// `D0` / `RX`
        ///
        /// * `RX` (UART)
        /// * `INT2`: External Interrupt
        pub d0: portd::pd2::PD2,
    }
}
