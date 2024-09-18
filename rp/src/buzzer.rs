use embassy_rp::pwm::{Config, Pwm};
use fixed::traits::ToFixed;

pub struct Buzzer<'a>(Pwm<'a>, u32, u32);

impl<'a> Buzzer<'a> {
    pub fn new(pwm: Pwm<'a>) -> Self {
        Self(pwm, 90, 0)
    }

    fn update(&mut self) {
        if self.2 == 0 {
            let mut c: Config = Default::default();
            c.enable = false;
            self.0.set_config(&c);
        } else {
            let top: u32 = 1000000 / self.2 - 1;

            let mut c: Config = Default::default();
            c.divider = 125.0.to_fixed();
            c.top = top.try_into().unwrap();
            c.compare_a = ((top + 1) * self.1 / 100 - 1).try_into().unwrap();

            self.0.set_config(&c);
        }
    }
}

impl<'a> app::buzzer::Buzzer for Buzzer<'a> {
    fn enable(&mut self) {}

    fn disable(&mut self) {}

    fn set_frequency(&mut self, frequency: u32) {
        self.2 = frequency;
        self.update();
    }
}
