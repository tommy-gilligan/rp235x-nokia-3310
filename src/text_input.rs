use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::DrawTarget,
    prelude::*,
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};

pub struct Model<'a> {
    buffer: &'a mut [Option<multi_tap::Event>],
    index: usize,
}

impl<'a> Model<'a> {
    pub fn new(buffer: &'a mut [Option<multi_tap::Event>]) -> Self {
        Self { buffer, index: 0 }
    }

    pub fn update(&mut self, event: multi_tap::Event) {
        match event {
            e @ multi_tap::Event::Decided(_) => {
                if let Some(multi_tap::Event::Tentative(c)) = self.buffer[self.index] {
                    self.buffer[self.index] = Some(multi_tap::Event::Decided(c));
                } else {
                    self.buffer[self.index] = Some(e);
                }
                self.index += 1;
            }
            e @ multi_tap::Event::Tentative(_) => {
                self.buffer[self.index] = Some(e);
            }
        }
    }
}

pub struct TextInput<'a, C>
where
    C: PixelColor,
{
    model: &'a mut Model<'a>,
    style: MonoTextStyle<'a, C>,
    tentative_style: MonoTextStyle<'a, C>,
}

impl<'a, C> TextInput<'a, C>
where
    C: PixelColor,
{
    pub fn new(
        model: &'a mut Model<'a>,
        style: MonoTextStyle<'a, C>,
        tentative_style: MonoTextStyle<'a, C>,
    ) -> Self {
        Self {
            model,
            style,
            tentative_style,
        }
    }

    pub fn update(&mut self, event: multi_tap::Event) {
        self.model.update(event);
    }
}

impl<'a, C> Drawable for TextInput<'a, C>
where
    C: PixelColor,
{
    type Color = C;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut point = Point::new(2, 15);
        for event in &self.model.buffer[..(self.model.index + 1)] {
            match event {
                Some(multi_tap::Event::Decided(c)) => {
                    point =
                        self.style
                            .draw_string(c.as_str(), point, Baseline::Alphabetic, target)?;
                }
                Some(multi_tap::Event::Tentative(c)) => {
                    point = self.tentative_style.draw_string(
                        c.as_str(),
                        point,
                        Baseline::Alphabetic,
                        target,
                    )?;
                }
                None => {}
            }
        }

        return Ok(());
    }
}
