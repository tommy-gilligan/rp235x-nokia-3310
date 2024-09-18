#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(trivial_bounds)]
#![feature(let_chains)]

use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::console;
use web_sys::MouseEvent;

use core::ascii::Char;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    prelude::Point,
    text::Text,
    Drawable,
};
use multi_tap::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::Element;

mod buzzer;
mod stub;
use buzzer::Buzzer as MyBuzzer;
use stub::*;

use app::buzzer::Buzzer;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::Pixel;

mod dom_keypad;
use dom_keypad::*;

use embedded_graphics::prelude::Dimensions;
use embedded_graphics::prelude::DrawTarget;

struct Flushing(WebSimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor>);

impl DrawTarget for Flushing {
    type Color = BinaryColor;

    type Error = <WebSimulatorDisplay<BinaryColor> as DrawTarget>::Error;

    fn draw_iter<I: IntoIterator<Item = Pixel<<Self as DrawTarget>::Color>>>(
        &mut self,
        i: I,
    ) -> Result<(), <Self as DrawTarget>::Error> {
        let a = self.0.draw_iter(i);
        self.0.flush();
        a
    }
}

impl Dimensions for Flushing {
    fn bounding_box(&self) -> Rectangle {
        self.0.bounding_box()
    }
}

#[embassy_executor::task]
async fn ticker() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let logo = document.get_element_by_id("nokia").unwrap();
    // let mut buzzer = MyBuzzer::new();
    // let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
    //     buzzer.enable();
    // });
    // // buzzer.set_frequency(440);
    // logo.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    let output_settings = OutputSettingsBuilder::new()
        .scale(2)
        .pixel_spacing(0)
        .alpha_color(embedded_graphics::pixelcolor::BinaryColor::On)
        .build();

    let mut display: WebSimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor> =
        WebSimulatorDisplay::new((84, 48), &output_settings, None);

    let mut dom_keypad = DomKeypad::new(
        "cancel", "select", "up", "down", "one", "two", "three", "four", "five", "six", "seven",
        "eight", "nine", "asterisk", "zero", "hash",
    );
    let mut menu = app::Menu::new(dom_keypad, Flushing(display));

    loop {
        menu.process().await;
        console::log_1(&"process".into());
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());
    spawner.spawn(ticker()).unwrap();
}
