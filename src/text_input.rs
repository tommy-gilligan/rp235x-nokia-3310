use core::str::from_utf8;

use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::DrawTarget,
    prelude::*,
    text::{renderer::TextRenderer, Baseline},
    Drawable,
};
use core::fmt::Display;

use crate::multitap::MultiTap;
use crate::MultiTapEvent;
use core::fmt::Formatter;

pub struct Model<'a> {
    buffer: &'a mut [Option<MultiTapEvent>],
    index: usize
}

impl <'a>Model<'a> {
    pub fn new(buffer: &'a mut [Option<MultiTapEvent>]) -> Self {
        Self {
            buffer,
            index: 0
        }
    }

    pub fn update(&mut self, event: MultiTapEvent) {
        match event {
            e @ MultiTapEvent::Decided(_) => {
                if let Some(MultiTapEvent::Tentative(c)) = self.buffer[self.index] {
                    self.buffer[self.index] = Some(MultiTapEvent::Decided(c));
                } else {
                    self.buffer[self.index] = Some(e);
                }
                self.index += 1;
            }
            e @ MultiTapEvent::Tentative(_) => {
                self.buffer[self.index] = Some(e);
            }
        }
    }
}

pub struct TextInput<'a, C> where C: PixelColor {
    model: &'a mut Model<'a>,
    style: MonoTextStyle<'a, C>,
    tentative_style: MonoTextStyle<'a, C>,
}

impl<'a, C> TextInput<'a, C> where C: PixelColor {
    pub fn new(
        model: &'a mut Model<'a>,
        style: MonoTextStyle<'a, C>,
        tentative_style: MonoTextStyle<'a, C>,
    ) -> Self {
        Self { model, style, tentative_style }
    }

    pub fn update(&mut self, event: Option<MultiTapEvent>) {
        if let Some(e) = event {
            self.model.update(e);
        }
    }
}

impl<'a, C> Drawable for TextInput<'a, C> where C: PixelColor {
    type Color = C;

    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let mut point = Point::new(10, 10);
        for event in &self.model.buffer[..(self.model.index + 1)] {
            match event {
                Some(MultiTapEvent::Decided(c)) => {
                    point = self.style.draw_string(
                        c.as_str(),
                        point,
                        Baseline::Alphabetic,
                        target,
                    )?;
                },
                Some(MultiTapEvent::Tentative(c)) => {
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
