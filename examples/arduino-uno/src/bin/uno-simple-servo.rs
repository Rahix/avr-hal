/*
 * Example of a simple servo library for the Arduino Uno
 * This library uses the Timer1 interrupt to control the servos
 * The library is based on the Servo library for the Arduino
 * The library is not optimized and is for educational purposes
 * The library is not thread safe
 * The library is not tested
 * The library is not complete
 */

#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;
use arduino_hal::port::{mode::Output, Pin};
use core::ptr::addr_of_mut;

const MAX_SERVOS: usize = 8;
const REFRESH_INTERVAL: u16 = 20000; // 20ms
const DEFAULT_PULSE_WIDTH: u16 = 1500; // 1.5ms
const INTERRUPT_OVERHEAD: u16 = 4;
const MIN_PULSE_WIDTH: u16 = 544; // 0.544ms
const MAX_PULSE_WIDTH: u16 = 2400; // 2.4ms
static mut SERVO_ARRAY: [Option<*mut Servo>; MAX_SERVOS] = [None; MAX_SERVOS];
static mut CURRENT_SERVO_INDEX: usize = 0;
static mut CURRENT_OPERATIVE_SERVOS: usize = 0;

#[allow(dead_code)]
pub struct Servo{
   pin: Pin<Output>,
   pulse_width_ticks: u16,
   index: usize,
}

#[allow(dead_code)]
impl Servo {
  pub fn new(pin: Pin<Output>) -> Self {
        unsafe {
            if CURRENT_OPERATIVE_SERVOS > MAX_SERVOS {
                panic!("Too many servos");
            }

            let servo = Servo {
                index: CURRENT_OPERATIVE_SERVOS,
                pin,
                pulse_width_ticks: us_to_ticks(DEFAULT_PULSE_WIDTH),
            };

            CURRENT_OPERATIVE_SERVOS += 1;

            if CURRENT_OPERATIVE_SERVOS == 1 {
                // Enable the timer 1 interrupt
                let dp = arduino_hal::Peripherals::steal();
                let t1 = dp.TC1; // Get the timer 1
                set_clock(&t1);
            }

            servo
        }
    }

  pub fn attach(&mut self) {
      unsafe{
          SERVO_ARRAY[self.index] = Some(self as *mut Servo);
      }
  }

    pub fn set_pulse_width(&mut self, pulse_width: u16) {
        let mut pw = pulse_width;

        // Check if the pulse width is within the limits
        if pulse_width < MIN_PULSE_WIDTH {
            pw = MIN_PULSE_WIDTH;
        } else if pulse_width > MAX_PULSE_WIDTH {
            pw = MAX_PULSE_WIDTH;
        }

        // Trim interruption overhead compensation
        pw = pw - 2;
        let value = us_to_ticks(pw);

        avr_device::interrupt::free(|_|  self.pulse_width_ticks = value )
    }

    pub fn set_angle(&mut self, angle: u16) {
        let mut ang = angle;

        // Check if the angle is within the limits
        if angle > 180 {
            ang = 180;
        }

        // Convert the angle to pulse width
        let pulse_width = 544 + (ang * 11); // This is the formula for the pulse width
        self.set_pulse_width(pulse_width);
    }
}

fn us_to_ticks(us: u16) -> u16 {
    us * 2 // 16MHz / 8 = 2MHz => 1 tick = 0.5us
}

fn set_clock(t1: &arduino_hal::pac::TC1) {
    t1.tccr1a.write(|w| w.wgm1().bits(0b00));
    t1.tccr1b
        .write(|w| w.cs1()
            .prescale_8()
            .wgm1()
            .bits(0b01));
    t1.ocr1a.write(|w| w.bits(us_to_ticks(REFRESH_INTERVAL)));
    t1.timsk1.write(|w| w.ocie1a().set_bit());

    unsafe {
        avr_device::interrupt::enable();
    }

}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.d13.into_output();
    let mut servo_1 = Servo::new(pins.d8.into_output().downgrade());
    let mut servo_2 = Servo::new(pins.d9.into_output().downgrade());
    servo_1.attach(); // We attach the servo to the array
    servo_2.attach(); // Try to comment this line and see what happens

    loop {
        led.set_high();
        servo_1.set_angle(180);
        servo_2.set_angle(0);
        arduino_hal::delay_ms(1500);
        led.set_low();
        arduino_hal::delay_ms(500);
        led.set_high();
        servo_1.set_angle(90);
        servo_2.set_angle(90);
        arduino_hal::delay_ms(1500);
        led.set_low();
        arduino_hal::delay_ms(500);
        led.set_high();
        servo_1.set_angle(0);
        servo_2.set_angle(180);
        arduino_hal::delay_ms(1500);
        led.set_low();
        arduino_hal::delay_ms(500);
    }
}

#[avr_device::interrupt(atmega328p)]
fn TIMER1_COMPA() {
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let t1 = dp.TC1; // Get the timer 1

    // Get the current servo index
    let index = unsafe {addr_of_mut!(CURRENT_SERVO_INDEX)};

    // Check current servo
    // If it is the last one, reset the counter
    match unsafe { *index  >= CURRENT_OPERATIVE_SERVOS } {
        true => {
            // Reset the counter TCNT1
            t1.tcnt1.write(|w| w.bits(0b0));

            // Set the index to 0
            // To check the first servo in the array
            unsafe { *index = 0 } ;
        }
        false => {
            unsafe {
                if let Some(servo_ptr) = SERVO_ARRAY[*index] { // Get the servo pointer
                    let servo = &mut *servo_ptr;
                    servo.pin.set_low();
                }

                // Increment the index to check
                // The next servo in the array
                *index += 1;
            }
        }
    }

    // Check if we are at the end of the array
    match unsafe { *index < CURRENT_OPERATIVE_SERVOS } {
        true => {
            // Set next interrupt to current counter value
            // Plus the pulse width of the next servo
            // OCR1A = TCNT1 + SERVO_ARRAY[*index].pulse_width_ticks
            // And set the pin to high
            // SERVO_ARRAY[*index].pin.set_high();
            unsafe {
                if let Some(servo_ptr) = SERVO_ARRAY[*index] { // Get the servo pointer
                    let servo = &mut *servo_ptr;
                    let next_interrupt = t1.tcnt1.read().bits() + servo.pulse_width_ticks;
                    t1.ocr1a.write(|w| w.bits(next_interrupt));
                    servo.pin.set_high();
                }
            }
        }
        false => {
            // Check value of the current counter TCNT1 + interrupt overhead
            // If it is less than the value of the REFRESH_INTERVAL
            // Set next interrupt to REFRESH_INTERVAL
            // Else set next interrupt to TCNT1 + interrupt overhead
            // And set index >= CURRENT_OPERATIVE_SERVOS
            let mut next_interrupt = t1.tcnt1.read().bits() + INTERRUPT_OVERHEAD;
            if next_interrupt < us_to_ticks(REFRESH_INTERVAL) {
                next_interrupt = us_to_ticks(REFRESH_INTERVAL);
            } 

            t1.ocr1a.write(|w| w.bits(next_interrupt));
            unsafe { *index = CURRENT_OPERATIVE_SERVOS };
        }
    }
}
