use crate::port::portd;

crate::avr_hal::impl_usart! {
    pub struct Usart1 {
        peripheral: crate::atmega32u4::USART1,
        pins: {
            rx: portd::PD2,
            tx: portd::PD3,
        },
        registers: {
            control_a: ucsr1a {
                data_empty: udre1,
                recv_complete: rxc1,
            },
            control_b: ucsr1b {
                tx_enable: txen1,
                rx_enable: rxen1,
            },
            control_c: ucsr1c {
                mode: umsel1,
                char_size: ucsz1,
                stop_bits: usbs1,
                parity: upm1,
            },
            baud: ubrr1,
            data: udr1,
        },
    }
}
