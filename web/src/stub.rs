use core::ascii::Char;
use embassy_time::Timer;
use multi_tap::*;

#[derive(PartialEq, Clone)]
pub enum Button {
    A,
}

impl From<Button> for Char {
    fn from(_: Button) -> Char {
        Char::Space
    }
}

pub struct Stub;

impl Keypad for Stub {
    type Button = Button;

    async fn event(&mut self) -> multi_tap::keypad::Event<Button> {
        Timer::after_secs(1).await;
        return multi_tap::keypad::Event::Down(Button::A);
    }
}
