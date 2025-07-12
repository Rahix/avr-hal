//! # ATtiny13A Dual PWM Example
//!
//! This example demonstrates using both PWM outputs from Timer0 on the ATtiny13A.
//! It shows how to control two LEDs independently with different PWM patterns.
//!
//! ## Hardware Setup
//! - Connect LED1 anode to ATtiny13A pin 5 (PB0/OC0A) through a 220Ω-1kΩ resistor
//! - Connect LED2 anode to ATtiny13A pin 6 (PB1/OC0B) through a 220Ω-1kΩ resistor
//! - Connect both LED cathodes to GND
//! - Connect VCC to pin 8 and GND to pin 4
//!
//! ## PWM Configuration
//! - Timer: TC0 (8-bit Timer/Counter0)
//! - PWM Pins: PB0 (OC0A) and PB1 (OC0B)
//! - Mode: Phase-correct PWM
//! - Prescaler: 64 (configurable)
//! - PWM Frequency: F_CPU / (Prescaler × 510)
//!   - At 1.2MHz: ~37 Hz
//!   - At 8MHz: ~245 Hz
//!   - At 9.6MHz: ~294 Hz
//! - Duty Cycle Range: 0-255 (8-bit)
//!
//! ## ATtiny13A Pinout Reference
//! ```
//!     1 - PB5 (RST)    VCC - 8
//!     2 - PB3 (ADC3)   PB2 - 7
//!     3 - PB4 (ADC2)   PB1 - 6  <- OC0B (PWM Output)
//!     4 - GND          PB0 - 5  <- OC0A (PWM Output)
//! ```
//!
//! ## Behavior
//! - LED1 (PB0): Sawtooth wave pattern - brightness increases linearly then drops
//! - LED2 (PB1): Sine wave pattern - smooth breathing effect
//! - Both LEDs operate independently with different timing
//!
//! ## Customization
//! - Adjust `SAWTOOTH_STEP` and `SINE_STEP` to change pattern speeds
//! - Change `DELAY_MS` to modify overall animation speed
//! - Modify `PRESCALER` to change PWM frequency
//! - Swap patterns between LEDs by changing the assignments

#![no_std]
#![no_main]

use attiny_hal::pac::Peripherals;
use attiny_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm};
use panic_halt as _;

// Configuration constants
const DELAY_MS: u32 = 30; // Animation update interval
const SAWTOOTH_STEP: u8 = 4; // Sawtooth brightness increment per step
const SINE_STEP: u8 = 8; // Sine wave phase increment per step
const PRESCALER: Prescaler = Prescaler::Prescale64;

// Compact sine table for smooth breathing effect (32 entries covering quarter wave)
// Full sine wave reconstructed by mirroring and inverting
const SINE_TABLE: [u8; 32] = [
    128, 152, 176, 198, 218, 234, 245, 252, 255, 252, 245, 234, 218, 198, 176, 152, 128, 103, 79,
    57, 37, 21, 10, 3, 0, 3, 10, 21, 37, 57, 79, 103,
];

#[attiny_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);

    // Initialize Timer0 for PWM (Phase-correct mode)
    // Both OC0A and OC0B will use the same timer but can have independent duty cycles
    let mut timer0 = Timer0Pwm::new(dp.TC0, PRESCALER);

    // Configure both PWM outputs
    // PB0 = OC0A (Timer0 Compare Output A)
    // PB1 = OC0B (Timer0 Compare Output B)
    let mut led1_sawtooth = pins.pb0.into_output().into_pwm(&mut timer0); // Sawtooth pattern
    let mut led2_sine = pins.pb1.into_output().into_pwm(&mut timer0); // Sine pattern

    // Enable PWM on both pins
    led1_sawtooth.enable();
    led2_sine.enable();

    // Animation state variables
    let mut sawtooth_brightness: u8 = 0; // Current brightness for sawtooth LED
    let mut sine_phase: u8 = 0; // Current phase index for sine LED

    loop {
        // Update LED1 with sawtooth pattern
        led1_sawtooth.set_duty(sawtooth_brightness);

        // Increment sawtooth brightness, wrapping at overflow
        sawtooth_brightness = sawtooth_brightness.saturating_add(SAWTOOTH_STEP);
        if sawtooth_brightness >= 255 - SAWTOOTH_STEP {
            sawtooth_brightness = 0; // Reset to create sawtooth effect
        }

        // Update LED2 with sine wave pattern
        let sine_index = (sine_phase >> 3) & 31; // Use upper bits, mask to table size
        led2_sine.set_duty(SINE_TABLE[sine_index as usize]);

        // Increment sine phase for smooth progression
        sine_phase = sine_phase.wrapping_add(SINE_STEP);

        // Control animation speed
        attiny_hal::delay_ms(DELAY_MS);
    }
}
