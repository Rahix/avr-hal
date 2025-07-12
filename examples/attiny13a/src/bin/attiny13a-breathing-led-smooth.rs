//! # ATtiny13A Smooth Breathing LED Example
//!
//! This example demonstrates a smooth "breathing" LED effect using Timer0 PWM on pin PB1.
//! The LED gradually fades in and out using a sine wave lookup table for a natural breathing pattern.
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
//! ## Features
//! - Uses precomputed sine wave lookup table for smooth transitions
//! - Full sine wave table (128 bytes) traversed bidirectionally
//! - Natural breathing rhythm similar to human breathing
//! - Configurable breathing speed and PWM frequency
//!
//! ## Memory Usage
//! - **Program Memory**: ~500-600 bytes (includes 128-byte sine table)
//! - **RAM Usage**: Minimal - only a few bytes for variables
//! - **Sine Table**: 128 bytes stored in program memory (PROGMEM)
//!
//! ## Customization
//! - Adjust `DELAY_MS` to fine-tune breathing speed (16-100ms recommended)
//! - Change `PRESCALER` to modify PWM frequency
//! - Modify `SINE_TABLE` for different breathing curves
//! - Use PB0 instead of PB1 by changing pin initialization
//! - Adjust table size (smaller tables save memory but reduce smoothness)

#![no_std]
#![no_main]

use attiny_hal::pac::Peripherals;
use attiny_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm};
use avr_progmem::progmem;
use panic_halt as _;

// Configuration constants - adjust these to customize the breathing effect
const DELAY_MS: u32 = 32; // Delay between steps in milliseconds
const PRESCALER: Prescaler = Prescaler::Prescale64; // PWM frequency control

// ATtiny13A has limited RAM (64 bytes), so we store the sine table in program memory.
progmem! {
static progmem SINE_TABLE: [u8; 128] = [
    255, 255, 255, 255, 254, 254, 254, 253,
    253, 252, 251, 250, 250, 249, 248, 246,
    245, 244, 243, 241, 240, 238, 237, 235,
    234, 232, 230, 228, 226, 224, 222, 220,
    218, 215, 213, 211, 208, 206, 203, 201,
    198, 196, 193, 190, 188, 185, 182, 179,
    176, 173, 170, 167, 165, 162, 158, 155,
    152, 149, 146, 143, 140, 137, 134, 131,
    128, 124, 121, 118, 115, 112, 109, 106,
    103, 100, 97, 93, 90, 88, 85, 82,
    79, 76, 73, 70, 67, 65, 62, 59,
    57, 54, 52, 49, 47, 44, 42, 40,
    37, 35, 33, 31, 29, 27, 25, 23,
    21, 20, 18, 17, 15, 14, 12, 11,
    10, 9, 7, 6, 5, 5, 4, 3,
    2, 2, 1, 1, 1, 0, 0, 0
];
}

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
    led.enable();

    // Breathing effect state variables
    // Start at index 1 to avoid boundary condition on first iteration
    // phase: current position in sine table (0-127)
    // phase_direction: true = ascending through table, false = descending
    let mut phase = 1u8;
    let mut phase_direction = true; // true = ascending, false = descending

    loop {
        // Set LED brightness from sine table
        led.set_duty(SINE_TABLE.load_at(phase as usize));

        // Reverse direction when reaching table boundaries
        if phase as usize == SINE_TABLE.len() - 1 || phase == 0 {
            phase_direction = !phase_direction;
        }

        // Update phase index based on current direction
        phase = if phase_direction {
            phase.saturating_add(1)
        } else {
            phase.saturating_sub(1)
        };

        attiny_hal::delay_ms(DELAY_MS);
    }
}
