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


use crate::motors::{go_backward, go_forward, stop, turn_left, turn_right};
use crate::sensor::{return_distance, SensorUnit};
use crate::servo::ServoUnit;
use arduino_hal::simple_pwm::*;
use embedded_hal::serial::Read;
use core::marker::PhantomData;

//
mod motors;
mod sensor;
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

    let mut sensor_unit = SensorUnit {
        trig: pins.d12.into_output(),
        // floating input is set by default so we can configure echo without ddr
        echo: pins.d11,
        timer: timer1,
    };

    // downgrading the pins allow to put them in an array and simplify functions:
    // according to docs : Downgrade this pin into a type that is generic over all pins.
    let left_forw = pins.d4.into_output().downgrade();
    let left_back = pins.d5.into_output().downgrade();
    let right_forw = pins.d6.into_output().downgrade();
    let right_back = pins.d7.into_output().downgrade();

    // we have now mutable wheels that can be sent to motor functions
    let mut wheels = [left_forw, left_back, right_forw, right_back];
    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    'outer: loop {
        servo_unit.look_front();
        go_forward(&mut wheels);

        let value = return_distance(&mut sensor_unit);
        ufmt::uwriteln!( & mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();

        if value < MINIMAL_DISTANCE {
            // the 'obstacle_avoidance loop. I would like to name it, but the compiler will complain :)
            loop {
                stop(&mut wheels);

                servo_unit.look_right();
                let value_right = return_distance(&mut sensor_unit);
                ufmt::uwriteln!( & mut serial, "On right, we are {} cms away from target!\r", value).void_unwrap();

                delay.delay_ms(WAIT_BETWEEN_ACTIONS);

                servo_unit.look_left();
                let value_left = return_distance(&mut sensor_unit);
                ufmt::uwriteln!( & mut serial, "On left, we are {} cms away from target!\r", value).void_unwrap();

                delay.delay_ms(WAIT_BETWEEN_ACTIONS);

                if (value_left > value_right) && value_left > ACCEPTABLE_DISTANCE {
                    turn_left(&mut wheels);
                } else if (value_right > value_left) && value_right > ACCEPTABLE_DISTANCE {
                    turn_right(&mut wheels);
                } else {
                    go_backward(&mut wheels);
                    delay.delay_ms(WAIT_BETWEEN_ACTIONS);
                    turn_right(&mut wheels);
                }
                continue 'outer;
            }
        }

        // the sensor needs to wait approximately 60 ms between two waves.
        // we ensure that by waiting while the register reaches 25000
        // one count == 4 us, and 4us*0.000004 == 100 ms
        while sensor_unit.timer.tcnt1.read().bits() < 25000 {}

        // I honestly forgot why I print that twice...
        ufmt::uwriteln!( & mut serial, "Hello, we are {} cms away from target!\r", value).void_unwrap();
    }

}
