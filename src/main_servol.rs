//! ## Classical obstacle avoiding robot with Rust
//! This project takes and Arduino classic and tries to port it to
//! Rust as a beginner project.
//! It uses avr-hal crate for abstraction.
//! For hardware it uses a simple HC-SR04 sensor, an L298N motor driver and a SG90
//! servo motor (details in the Readme).
//! The sensor is reading distance every > 100ms and the robot should take appropriate
//! action if an obstacle is detected.

// Macros to inform rust that the project will not use
// main and the standard library (lib std adds a layer to build the usual functions.)
#![no_std]
#![no_main]


// Pull in the panic handler from panic-halt
extern crate panic_halt;

use arduino_hal::hal::port::mode::Floating;
use arduino_hal::prelude::*;


use crate::servo::ServoUnit;
use arduino_hal::simple_pwm::*;
use embedded_hal::serial::Read;
use core::marker::PhantomData;

//

mod servo;

const WAIT_BETWEEN_ACTIONS: u16 = 1000u16;
const MINIMAL_DISTANCE: u16 = 10u16;
const ACCEPTABLE_DISTANCE: u16 = 10u16;
// creates the main function
// attribute macro -> transforms the next as the entry point
// "!" is a never type. It informs nothing should return from the main function.
#[arduino_hal::entry]
fn main() -> ! {

    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    let mut delay = arduino_hal::Delay::new();

    let timer1 = dp.TC1;
    timer1.tccr1b.write(|w| w.cs1().prescale_64());

    let mut timer2 = arduino_hal::simple_pwm::Timer2Pwm::new(dp.TC2, arduino_hal::simple_pwm::Prescaler::Prescale1024);
    let mut pd3 = pins.d3.into_output().into_pwm(&timer2);
    pd3.enable();
    let mut servo_unit = ServoUnit{
        servo: pd3,
    };


    'outer: loop {
        ufmt::uwriteln!( & mut serial, "Hello！！!\r").void_unwrap();
        delay.delay_ms(WAIT_BETWEEN_ACTIONS);
        servo_unit.look_front();
        delay.delay_ms(WAIT_BETWEEN_ACTIONS);
        servo_unit.look_right();
        delay.delay_ms(WAIT_BETWEEN_ACTIONS);
        servo_unit.look_left();
        // I honestly forgot why I print that twice...
    }
}
