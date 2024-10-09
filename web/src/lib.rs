#![feature(ascii_char)]
#![feature(ascii_char_variants)]

mod buzzer;
mod display;
mod keypad;
mod stub;

use embassy_executor::Spawner;
use app::text_input::{Model, TextInput};
use embedded_graphics::{
    mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor
};
use multi_tap::MultiTap;
use embedded_graphics::Drawable;

#[derive(PartialEq, Clone)]
pub struct Button(app::keypad::Button);

impl From<Button> for core::ascii::Char {
    fn from(button: Button) -> Self { 
        match button.0 {
            app::keypad::Button::Cancel => core::ascii::Char::CapitalA,
            app::keypad::Button::Select => core::ascii::Char::CapitalA,
            app::keypad::Button::Up => core::ascii::Char::CapitalA,
            app::keypad::Button::Down => core::ascii::Char::CapitalA,
            app::keypad::Button::One => core::ascii::Char::CapitalA,
            app::keypad::Button::Two => core::ascii::Char::CapitalA,
            app::keypad::Button::Three => core::ascii::Char::CapitalD,
            app::keypad::Button::Four => core::ascii::Char::CapitalG,
            app::keypad::Button::Five => core::ascii::Char::CapitalJ,
            app::keypad::Button::Six => core::ascii::Char::CapitalM,
            app::keypad::Button::Seven => core::ascii::Char::CapitalP,
            app::keypad::Button::Eight => core::ascii::Char::CapitalT,
            app::keypad::Button::Nine => core::ascii::Char::CapitalA,
            app::keypad::Button::Asterisk => core::ascii::Char::CapitalA,
            app::keypad::Button::Zero => core::ascii::Char::CapitalA,
            app::keypad::Button::Hash => core::ascii::Char::CapitalA,
        }
    }
}

impl multi_tap::Keypad for keypad::DomKeypad {
    type Button = Button;

    async fn event(&mut self) -> multi_tap::keypad::Event<<Self as multi_tap::Keypad>::Button> {
        match <Self as app::keypad::Keypad>::event(self).await {
            app::keypad::Event::Down(b) => multi_tap::keypad::Event::Down(Button(b)),
            app::keypad::Event::Up(b) => multi_tap::keypad::Event::Up(Button(b)),
        }
        
    }
}

#[embassy_executor::task]
async fn ticker() {
    let keypad = keypad::DomKeypad::new(
        "cancel", "select", "up", "down", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine", "asterisk", "zero", "hash",
    );
    let mut multi_tap = MultiTap::new(keypad);
    let mut buffer: [Option<multi_tap::Event>; 80] = [Default::default(); 80];
    let mut model = Model::new(&mut buffer);
    let mut text_input = TextInput::new(
        &mut model,
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build(),
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build(),
    );

    let mut display = display::Display::new();
    loop {
        // multi_tap.event().await
        // text_input.update(multi_tap::keypad::Event::Down( app::keypad::Button::Two));
        let _ = text_input.draw(&mut display);

        // otp.process().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());
    spawner.spawn(ticker()).unwrap();
}
