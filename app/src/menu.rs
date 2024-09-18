use crate::keypad::Keypad;
use crate::text_input::{Model, TextInput};
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};

pub struct Menu<'a, KEYPAD, DRAW_TARGET>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
{
    keypad: KEYPAD,
    draw_target: DRAW_TARGET,
    items: &'a [&'a str],
    index: usize,
}

const MENU_ITEMS: [&'static str; 3] = ["Text Input", "Music", "Snake"];

impl<'a, KEYPAD, DRAW_TARGET> Menu<'a, KEYPAD, DRAW_TARGET>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
{
    pub fn new(keypad: KEYPAD, draw_target: DRAW_TARGET) -> Self {
        let mut s = Self {
            keypad,
            draw_target,
            items: &MENU_ITEMS,
            index: 0,
        };
        s.draw();
        s
    }

    fn draw(&mut self) {
        let bounding_box = self.draw_target.bounding_box();
        let top_left = bounding_box.top_left;

        bounding_box
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut self.draw_target);

        for (index, item) in self.items.into_iter().enumerate() {
            let y_offset: i32 = (index * 10).try_into().unwrap();
            if self.index == index {
                Rectangle::new(
                    top_left + Point::new(0, y_offset + 2),
                    Size::new(self.draw_target.bounding_box().size.width, 11),
                )
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
                .draw(&mut self.draw_target);

                Text::with_alignment(
                    item,
                    top_left + Point::new(0, 10) + Point::new(0, y_offset),
                    MonoTextStyle::new(&FONT_6X10, BinaryColor::On),
                    Alignment::Left,
                )
                .draw(&mut self.draw_target);
            } else {
                Text::with_alignment(
                    item,
                    top_left + Point::new(0, 10) + Point::new(0, y_offset),
                    MonoTextStyle::new(&FONT_6X10, BinaryColor::Off),
                    Alignment::Left,
                )
                .draw(&mut self.draw_target);
            }
        }
    }

    fn down(&mut self) {
        self.index = (self.index + 1) % self.items.len()
    }

    fn up(&mut self) {
        if self.index == 0 {
            self.index = self.items.len() - 1;
        } else {
            self.index -= 1;
        }
    }

    pub async fn process(&mut self) {
        match self.keypad.event().await {
            crate::keypad::Event::Down(crate::keypad::Button::Down) => {
                self.down();
                self.draw();
            }
            crate::keypad::Event::Down(crate::keypad::Button::Up) => {
                self.up();
                self.draw();
            }
            _ => {}
        }
    }
}
