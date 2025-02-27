#![no_std]

use core::fmt::Debug;

use embedded_graphics::{draw_target::DrawTarget, pixelcolor::BinaryColor};
use shared::Application;

pub struct AppMenu<'a>(shared::menu::Menu<'a>);

const ITEMS: [&str; 3] = ["Apple", "Banana", "Carrot"];

impl Default for AppMenu<'_> {
    fn default() -> Self {
        Self(shared::menu::Menu::new(&ITEMS))
    }
}

impl Application for AppMenu<'_> {
    async fn run<D: DrawTarget<Color = BinaryColor>>(
        &mut self,
        _vibration_motor: &mut impl shared::VibrationMotor,
        _buzzer: &mut impl shared::Buzzer,
        display: &mut D,
        keypad: &mut impl shared::Keypad,
        _rtc: &mut impl shared::Rtc,
        _backlight: &mut impl shared::Backlight,
        _system_response: Option<Result<shared::SystemRequest, ()>>,
    ) -> Result<Option<shared::SystemRequest>, ()>
    where
        <D as DrawTarget>::Error: Debug,
    {
        let _i = loop {
            if let Some(index) = self.0.process(keypad, display).await {
                break index;
            }
        };

        Ok(None)
    }
}
