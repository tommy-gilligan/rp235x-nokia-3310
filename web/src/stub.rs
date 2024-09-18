use app::keypad::Keypad;
use embassy_time::Timer;

pub struct Stub;

impl Keypad for Stub {
    async fn event(&mut self) -> app::keypad::Event<app::keypad::Button> {
        Timer::after_secs(1).await;
        app::keypad::Event::Down(app::keypad::Button::Down)
    }
}
