use embassy_rp::{
    gpio::{Input, Pull},
    peripherals::PIN_28,
};
use shared::{ButtonEvent, PowerButton};

pub struct Button<'a>(Input<'a>, bool);

impl Button<'_> {
    pub fn new(pin: PIN_28) -> Self {
        Self(Input::new(pin, Pull::Up), false)
    }
}

impl PowerButton for Button<'_> {
    fn is_pressed(&self) -> bool {
        self.0.is_low()
    }

    async fn event(&mut self) -> ButtonEvent {
        if self.1 {
            self.0.wait_for_high().await;
            self.1 = false;
            ButtonEvent::Up
        } else {
            self.0.wait_for_low().await;
            self.1 = true;
            ButtonEvent::Down
        }
    }
}
