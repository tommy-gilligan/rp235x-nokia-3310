#![no_std]

use core::future::Future;

use embassy_time::Timer;

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
    fn run(
        vibration_motor: impl VibrationMotor,
        buzzer: impl Buzzer,
        rtc: impl Rtc,
    ) -> impl Future<Output = ()>;
}

pub struct Beepy;

impl Application for Beepy {
    async fn run(
        mut vibration_motor: impl VibrationMotor,
        mut buzzer: impl Buzzer,
        _rtc: impl Rtc,
    ) {
        loop {
            buzzer.unmute();
            // buzzer.set_volume(50);
            buzzer.set_frequency(440);
            vibration_motor.start();
            Timer::after_millis(1000).await;

            buzzer.set_frequency(880);
            vibration_motor.stop();
            Timer::after_millis(1000).await;
        }
    }
}
