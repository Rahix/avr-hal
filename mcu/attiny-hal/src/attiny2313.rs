use crate::r#impl::avr_hal;

avr_hal! {
    device: attiny2313,
    eeprom: {
        capacity: 128,
        addr_width: u8,
        addr_reg: eear,
    },
    port: {
        ports: {
            A: [0, 1, 2],
            B: [0, 1, 2, 3, 4, 5, 6, 7],
            D: [0, 1, 2, 3, 4, 5, 6],
        },
        impl!: avr_hal_generic::impl_port_traditional,
    },
    wdt: {
        wdtcsr_name: wdtcr,
    },
}
