/*!
 * Sweep a standard SG90 compatible servo from its left limit all the way to its right limit.
 *
 * Because avr-hal does not have a dedicated servo driver library yet, we do this manually using
 * timer TC1.  The servo should be connected to D9 (AND D9 ONLY!  THIS DOES NOT WORK ON OTHER PINS
 * AS IT IS).
 *
 * As the limits are not precisely defined, we undershoot the datasheets 1ms left limit and
 * overshoot the 2ms right limit by a bit - you can figure out where exactly the limits are for
 * your model by experimentation.
 *
 * Connections
 * -----------
 *  - `D9`: Servo's PWM signal
 */
#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Important because this sets the bit in the DDR register!
    pins.d9.into_output();

    // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
    // - Each count increases the duty-cycle by 4us.
    // - Use OC1A which is connected to D9 of the Arduino Uno.
    let tc1 = dp.TC1;
    tc1.icr1().write(|w| w.set(4999));
    tc1.tccr1a()
        .write(|w| w.wgm1().set(0b10).com1a().match_clear());
    tc1.tccr1b()
        .write(|w| w.wgm1().set(0b11).cs1().prescale_64());

    loop {
        // 100 counts => 0.4ms
        // 700 counts => 2.8ms
        for duty in 100..=700 {
            tc1.ocr1a().write(|w| w.set(duty));
            arduino_hal::delay_ms(20);
        }
    }
}
