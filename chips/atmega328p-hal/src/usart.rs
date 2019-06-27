use crate::port::portd;

crate::avr_hal::impl_usart! {
    pub struct Usart0 {
        peripheral: crate::atmega328p::USART0,
        pins: {
            rx: portd::PD0,
            tx: portd::PD1,
        },
        registers: {
            control_a: ucsr0a {
                data_empty: udre0,
                recv_complete: rxc0,
            },
            control_b: ucsr0b {
                tx_enable: txen0,
                rx_enable: rxen0,
            },
            control_c: ucsr0c {
                mode: umsel0,
                char_size: ucsz0,
                stop_bits: usbs0,
                parity: upm0,
            },
            baud: ubrr0,
            data: udr0,
        },
    }
}
