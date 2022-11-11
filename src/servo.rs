const SERVO_CENTER: u8 = 23;
const SERVO_RIGHT: u8 = 15;
const SERVO_LEFT: u8 = 31;

use arduino_hal::hal::port::*;
use arduino_hal::prelude::*;
use arduino_hal::port::*;
use arduino_hal::simple_pwm::*;
use core::marker::PhantomData;

/// We use a generic for the pin
// pub struct ServoUnit<S: embedded_hal::PwmPin<Duty = u8>> {
//     pub servo: S,
// }

// pub struct ServoUnit<S: PwmPinOps<TC>,TC> {
//     pub servo: S,
//     pub(crate) _p: PhantomData<TC>
// }
//
//
//
// /// We implement embedded_hal::PwmPin for the struct ServoUnit,
// /// with rotations as methods and not lost functions
// impl<S: PwmPinOps<TC>,TC> ServoUnit<S,TC> {
//     pub fn look_right(&mut self) {
//         self.servo.set_duty(SERVO_RIGHT);
//     }
//     pub fn look_left(&mut self) {
//         self.servo.set_duty(SERVO_LEFT);
//     }
//     pub fn look_front(&mut self) {
//         self.servo.set_duty(SERVO_CENTER);
//     }
// }


pub struct ServoUnit<TC,PIN:arduino_hal::simple_pwm::PwmPinOps<TC>>{
    pub servo: arduino_hal::hal::port::Pin<mode::PwmOutput<TC>, PIN>,
}


/// We implement embedded_hal::PwmPin for the struct ServoUnit,
/// with rotations as methods and not lost functions
impl<TC,PIN:arduino_hal::simple_pwm::PwmPinOps<TC>> ServoUnit<TC,PIN>{
    pub fn look_right(&mut self) {
        self.servo.set_duty(SERVO_RIGHT);
    }
    pub fn look_left(&mut self) {
        self.servo.set_duty(SERVO_LEFT);

    }
    pub fn look_front(&mut self) {
        self.servo.set_duty(SERVO_CENTER);
    }
}
