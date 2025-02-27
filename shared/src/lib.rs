#![no_std]

use core::{fmt::Debug, future::Future};

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};

pub trait Backlight {
    fn on(&mut self);
    fn off(&mut self);
}

pub trait VibrationMotor {
    fn start(&mut self);
    fn stop(&mut self);
}

pub trait Buzzer {
    fn set_frequency(&mut self, frequency: u16);
    fn set_volume(&mut self, volume: u8);
    fn mute(&mut self);
    fn unmute(&mut self);
}

pub enum ButtonEvent {
    Up,
    Down,
}

pub trait PowerButton {
    fn is_pressed(&self) -> bool;
    fn event(&mut self) -> impl core::future::Future<Output = ButtonEvent> + core::marker::Send;
}
pub trait Rtc {
    fn timestamp(&mut self) -> i64;
    fn set_timestamp(&mut self, timestamp: i64);
}

pub enum Key {
    Select,
    Cancel,
    Up,
    Down,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Asterisk,
    Zero,
    Hash,
}

pub enum KeyEvent {
    Up(Key),
    Down(Key),
}

pub trait Keypad {
    fn event(&mut self) -> impl core::future::Future<Output = KeyEvent> + core::marker::Send;
}

pub trait Application {
    // should record:
    // how long this takes
    // how long between calls
    fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        vibration_motor: &mut impl VibrationMotor,
        buzzer: &mut impl Buzzer,
        display: &mut D,
        keypad: &mut impl Keypad,
        rtc: &mut impl Rtc,
        backlight: &mut impl Backlight,
        // NB. placeholder () here are for very different purposes
        // system_response could be a collection of system events that have happened since last run
        // call
        system_response: Option<Result<SystemRequest, ()>>,
    ) -> impl Future<Output = Result<Option<SystemRequest>, ()>>
    where
        <D as DrawTarget>::Error: Debug;
}

pub enum SystemRequest {}
