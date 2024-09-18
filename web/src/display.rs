use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::Dimensions;
use embedded_graphics::prelude::DrawTarget;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Pixel;
use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};

pub struct Display(WebSimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor>);

impl Display {
    pub fn new() -> Self {
        let output_settings = OutputSettingsBuilder::new()
            .scale(2)
            .pixel_spacing(0)
            .alpha_color(embedded_graphics::pixelcolor::BinaryColor::On)
            .build();

        let display: WebSimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor> =
            WebSimulatorDisplay::new((84, 48), &output_settings, None);

        Self(display)
    }
}

impl DrawTarget for Display {
    type Color = BinaryColor;

    type Error = <WebSimulatorDisplay<BinaryColor> as DrawTarget>::Error;

    fn draw_iter<I: IntoIterator<Item = Pixel<<Self as DrawTarget>::Color>>>(
        &mut self,
        i: I,
    ) -> Result<(), <Self as DrawTarget>::Error> {
        let a = self.0.draw_iter(i);
        let _ = self.0.flush();
        a
    }
}

impl Dimensions for Display {
    fn bounding_box(&self) -> Rectangle {
        self.0.bounding_box()
    }
}
