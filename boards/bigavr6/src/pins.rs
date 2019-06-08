use atmega1280_hal::port::PortExt;

avr_hal_generic::impl_board_pins! {
    #[port_defs]
    use atmega1280_hal::port;

    /// Generic DDR that works for all ports
    pub struct DDR {
        porta: crate::atmega1280::PORTA,
        portb: crate::atmega1280::PORTB,
        portc: crate::atmega1280::PORTC,
        portd: crate::atmega1280::PORTD,
        porte: crate::atmega1280::PORTE,
        portf: crate::atmega1280::PORTF,
        portg: crate::atmega1280::PORTG,
        porth: crate::atmega1280::PORTH,
        portj: crate::atmega1280::PORTJ,
        portk: crate::atmega1280::PORTK,
        portl: crate::atmega1280::PORTL,
    }

    pub struct Pins {
        /// `D0` / `RX`
        ///
        /// * `RX` (UART)
        /// * `INT2`: External Interrupt
        pub d0: portd::pd0::PD0,
        pub d2: portd::pd2::PD2,
        pub d3: portd::pd3::PD3,
    }
}
