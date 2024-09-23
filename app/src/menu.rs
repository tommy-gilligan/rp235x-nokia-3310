use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};

use crate::keypad::Keypad;

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

const MENU_ITEMS: [&str; 3] = ["Text Input", "Music", "Snake"];

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

        let _ = bounding_box
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut self.draw_target);

        for (index, item) in self.items.iter().enumerate() {
            let y_offset: i32 = (index * 10).try_into().unwrap();
            if self.index == index {
                let _ = Rectangle::new(
                    top_left + Point::new(0, y_offset + 2),
                    Size::new(self.draw_target.bounding_box().size.width, 11),
                )
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::Off))
                .draw(&mut self.draw_target);

                let _ = Text::with_alignment(
                    item,
                    top_left + Point::new(0, 10) + Point::new(0, y_offset),
                    MonoTextStyle::new(&FONT_6X10, BinaryColor::On),
                    Alignment::Left,
                )
                .draw(&mut self.draw_target);
            } else {
                let _ = Text::with_alignment(
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
