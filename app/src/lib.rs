#![no_std]
#![feature(ascii_char)]
pub mod text_input;
use multi_tap::*;
use text_input::{Model, TextInput};
use embedded_graphics::{
    mono_font::{MonoTextStyle, MonoTextStyleBuilder, ascii::FONT_6X10},
    pixelcolor::BinaryColor,
    text::{Alignment, Text},
    prelude::*,
};

pub struct Menu<'a, KEYPAD, DRAW_TARGET> where KEYPAD: Keypad, DRAW_TARGET: DrawTarget<Color = BinaryColor> {
    keypad: KEYPAD,
    draw_target: DRAW_TARGET,
    items: &'a [&'a str],
    selected: &'a &'a str
}

const MENU_ITEMS: [&'static str; 3] = ["Text Input", "Music", "Snake"];

impl <'a, KEYPAD, DRAW_TARGET>Menu<'a, KEYPAD, DRAW_TARGET> where KEYPAD: Keypad, DRAW_TARGET: DrawTarget<Color = BinaryColor> {
    pub fn new(keypad: KEYPAD, draw_target: DRAW_TARGET) -> Self {
	Self { keypad, draw_target, items: &MENU_ITEMS, selected: &MENU_ITEMS[0] }
    }

    pub async fn process(&mut self) {
        let top_left = self.draw_target.bounding_box().top_left;
	let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

        for (index, item) in self.items.into_iter().enumerate() {
	    Text::with_alignment(
		item,
		top_left + Point::new(0, 0) + Point::new(0, (index * 10).try_into().unwrap()),
		character_style,
		Alignment::Left,
	    ).draw(&mut self.draw_target);
        }
    }
}
