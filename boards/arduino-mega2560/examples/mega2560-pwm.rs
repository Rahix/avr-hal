#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_mega2560::prelude::*;
use arduino_mega2560::pwm;

#[arduino_mega2560::entry]
fn main() -> ! {
    let dp = arduino_mega2560::Peripherals::take().unwrap();

    let mut delay = arduino_mega2560::Delay::new();

    let mut pins = arduino_mega2560::Pins::new(
        dp.PORTA,
        dp.PORTB,
        dp.PORTC,
        dp.PORTD,
        dp.PORTE,
        dp.PORTF,
        dp.PORTG,
        dp.PORTH,
        dp.PORTJ,
        dp.PORTK,
        dp.PORTL,
    );
    let mut timer0 = pwm::Timer0Pwm::new(dp.TC0, pwm::Prescaler::Prescale64);

    let mut pin = pins.d13.into_output(&mut pins.ddr).into_pwm(&mut timer0);

    pin.set_duty(128);
    pin.enable();
    
    loop {
        for i in 0..=255_u16 {
            let duty = i * i  / 256;
            pin.set_duty(duty as u8);
            delay.delay_ms(10_u16);
        }     
    }
}
