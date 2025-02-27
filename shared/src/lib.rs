#![no_std]

pub mod menu;

use core::{fmt::Debug, future::Future};

use embedded_graphics::{Drawable, prelude::Primitive, primitives::PrimitiveStyle};
use embedded_graphics_core::{draw_target::DrawTarget, pixelcolor::BinaryColor};

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
    fn was_pressed(&mut self) -> impl core::future::Future<Output = bool> + core::marker::Send;
}

pub trait Rtc {
    fn timestamp(&mut self) -> i64;
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
    #[allow(clippy::too_many_arguments)]
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

// decide your time budgets
// 'trust' application takes at most 750ms
// force pre-emption at 1500ms
// how do you progress things inside app that take longer than 750?
// special kind of timer?
// forced pre-emption should be signalled back to application + print log entry
#[allow(clippy::too_many_arguments)]
pub async fn run_app<D: DrawTarget<Color = BinaryColor>>(
    mut app: impl Application,
    vibration_motor: &mut impl VibrationMotor,
    buzzer: &mut impl Buzzer,
    display: &mut D,
    keypad: &mut impl Keypad,
    rtc: &mut impl Rtc,
    light: &mut impl Backlight,
    power: &mut impl PowerButton,
) where
    <D as DrawTarget>::Error: Debug,
{
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    display
        .bounding_box()
        .into_styled(fill)
        .draw(display)
        .unwrap();

    loop {
        match embassy_time::with_timeout(
            embassy_time::Duration::from_millis(1000),
            app.run(vibration_motor, buzzer, display, keypad, rtc, light, None),
        )
        .await
        {
            Ok(Ok(None)) => {}
            Err(embassy_time::TimeoutError) => {
                // println!("timed out");
            }
            _ => {
                unimplemented!()
            }
        }

        if power.was_pressed().await {
            return;
        }
    }
}
