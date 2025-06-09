//! Port
//!
//! # Example
//!
//! Complete example source code can be found in the repository:
//! [`atmega2560-blink.rs`](https://github.com/Rahix/avr-hal/blob/main/examples/atmega2560/src/bin/atmega2560-blink.rs)
//!
//! ```
//! let dp = avrmodern_hal::Peripherals::take().unwrap();
//! let pins = avrmodern_hal::pins!(dp);
//!
//! let mut led = pins.pb7.into_output();
//!
//! loop {
//!     led.toggle();
//!     delay_ms(1000);
//! }
//! ```

pub use avr_hal_generic::port::{mode, PinMode, PinOps};

#[cfg(feature = "attiny402")]
avr_hal_generic::impl_port_new! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3, 6, 7],
    },
    input
}

#[cfg(any(
    feature = "attiny1614",
    //feature = "attiny3224",
))]
avr_hal_generic::impl_port_new! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
        B: crate::pac::PORTB = [0, 1, 2, 3],
    },
    in_
}

/*
#[cfg(any(feature = "avr128db28"))]
avr_hal_generic::impl_port_new! {
    enum Ports {
        A: crate::pac::PORTA = [0, 1, 2, 3, 4, 5, 6, 7],
        C: crate::pac::PORTC = [0, 1, 2, 3],
        D: crate::pac::PORTD = [1, 2, 3, 4, 5, 6, 7],
        F: crate::pac::PORTF = [0, 1, 6],
    }
}
*/
