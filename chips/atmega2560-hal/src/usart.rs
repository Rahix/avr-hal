use crate::port::porte;
use crate::port::portd;
use crate::port::porth;
use crate::port::portj;

crate::avr_hal::impl_usart! {
    pub struct Usart0 {
        peripheral: crate::atmega2560::USART0,
        pins: {
            rx: porte::PE0,
            tx: porte::PE1,
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

crate::avr_hal::impl_usart! {
    pub struct Usart1 {
        peripheral: crate::atmega2560::USART1,
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

crate::avr_hal::impl_usart! {
    pub struct Usart2 {
        peripheral: crate::atmega2560::USART2,
        pins: {
            rx: porth::PH0,
            tx: porth::PH1,
        },
        registers: {
            control_a: ucsr2a {
                data_empty: udre2,
                recv_complete: rxc2,
            },
            control_b: ucsr2b {
                tx_enable: txen2,
                rx_enable: rxen2,
            },
            control_c: ucsr2c {
                mode: umsel2,
                char_size: ucsz2,
                stop_bits: usbs2,
                parity: upm2,
            },
            baud: ubrr2,
            data: udr2,
        },
    }
}

crate::avr_hal::impl_usart! {
    pub struct Usart3 {
        peripheral: crate::atmega2560::USART3,
        pins: {
            rx: portj::PJ0,
            tx: portj::PJ1,
        },
        registers: {
            control_a: ucsr3a {
                data_empty: udre3,
                recv_complete: rxc3,
            },
            control_b: ucsr3b {
                tx_enable: txen3,
                rx_enable: rxen3,
            },
            control_c: ucsr3c {
                mode: umsel3,
                char_size: ucsz3,
                stop_bits: usbs3,
                parity: upm3,
            },
            baud: ubrr3,
            data: udr3,
        },
    }
}

