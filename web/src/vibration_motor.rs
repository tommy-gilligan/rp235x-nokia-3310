use shared::VibrationMotor;
use web_sys::Element;

pub struct Motor(Element);

impl Motor {
    pub fn new(element: Element) -> Self {
        Self(element)
    }
}

impl VibrationMotor for Motor {
    fn start(&mut self) {
        self.0.class_list().add_1("vibrating").unwrap();
    }

    fn stop(&mut self) {
        self.0.class_list().remove_1("vibrating").unwrap();
    }
}
