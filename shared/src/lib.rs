#![no_std]

use core::{fmt::Debug, future::Future};

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle},
};

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
    #[cfg(feature = "chrono")]
    fn now(&mut self) -> chrono::DateTime<chrono::Utc> {
        chrono::DateTime::<chrono::Utc>::from_timestamp(self.timestamp(), 0).unwrap()
    }

    #[cfg(feature = "chrono")]
    fn set_now(&mut self, now: chrono::DateTime<chrono::Utc>) {
        self.set_timestamp(now.timestamp())
    }

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
    fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        vibration_motor: &mut impl VibrationMotor,
        buzzer: &mut impl Buzzer,
        display: &mut D,
        keypad: &mut impl Keypad,
        rtc: &mut impl Rtc,
        // NB. placeholder () here are for very different purposes
        system_response: Option<Result<SystemRequest, ()>>,
    ) -> impl Future<Output = Result<Option<SystemRequest>, ()>>
    where
        <D as DrawTarget>::Error: Debug;
}

pub enum SystemRequest {}

pub struct Beepy(i32);

impl Beepy {
    pub fn new(yoffset: i32) -> Self {
        Self(yoffset)
    }
}

impl Default for Beepy {
    fn default() -> Self {
        Self::new(10)
    }
}

impl Application for Beepy {
    async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        vibration_motor: &mut impl VibrationMotor,
        buzzer: &mut impl Buzzer,
        display: &mut D,
        keypad: &mut impl Keypad,
        _rtc: &mut impl Rtc,
        _system_response: Option<Result<SystemRequest, ()>>,
    ) -> Result<Option<SystemRequest>, ()>
    where
        <D as DrawTarget>::Error: Debug,
    {
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::Off, 1);
        let border_stroke = PrimitiveStyleBuilder::new()
            .stroke_color(BinaryColor::Off)
            .stroke_width(3)
            .stroke_alignment(StrokeAlignment::Inside)
            .build();
        let fill = PrimitiveStyle::with_fill(BinaryColor::Off);

        display
            .bounding_box()
            .into_styled(border_stroke)
            .draw(display)
            .unwrap();

        match keypad.event().await {
            KeyEvent::Down(Key::Down) => {
                self.0 -= 1;
            }
            KeyEvent::Down(Key::Up) => {
                self.0 += 1;
            }
            KeyEvent::Down(Key::One) => {
                buzzer.unmute();
            }
            KeyEvent::Down(Key::Two) => {
                buzzer.mute();
            }
            KeyEvent::Down(Key::Four) => {
                buzzer.set_frequency(440);
            }
            KeyEvent::Down(Key::Five) => {
                buzzer.set_frequency(660);
            }
            KeyEvent::Down(Key::Six) => {
                buzzer.set_frequency(880);
            }
            KeyEvent::Down(Key::Eight) => {
                vibration_motor.start();
            }
            KeyEvent::Down(Key::Seven) => {
                vibration_motor.stop();
            }
            _ => {}
        }

        Triangle::new(
            Point::new(16, 16 + self.0),
            Point::new(16 + 16, 16 + self.0),
            Point::new(16 + 8, self.0),
        )
        .into_styled(thin_stroke)
        .draw(display)
        .unwrap();

        Rectangle::new(Point::new(52, self.0), Size::new(16, 16))
            .into_styled(fill)
            .draw(display)
            .unwrap();

        Ok(None)
    }
}
