use embassy_rp::{
    gpio::{Input, Pull},
    peripherals::PIN_28,
};
use shared::PowerButton;

pub struct Button<'a>(Input<'a>);

impl Button<'_> {
    pub fn new(pin: PIN_28) -> Self {
        Self(Input::new(pin, Pull::Up))
    }
}

impl PowerButton for Button<'_> {
    async fn was_pressed(&mut self) -> bool {
        self.0.is_low()
    }
}
