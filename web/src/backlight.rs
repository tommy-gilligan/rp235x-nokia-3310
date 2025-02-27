use shared::Backlight;
use web_sys::Element;

pub struct Light(Element);

impl Light {
    pub fn new(element: Element) -> Self {
        Self(element)
    }
}

impl Backlight for Light {
    fn on(&mut self) {
        self.0.set_attribute("style", "color: lime").unwrap();
    }

    fn off(&mut self) {
        self.0.set_attribute("style", "color: black").unwrap();
    }
}
