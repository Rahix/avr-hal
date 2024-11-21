pub use avr_device::attiny2313 as pac;

pub struct Hal;

use crate::r#impl::*;

impl_mod_eeprom! {
    hal: crate::attiny2313,
    capacity: 128,
    addr_width: u8,
    addr_reg: eear,
}

impl_mod_port! {
    hal: crate::attiny2313,
    ports: {
        A: [0, 1, 2],
        B: [0, 1, 2, 3, 4, 5, 6, 7],
        D: [0, 1, 2, 3, 4, 5, 6],
    },
    impl!: avr_hal_generic::impl_port_traditional,
}

impl_mod_wdt! {
    hal: crate::attiny2313,
    wdtcsr_name: wdtcr,
}

