//! Board Support Crate for _Arduino Leonardo_.
//!
//! This crate provides abstractions for interfacing with the hardware of Arduino Leonardo.  It
//! re-exports functionality from the underlying HAL in ways that make more sense for this
//! particular board.  For example, the pins are named by what is printed on the PCB instead of the
//! MCU names.
//!
//! # Examples
//! A number of examples can be found in the [`examples/`][ex] subdirectory of this crate.
//!
//! [ex]: https://github.com/Rahix/avr-hal/tree/master/boards/arduino-leonardo/examples
//!
//! # Getting Started
//! Please follow the guide from [`avr-hal`'s README][guide] for steps on how to set up a project
//! with this board.  A rough skeleton for an application looks like this:
//!
//! ```no_run
//! #![no_std]
//! #![no_main]
//!
//! // Pull in the panic handler from panic-halt
//! extern crate panic_halt;
//!
//! // The prelude just exports all HAL traits anonymously which makes
//! // all trait methods available.  This is probably something that
//! // should always be added.
//! use arduino_leonardo::prelude::*;
//!
//! // Define the entry-point for the application.  This can only be
//! // done once in the entire dependency tree.
//! #[arduino_leonardo::entry]
//! fn main() -> ! {
//!     // Get the peripheral singletons for interacting with them.
//!     let dp = arduino_leonardo::Peripherals::take().unwrap();
//!
//!     unimplemented!()
//! }
//! ```
//!
//! [guide]: https://github.com/Rahix/avr-hal#starting-your-own-project

#![no_std]

// Expose hal & pac crates
pub use atmega32u4_hal as hal;
pub use crate::hal::pac;

/// See [`avr_device::entry`](https://docs.rs/avr-device/latest/avr_device/attr.entry.html).
#[cfg(feature = "rt")]
pub use crate::hal::entry;

pub use crate::pac::Peripherals;

mod pins;
pub use crate::pins::*;

pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use crate::hal::usart::BaudrateExt as _;
}
pub use crate::hal::usart;

/// Busy-Delay
///
/// **Note**: For just delaying, using [`arduino_leonardo::delay_ms()`][delay_ms] or
/// [`arduino_leonardo::delay_us()`][delay_us] is probably the better choice.  This type is more
/// useful when an `embedded-hal` driver needs a delay implementation.
///
/// [delay_ms]: fn.delay_ms.html
/// [delay_us]: fn.delay_us.html
pub type Delay = hal::delay::Delay<hal::clock::MHz16>;

/// Wait (busy spin) for `ms` milliseconds
pub fn delay_ms(ms: u16) {
    use prelude::*;

    Delay::new().delay_ms(ms)
}

/// Wait (busy spin) for `us` microseconds
pub fn delay_us(us: u16) {
    use prelude::*;

    Delay::new().delay_us(us)
}

/// Support for the Serial Peripheral Interface
///
/// # Example
/// For a full example, see [`examples/leonardo-spi-feedback.rs`][ex-spi].  In short:
/// ```no_run
/// let dp = arduino_leonardo::Peripherals::take().unwrap();
///
/// let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);
///
/// // Create SPI interface.
/// let mut spi = arduino_leonardo::spi::Spi::new(
///     dp.SPI,
///     pins.sck.into_output(&mut pins.ddr),
///     pins.mosi.into_output(&mut pins.ddr),
///     pins.miso.into_pull_up_input(&mut pins.ddr),
///     pins.led_rx.into_output(&mut pins.ddr),
///     arduino_leonardo::spi::Settings::default(),
/// );
/// ```
///
/// [ex-spi]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-leonardo/examples/leonardo-spi-feedback.rs
pub mod spi {
    pub use atmega32u4_hal::spi::*;
}

/// Support for the Analog to Digital Converter
///
/// # Example
/// For a full example, see [`examples/leonardo-adc.rs`][ex-adc].  In short:
/// ```no_run
/// let dp = arduino_leonardo::Peripherals::take().unwrap();
///
/// let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);
///
/// let mut adc = arduino_leonardo::adc::Adc::new(dp.ADC, Default::default());
///
/// let v_bandgap: u16 = nb::block!(
///     adc.read(&mut arduino_leonardo::adc::channel::Vbg)
/// ).void_unwrap(),
///
/// let portf = dp.PORTF.split();
/// let mut a0 = portf.pf7.into_analog_input(&mut adc);
/// let mut a1 = portf.pf6.into_analog_input(&mut adc);
/// let mut a2 = portf.pf5.into_analog_input(&mut adc);
/// let mut a3 = portf.pf4.into_analog_input(&mut adc);
/// let mut a4 = portf.pf1.into_analog_input(&mut adc);
/// let mut a5 = portf.pf0.into_analog_input(&mut adc);
///
/// let v: u16 = nb::block!(adc.read(&mut a0)).void_unwrap(),
/// ```
///
/// [ex-adc]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-leonardo/examples/leonardo-adc.rs
pub mod adc {
    pub use atmega32u4_hal::adc::*;
}

/// Support for PWM pins
///
/// The 4 timers of ATmega32U4 can be used for PWM on certain pins.
/// The PWM methods are from `embedded_hal::PwmPin`.
///
/// # Example
/// For a full example, see [`examples/leonardo-pwm.rs`][ex-pwm].  In short:
/// ```
/// let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);
/// let mut timer1 = arduino_leonardo::pwm::Timer1Pwm::new(
///     dp.TC1,
///     arduino_leonardo::pwm::Prescaler::Prescale64,
/// );
///
/// let mut d3 = pins.d3.into_output(&mut pins.ddr).into_pwm(&mut timer0);
///
/// d3.set_duty(128);
/// d3.enable();
/// ```
///
/// [ex-pwm]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-leonardo/examples/leonardo-pwm.rs
///
/// Here is an overview of pins and which timer they work with:
///
/// | Pin | Conversion Method | Alternate Conversion Method |
/// | --- | --- | --- |
/// | `pins.d3` | `.into_pwm(&mut timer0)` | |
/// | `pins.d5` | `.into_pwm(&mut timer3)` | |
/// | `pins.d6` | `.into_pwm(&mut timer4)` | |
/// | `pins.d9` | `.into_pwm(&mut timer1)` | |
/// | `pins.d10` | `.into_pwm(&mut timer1)` | `.into_pwm4(&mut timer4)` |
/// | `pins.d11` | `.into_pwm(&mut timer0)` | `.into_pwm1(&mut timer1)` |
/// | `pins.d13` | `.into_pwm(&mut timer4)` | |
pub mod pwm {
    pub use atmega32u4_hal::pwm::*;
}

/// Serial (UART) interface on pins `D0` (RX) and `D1` (TX)
///
/// # Example
/// For a full example, see [`examples/leonardo-serial.rs`][ex-serial].  In short:
/// ```no_run
/// let dp = arduino_leonardo::Peripherals::take().unwrap();
///
/// let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);
///
/// let mut serial = arduino_leonardo::Serial::new(
///     dp.USART1,
///     pins.d0,
///     pins.d1.into_output(&mut pins.ddr),
///     57600.into_baudrate(),
/// );
///
/// ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();
/// ```
///
/// [ex-serial]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-leonardo/examples/leonardo-serial.rs
pub type Serial<IMODE> = hal::usart::Usart1<hal::clock::MHz16, IMODE>;

/// I2C Master on pins `D2` (SDA) and `D3` (SCL)
///
/// # Example
/// For a full example, see [`examples/leonardo-i2cdetect.rs`][ex-i2c].  In short:
/// ```no_run
/// let dp = arduino_leonardo::Peripherals::take().unwrap();
///
/// let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE);
///
/// let mut i2c = arduino_leonardo::I2cMaster::new(
///     dp.TWI,
///     pins.d2.into_pull_up_input(&mut pins.ddr),
///     pins.d3.into_pull_up_input(&mut pins.ddr),
///     50000,
/// );
/// ```
///
/// [ex-i2c]: https://github.com/Rahix/avr-hal/blob/master/boards/arduino-leonardo/examples/leonardo-i2cdetect.rs
pub type I2cMaster<M> = hal::i2c::I2cMaster<hal::clock::MHz16, M>;
#[doc(hidden)]
#[deprecated = "Please use `I2cMaster` instead of `I2c`"]
pub type I2c<M> = I2cMaster<M>;

/// Support for the Watchdog Timer
///
/// # Note
/// Changing the watchdog configuration requires two separate writes to WDTCSR where the second
/// write must occur within 4 cycles of the first or the configuration will not change. You may need
/// to adjust optimization settings to prevent other operations from being emitted between these two
/// writes.
///
/// # Example
/// ```
/// let mut watchdog = arduino_leonardo::wdt::Wdt::new(&dp.CPU.mcusr, dp.WDT);
/// watchdog.start(arduino_leonardo::wdt::Timeout::Ms8000);
///
/// loop {
///     watchdog.feed();
/// }
/// ```
pub mod wdt {
    pub use atmega32u4_hal::wdt::*;
}
