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
        pub a0: porta::pa0::PA0,
        pub a1: porta::pa1::PA1,
        pub a2: porta::pa2::PA2,
        pub a3: porta::pa3::PA3,
        pub a4: porta::pa4::PA4,
        pub a5: porta::pa5::PA5,
        pub a6: porta::pa6::PA6,
        pub a7: porta::pa7::PA7,
        
        pub b0: portb::pb0::PB0,
        pub b1: portb::pb1::PB1,
        pub b2: portb::pb2::PB2,
        pub b3: portb::pb3::PB3,
        pub b4: portb::pb4::PB4,
        pub b5: portb::pb5::PB5,
        pub b6: portb::pb6::PB6,
        pub b7: portb::pb7::PB7,
        
        pub c0: portc::pc0::PC0,
        pub c1: portc::pc1::PC1,
        pub c2: portc::pc2::PC2,
        pub c3: portc::pc3::PC3,
        pub c4: portc::pc4::PC4,
        pub c5: portc::pc5::PC5,
        pub c6: portc::pc6::PC6,
        pub c7: portc::pc7::PC7,
        
        pub d0: portd::pd0::PD0,
        pub d1: portd::pd1::PD1,
        pub d2: portd::pd2::PD2,
        pub d3: portd::pd3::PD3,
        pub d4: portd::pd4::PD4,
        pub d5: portd::pd5::PD5,
        pub d6: portd::pd6::PD6,
        pub d7: portd::pd7::PD7,
        
        pub e0: porte::pe0::PE0,
        pub e1: porte::pe1::PE1,
        pub e2: porte::pe2::PE2,
        pub e3: porte::pe3::PE3,
        pub e4: porte::pe4::PE4,
        pub e5: porte::pe5::PE5,
        pub e6: porte::pe6::PE6,
        pub e7: porte::pe7::PE7,
        
        pub f0: portf::pf0::PF0,
        pub f1: portf::pf1::PF1,
        pub f2: portf::pf2::PF2,
        pub f3: portf::pf3::PF3,
        pub f4: portf::pf4::PF4,
        pub f5: portf::pf5::PF5,
        pub f6: portf::pf6::PF6,
        pub f7: portf::pf7::PF7,
        
        pub g0: portg::pg0::PG0,
        pub g1: portg::pg1::PG1,
        pub g2: portg::pg2::PG2,
        pub g3: portg::pg3::PG3,
        pub g4: portg::pg4::PG4,
        pub g5: portg::pg5::PG5,
        
        pub h0: porth::ph0::PH0,
        pub h1: porth::ph1::PH1,
        pub h2: porth::ph2::PH2,
        pub h3: porth::ph3::PH3,
        pub h4: porth::ph4::PH4,
        pub h5: porth::ph5::PH5,
        pub h6: porth::ph6::PH6,
        pub h7: porth::ph7::PH7,
        
        pub j0: portj::pj0::PJ0,
        pub j1: portj::pj1::PJ1,
        pub j2: portj::pj2::PJ2,
        pub j3: portj::pj3::PJ3,
        pub j4: portj::pj4::PJ4,
        pub j5: portj::pj5::PJ5,
        pub j6: portj::pj6::PJ6,
        pub j7: portj::pj7::PJ7,
        
        pub k0: portk::pk0::PK0,
        pub k1: portk::pk1::PK1,
        pub k2: portk::pk2::PK2,
        pub k3: portk::pk3::PK3,
        pub k4: portk::pk4::PK4,
        pub k5: portk::pk5::PK5,
        pub k6: portk::pk6::PK6,
        pub k7: portk::pk7::PK7,
        
        pub l0: portl::pl0::PL0,
        pub l1: portl::pl1::PL1,
        pub l2: portl::pl2::PL2,
        pub l3: portl::pl3::PL3,
        pub l4: portl::pl4::PL4,
        pub l5: portl::pl5::PL5,
        pub l6: portl::pl6::PL6,
        pub l7: portl::pl7::PL7,
    }
}
