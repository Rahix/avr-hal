/*!
 * Simple example for direct bit manipulation in registers of an Arduino Uno.
 * Compiled size of this example is 212 bytes as of writing.
 *
 * The code is an equivalent of this normal version, which compiles to 220 bytes.
 *
 * ```rust
 * fn main() -> ! {
 *      let dp = arduino_hal::Peripherals::take().unwrap();
 *      let pins = arduino_hal::pins!(dp);
 *
 *      let mut led = pins.d8.into_output();
 *
 *      let button = pins.d2.into_pull_up_input();
 *
 *      loop {
 *          if button.is_low() {
 *              led.set_high();
 *          } else {
 *              led.set_low();
 *          }
 *      }
 * }
 * ```
 *
 * There's no huge savings in terms of program size by doing direct bit manipulation on the
 * hardware for this example. As the build system is already quite capable of optimizing for size.
 * What we end up saving is mere 8 bytes.
 *
 * This example is intended to show how register manipulation is done with avr-hal to be a
 * quickstart reference.
 *
 * Connections (Uno)
 * -----------
 *   - `d8`: Standard LED wired series with a 330 ohm current limiting resistor
 *   - `d2`: Standard push button pulling the pin to ground to enable it (active low)
 *
 *   Pressing the button should turn on the LED.
 *   Releasing the button should turn the LED off.
 */

#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    // No pins!
    //let pins = arduino_hal::pins!(dp);

    // Set digital 8 pin of Uno or PB0 in MCU's terminology to output by manipulating
    // the data direction register for port B (DDRB) with PB0 pin bit set to 1.
    // PB0 stands for Port B, pin zero.
    // Resulting byte value in the register will be 1 or 0b_0000_0001 in binary format.
    // Because we change the rightmost or least significant bit of this
    // byte to configure pin 0 of port B.
    dp.PORTB.ddrb.write(|w| w.pb0().set_bit());

    // Set digital 2 pin of Uno to input pullup by manipulating PORTD register.
    // DDRD (data direction register for port D) is already set as input or 0 for all pins by default.
    // So we don't modify it. We modify PORTD register's bit for PD2 to turn on the input pull-up resistor.
    // PORTx registers have dual function. If the pin is configured as output in DDRx register,
    // they output a digital high signal.
    // But for input pins (which is default) it's function is enabling the internal pull-up resistor.
    dp.PORTD.portd.write(|w| w.pd2().set_bit());

    // We start the main loop of the program. Which shouldn't return any value.
    // The return type of a "loop {}" which doesn't return a value is !, or the never type.
    // This fact is matched by the "fn main() -> !" signature of the main function.
    // This is effectively an infinite loop, that can only be disrupted by a reset, or an
    // interrupt.
    loop {
        // The PIND register holds values of port D input pins.
        // Port D pin 2 is where our button is connected as an input pullup.
        // We expect the PD2 pin's bit in this register to be false if the button is pressed.
        // Because this pin is configured as input pull-up above.
        // In other words it's an active low pin. It is on when pulled down to ground.
        if dp.PORTD.pind.read().pd2().bit() == false {
            // Button is pressed.
            // Write 1 to the PB0 (d8 Uno) pin's bit in PORTB register to light-up the LED.
            // PORTx registers work differently for input and output pins.
            // If the pin is input, it will enable it's internal pull-up resistor to convert it
            // into a active low pin.
            // If the pin is output, it will output a digital high signal which is the case here.
            dp.PORTB.portb.write(|w| w.pb0().set_bit());
        } else {
            // Button is released.
            // Clear the bit value to 0 of PB0 (d8 Uno) pin's bit in PORTB register to turn off the LED.
            dp.PORTB.portb.write(|w| w.pb0().clear_bit());
        }
    }
}
