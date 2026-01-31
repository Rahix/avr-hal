//! This example demonstrates how to use the MCU's internal timer/counters
//! to move a servo motor.
//! 
//! NOTE: using timer/counters will not put any load to the CPU, but will limit the amount of
//! servo's you can control with a single board.
//! 
//! # Servo control
//!
//! * 20 ms PWM period required = 50 Hz
//! * Control range: 0.5 to 2.5 ms for a 0 to 180 degrees rotation.
//!
//! If we use the standard simple_pwm, we'll use an 8-bit TC.
//! We'd need a prescaler equal to factor 320 000 to get a 50 Hz PWM cycle.
//! 
//! The longest PWM cycle per prescaler is defined by MAX value (2^16) = 65c536
//! Options:
//!     * Prescale8: 16 MHz / 8 = 2 MHz => 0.5 us per clock tick * 65 5363 = 32.8 ms period max.
//!         --> 20 ms is achieved after 39 999 clock ticks.
//!     * Prescale64: 16 MHz / 64 = 250 Khz => 4us per clock tick * 65 536 = 262 ms perios max.
//!         --> 20 ms is achieved after 4 999 clock ticks.
//! This gives the following possible resolutions:
//! * Prescale8: 0.5 ms - 2.5 ms = 2 ms control range = 4000 clock ticks => 0.045 degrees per step.
//! * Prescale64: 0.5 - 2.5 ms = 500 clock ticks => 0.36 degrees per step.
//! 
//! For most cases Prescale64 will be sufficient.
//! 
//! For details on the timer/counter registers, refer to the ATMega2560 docs.
#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main () -> ! {
   let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Take Timer/Counter3 (TC3) from Peripherals
    let tc3 = dp.TC3;
    // Reset all TC3 registers A, B and C. These registers define the behaviour of the TC.
    tc3.tccr3a().reset();
    tc3.tccr3b().reset();
    tc3.tccr3c().reset();

    // // Compare mode set-up in register TCCR1A
    // // - bits 7:6 - COM1A1:0 = Output compare mode for OC1A (for channel A)
    // // - bits 5:4 - COM1B1:0 = Output compare mode for OC1B (for channel B)
    // // - bits 3:2 - COM1C1:0 = Output compare mode for OC1C (for channel C)
    // Here will use pin D3, which is port PE5. PE5 uses TC3 with output compare channel A.
    
    // Wave generation mode 14: WGM bits 0:3 1110 => FastPWM with ICRn to define TOP and OCRNX to define compare output.
    // WGM bits 0:1 go into TCCR3A (0b10)
    // WGM bits 2:3 go into TCCR3B (0b11)
    
    tc3  // In TC3
        .tccr3a() // Access register TCCR3A (TCCnx in the datasheet)
        .write(|w| unsafe{   // write to the register. This is unsafe as there's no protection against concurrent writing.
            w // Writer wrapper
                .com3a().bits(0b10) // Write bits 0b10 to the com3a bits in register TCCR3A 
                .wgm3().bits(0b10)} // Write WGM bits 2:3 into
        );
    // Same operation for TCCR3B.
    tc3.tccr3b().write(|w| unsafe{ 
        w
        .wgm3().bits(0b11) // Set WGM bits 2:3 
        .cs3().prescale_64()} // Set clock select bits. Using the avr-hal pre-defined prescale bits.
    );
    // We don't need TCCR3C for the servo.

    // TOP will define the length of the total PWM cycle.
    // Setting IRC3 to 4999 (now TOP for this TC) to achieve 50 Hz cycle. 
    // IMPORTANT: This affects all channels (A, B and C) of Timer/Counter3!
    tc3.icr3().write(|w| w.set(4999u16));
    
    // Setting duty cycle on channel C using OCR3A. When the TC3 count register hits this value,
    // Output Compare 3 will be toggled. It will invert the pin from high to low (due to how we set the COM3A bits)
    // * 0.5 ms = 125 ticks (0 - 124)
    // * 2.0 ms = 500 ticks (0 - 499)
    // * 2.5 ms = 625 ticks (0 - 624)
    // Servo SG90 0 degrees with Prescale8 = 124
    // Servo SG90 180 degrees with Prescale8 = 624
    tc3.ocr3a().write(|w| w.set(124u16)); // Now we set it to 0 degrees.
    
    // Toggle pin D3 into an output pin connect it to the TC3 output.
    // Connect your servo with its PWM pin to your board's D3 pin.
    // VCC to 5V and GND to GND offcourse.
    pins.d3.into_output(); 
    
    loop {
        for ticks in [124u16, 374u16, 624u16, 374u16] {
            tc3.ocr3a().write(|w| w.set(ticks)); // Now we set it to 0 degrees.
            arduino_hal::delay_ms(500);
        }
    }
}