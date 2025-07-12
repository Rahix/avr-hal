//! # ATtiny13A Breathing LED Example
//!
//! This example demonstrates a "breathing" LED effect using Timer0 PWM on pin PB1.
//! The LED gradually fades in and out using a linear brightness ramp.
//!
//! ## Hardware Setup
//! - Connect LED anode to ATtiny13A pin 6 (PB1) through a 220Ω-1kΩ resistor
//! - Connect LED cathode to GND
//! - Connect VCC to pin 8 and GND to pin 4
//!
//! ## PWM Configuration
//! - Timer: TC0 (8-bit Timer/Counter0)
//! - PWM Pin: PB1 (OC0B)
//! - Prescaler: 64 (configurable)
//! - PWM Frequency: ~977 Hz @ 8MHz clock
//! - Duty Cycle Range: 0-255 (8-bit)
//!
//! ## Timer0 PWM Configuration Details
//! - **Mode**: Phase-correct PWM (WGM0 = 01)
//! - **Resolution**: 8-bit (0-255 duty cycle range)
//! - **Frequency calculation**: F_PWM = F_CPU / (Prescaler × 510)
//!   - At 1.2MHz with Prescale64: 1,200,000 / (64 × 510) ≈ 37 Hz
//!   - At 8MHz with Prescale64: 8,000,000 / (64 × 510) ≈ 245 Hz
//!   - At 9.6MHz with Prescale64: 9,600,000 / (64 × 510) ≈ 294 Hz
//!
//! ## Available Prescaler Options
//! - `Prescaler::Direct` - No prescaling (F_CPU / 510)
//! - `Prescaler::Prescale8` - Divide by 8
//! - `Prescaler::Prescale64` - Divide by 64 (recommended)
//! - `Prescaler::Prescale256` - Divide by 256
//! - `Prescaler::Prescale1024` - Divide by 1024
//!
//! ## Customization
//! - Adjust `STEP_SIZE` to change fade smoothness (1-255)
//! - Adjust `DELAY_MS` to change breathing speed
//! - Change `PRESCALER` to modify PWM frequency
//! - Use PB0 instead of PB1 by changing pin initialization

#![no_std]
#![no_main]

use attiny_hal::pac::Peripherals;
use attiny_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm};
use panic_halt as _;

// Configuration constants - adjust these to customize the breathing effect
const STEP_SIZE: u8 = 2; // Brightness change per step (1-255)
const DELAY_MS: u32 = 20; // Delay between steps in milliseconds
const PRESCALER: Prescaler = Prescaler::Prescale64; // PWM frequency control

#[attiny_hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = attiny_hal::pins!(dp);

    // Initialize Timer0 for PWM (Phase-correct mode)
    // PWM frequency = CLK_io / (Prescaler * 510) for phase-correct PWM
    // With default 1.2MHz clock and Prescale64: 1.2MHz / (64 * 510) ≈ 37 Hz
    // With 8MHz clock and Prescale64: 8MHz / (64 * 510) ≈ 245 Hz
    let mut timer0 = Timer0Pwm::new(dp.TC0, PRESCALER);

    // Configure PB1 as PWM output (OC0B)
    // Alternative: Use PB0 for OC0A by changing to pins.pb0
    // ATtiny13A Timer0 PWM pins: PB0 (OC0A), PB1 (OC0B)
    let mut led = pins.pb1.into_output().into_pwm(&mut timer0);

    // Enable PWM output on the pin
    led.enable();

    // Breathing effect state variables
    let mut brightness: u8 = 0;
    let mut direction: i8 = 1; // 1 for increasing, -1 for decreasing

    loop {
        // Set the PWM duty cycle (0 = off, 255 = full brightness)
        led.set_duty(brightness);

        // Calculate next brightness value with bounds checking
        if direction > 0 {
            // Fading in
            if brightness >= 255 - STEP_SIZE {
                brightness = 255;
                direction = -1; // Start fading out
            } else {
                brightness = brightness.saturating_add(STEP_SIZE);
            }
        } else {
            // Fading out
            if brightness <= STEP_SIZE {
                brightness = 0;
                direction = 1; // Start fading in
            } else {
                brightness = brightness.saturating_sub(STEP_SIZE);
            }
        }

        // Control breathing speed
        // Smaller values = faster breathing, larger values = slower breathing
        attiny_hal::delay_ms(DELAY_MS);
    }
}
