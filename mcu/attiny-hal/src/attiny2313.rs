pub mod eeprom {
    pub use crate::periphals::eeprom::*;

    avr_hal_generic::impl_eeprom_attiny! {
        hal: crate::Attiny,
        peripheral: crate::pac::EEPROM,
        capacity: 128,
        addr_width: u8,
        set_address: |peripheral, address| {
            peripheral.eear.write(|w| w.bits(address));
        },
    }
}

#[macro_export]
macro_rules! pins {
    ($p:expr) => {
        $crate::Pins::new($p.PORTA, $p.PORTB, $p.PORTD)
    };
}

pub mod port {
    pub use crate::periphals::port::*;

    avr_hal_generic::impl_port_traditional! {
        enum Ports {
            A: crate::pac::PORTA = [0, 1, 2],
            B: crate::pac::PORTB = [0, 1, 2, 3, 4, 5, 6, 7],
            D: crate::pac::PORTD = [0, 1, 2, 3, 4, 5, 6],
        }
    }
}

pub mod simple_pwm {
    pub use crate::periphals::simple_pwm::*;

    // Fixme: Implement PWM for ATtiny2313.
}

pub mod spi {
    pub use crate::periphals::spi::*;

    // Fixme: Implement SPI for ATtiny2313.
}

pub mod wdt {
    pub use crate::periphals::wdt::*;

    avr_hal_generic::impl_wdt! {
        hal: crate::Attiny,
        peripheral: crate::pac::WDT,
        mcusr: crate::pac::cpu::MCUSR,
        wdtcsr_name: wdtcr,
        timeout: |to, w| match to {
            Timeout::Ms16 => w.wdpl().cycles_2k_512k(),
            Timeout::Ms32 => w.wdpl().cycles_4k_1024k(),
            Timeout::Ms64 => w.wdpl().cycles_8k(),
            Timeout::Ms125 => w.wdpl().cycles_16k(),
            Timeout::Ms250 => w.wdpl().cycles_32k(),
            Timeout::Ms500 => w.wdpl().cycles_64k(),
            Timeout::Ms1000 => w.wdpl().cycles_128k(),
            Timeout::Ms2000 => w.wdpl().cycles_256k(),
            Timeout::Ms4000 => w.wdph().set_bit().wdpl().cycles_2k_512k(),
            Timeout::Ms8000 => w.wdph().set_bit().wdpl().cycles_4k_1024k(),
        },
    }
}
