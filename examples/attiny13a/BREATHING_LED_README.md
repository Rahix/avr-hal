# ATtiny13A Breathing LED Examples

This directory contains two examples demonstrating PWM-based breathing LED effects on the ATtiny13A microcontroller using pin PB1.

## Examples

### 1. attiny13a-breathing-led.rs
A basic breathing LED example using a simple linear fade in/out pattern.

**Features:**
- Uses Timer0 PWM on pin PB1
- Linear brightness ramping (0-255)
- Configurable step size and delay
- Simple and memory-efficient

### 2. attiny13a-breathing-led-smooth.rs
An advanced breathing LED example using a sine wave lookup table for a more natural breathing effect.

**Features:**
- Uses Timer0 PWM on pin PB1
- Sine wave-based brightness curve with 128-step resolution
- Smoother, more natural breathing pattern
- Precomputed lookup table traversed bidirectionally for efficiency

### 3. attiny13a-dual-pwm.rs
A comprehensive example demonstrating both Timer0 PWM outputs with different patterns.

**Features:**
- Uses both Timer0 PWM pins (PB0 and PB1)
- LED1 (PB0): Sawtooth wave pattern
- LED2 (PB1): Sine wave breathing pattern
- Independent control of both PWM channels
- Demonstrates practical multi-LED PWM applications

## Hardware Setup

### Required Components:
- ATtiny13A microcontroller
- LED
- 220Ω - 1kΩ resistor (current limiting)
- Breadboard and jumper wires

### Connections:
```
ATtiny13A Pin 6 (PB1) -> Resistor -> LED Anode
LED Cathode -> GND
VCC -> ATtiny13A Pin 8
GND -> ATtiny13A Pin 4
```

### ATtiny13A Pinout:
```
    1 - PB5 (RST)    VCC - 8
    2 - PB3 (ADC3)   PB2 - 7
    3 - PB4 (ADC2)   PB1 - 6  <- PWM Output
    4 - GND          PB0 - 5
```

## PWM Configuration

Both examples use Timer0 in Fast PWM mode with the following settings:
- **Timer**: TC0 (8-bit Timer/Counter0)
- **PWM Pin**: PB1 (OC0B)
- **Prescaler**: 64
- **PWM Frequency**: ~977 Hz (at 8MHz clock)
- **Duty Cycle Range**: 0-255 (8-bit)

## Building and Flashing

### Build the examples:
```bash
cargo build --bin attiny13a-breathing-led
cargo build --bin attiny13a-breathing-led-smooth
cargo build --bin attiny13a-dual-pwm
```

### Flash to ATtiny13A:
```bash
# Using avrdude (adjust programmer and port as needed)
avrdude -p attiny13a -c usbasp -U flash:w:target/avr-attiny13a/debug/attiny13a-breathing-led.elf:e

# Or using ravedude if available
cargo run --bin attiny13a-breathing-led
cargo run --bin attiny13a-dual-pwm
```

## Customization

### Adjusting Breathing Speed:
- **Basic example**: Modify the `delay_ms(20)` value and `step_size` variable
- **Smooth example**: Modify the `delay_ms(30)` value and `phase_step` variable
- **Dual PWM example**: Modify the `delay_ms(30)` value and step constants for each pattern

### Changing PWM Frequency:
Modify the prescaler in the `Timer0Pwm::new()` call:
- `Prescaler::Direct` - 62.5 kHz (8MHz clock)
- `Prescaler::Prescale8` - 7.81 kHz
- `Prescaler::Prescale64` - 977 Hz (default)
- `Prescaler::Prescale256` - 244 Hz
- `Prescaler::Prescale1024` - 61.0 Hz

### Using Different Pins:
The ATtiny13A Timer0 supports PWM on two pins:
- PB0 (OC0A) - Change `pins.pb1` to `pins.pb0`
- PB1 (OC0B) - Current configuration

## Technical Details

### Memory Usage:
- **Basic example**: ~200 bytes flash, minimal RAM
- **Smooth example**: ~500 bytes flash (includes 128-byte sine table), minimal RAM
- **Dual PWM example**: ~400 bytes flash (includes 32-byte sine table), minimal RAM

### Power Consumption:
- Active mode: ~1-2mA (depending on LED current)
- The microcontroller runs continuously; consider sleep modes for battery applications

### Clock Configuration:
Examples assume the default 8MHz internal oscillator. If using a different clock source, adjust the PWM frequency calculations accordingly.

## ATtiny13A Limitations and Capabilities

### Hardware Limitations:
- **Flash Memory**: 1KB (1024 bytes) - very limited code space
- **SRAM**: 64 bytes - extremely limited RAM
- **EEPROM**: 64 bytes - available but not implemented in this HAL
- **I/O Pins**: Only 6 I/O pins (PB0-PB5), with PB5 shared with RESET
- **Timers**: Only Timer0 (8-bit) - no Timer1 like larger ATtiny chips
- **Communication**: No dedicated UART, SPI, or I2C hardware
- **ADC**: No analog-to-digital converter
- **Clock**: Internal 9.6MHz RC oscillator (default: divided by 8 = 1.2MHz)

### Features NOT Available:
- Timer1 PWM (not present on ATtiny13A)
- SPI communication (no dedicated hardware)
- I2C communication (no dedicated hardware)
- UART/Serial communication (no dedicated hardware)
- ADC readings (no ADC peripheral)
- EEPROM access (not implemented in HAL)
- External interrupts on multiple pins
- Multiple timer instances

### What IS Available:
- ✅ GPIO on pins PB0-PB4 (PB5 is RESET unless configured otherwise)
- ✅ Timer0 PWM on pins PB0 and PB1
- ✅ Watchdog timer for resets and timeouts
- ✅ Basic delay functions (millisecond/microsecond timing)
- ✅ Digital input/output with internal pull-ups
- ✅ Sleep modes (if implemented via direct register access)

### Development Tips:
- **Code Size**: Keep programs small - 1KB fills up quickly
- **RAM Usage**: Minimize variables - only 64 bytes available
- **Pin Planning**: Carefully plan pin usage - only 5-6 pins available
- **Timing**: Default 1.2MHz clock affects timing calculations
- **Power**: Consider sleep modes for battery applications
- **Debugging**: Limited debugging options - use LED indicators

## Troubleshooting

### LED not lighting:
- Check wiring connections
- Verify LED polarity (anode to resistor, cathode to GND)
- Ensure resistor value is appropriate (220Ω - 1kΩ)
- Confirm pin PB1 is correctly configured as output

### No breathing effect:
- Check if PWM is enabled (`led.enable()` is called)
- Verify Timer0 initialization
- Test with a simple blink example first
- Ensure Timer0 prescaler is set correctly

### Compilation errors:
- Ensure you're in the correct directory
- Check that the `attiny13a` feature is enabled in Cargo.toml
- Verify all dependencies are available
- Check that avr-none target is installed: `rustup target add avr-unknown-none`

### Memory errors:
- Code too large: Optimize for size, remove unused features
- Data too large (`section '.bss' is not within region 'data'`): consider using
  the `avr_progmem` crate
- Stack overflow: Reduce local variables, use static allocation
- Use `cargo size` to check memory usage

### Timing issues:
- Delays incorrect: Verify clock speed assumptions (1MHz vs 1.2MHz default)
- PWM frequency wrong: Check prescaler settings and clock configuration
- Consider using calibrated internal oscillator for better accuracy
