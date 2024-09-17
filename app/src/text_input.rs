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

    pub fn scroll_up(&mut self) {
    }
}

impl<'a, C> Drawable for TextInput<'a, C>
where
    C: PixelColor,
{
    type Color = C;

    type Output = Point;

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut point = Point::new(0, 0);

        for event in &self.model.buffer[..(self.model.index + 1)] {
            match event {
                Some(multi_tap::Event::Decided(c)) => {
                    if self.style.measure_string(c.as_str(), point, Baseline::Top).next_position.x >= target.bounding_box().bottom_right().unwrap().x {
                        point.y = point.y + self.style.line_height() as i32;
                        point.x = 2;
                    }

                    point =
                        self.style
                            .draw_string(c.as_str(), point, Baseline::Top, target)?;
                }
                Some(multi_tap::Event::Tentative(c)) => {
                    if self.style.measure_string(c.as_str(), point, Baseline::Top).next_position.x >= target.bounding_box().bottom_right().unwrap().x {
                        point.y = point.y + self.style.line_height() as i32;
                        point.x = 2;
                    }

                    point = self.tentative_style.draw_string(
                        c.as_str(),
                        point,
                        Baseline::Top,
                        target,
                    )?;
                }
                None => {}
            }
        }

        return Ok(point);
    }
}

// #![no_std]
// #![feature(ascii_char)]
// pub mod text_input;
// use multi_tap::*;
// use embedded_graphics::prelude::DrawTarget;
// use text_input::{Model, TextInput};
// use embedded_graphics::{
//     mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10},
//     pixelcolor::BinaryColor
// };
// 
// struct Menu<'a, KEYPAD, DRAW_TARGET> where KEYPAD: Keypad, DRAW_TARGET: DrawTarget<Color = BinaryColor> {
//     multi_tap: MultiTap<KEYPAD>,
//     draw_target: DRAW_TARGET,
//     text_input: TextInput<'a, BinaryColor>
// }
// 
// impl <'a, KEYPAD, DRAW_TARGET>Menu<'a, KEYPAD, DRAW_TARGET> where KEYPAD: Keypad, DRAW_TARGET: DrawTarget<Color = BinaryColor> {
//     pub fn new(keypad: KEYPAD, draw_target: DRAW_TARGET) -> Self {
//         let mut buffer: [Option<multi_tap::Event>; 80] = [Default::default(); 80];
//         let mut model = Model::new(&mut buffer);
//         let mut multi_tap = MultiTap::new(keypad);
//         let mut text_input = TextInput::new(
//             &mut model,
//             MonoTextStyleBuilder::new()
//                 .font(&FONT_6X10)
//                 .text_color(BinaryColor::On)
//                 .background_color(BinaryColor::Off)
//                 .build(),
//             MonoTextStyleBuilder::new()
//                 .font(&FONT_6X10)
//                 .text_color(BinaryColor::Off)
//                 .background_color(BinaryColor::On)
//                 .build(),
//         );
// 
// 	Self { multi_tap, draw_target, text_input }
//     }
// }
