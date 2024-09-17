#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(trivial_bounds)]
#![feature(let_chains)]

use embassy_executor::Spawner;
use embassy_time::Timer;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use embedded_graphics_web_simulator::{
    display::WebSimulatorDisplay, output_settings::OutputSettingsBuilder,
};
use wasm_bindgen::prelude::*;
use web_sys::console;
use embedded_graphics::pixelcolor::BinaryColor;
use web_sys::MouseEvent;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    prelude::Point,
    text::Text,
    Drawable,
};
use multi_tap::*;
use core::ascii::Char;
use web_sys::Element;
use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::BorrowMut;

mod buzzer;
use buzzer::*;

mod keypad;
use keypad::*;

#[embassy_executor::task]
async fn ticker() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let logo = document.get_element_by_id("nokia").unwrap();
    let mut buzzer = Buzzer::new();
    let closure = Closure::<dyn FnMut(_)>::new(move |_event: web_sys::MouseEvent| {
        buzzer.start().unwrap();
        buzzer.set_frequency(440.0).unwrap();
    });
    logo.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    let output_settings = OutputSettingsBuilder::new()
        .scale(1)
        .pixel_spacing(0)
        .build();

    let mut display: WebSimulatorDisplay<embedded_graphics::pixelcolor::BinaryColor> = WebSimulatorDisplay::new(
        (84, 48),
        &output_settings,
        None
    );

    // let mut keypad = DomK::new(
    //   document.get_element_by_id("cancel").unwrap(),
    //   document.get_element_by_id("select").unwrap(),
    //   document.get_element_by_id("updown").unwrap(),
    //   document.get_element_by_id("one").unwrap(),
    //   document.get_element_by_id("two").unwrap(),
    //   document.get_element_by_id("three").unwrap(),
    //   document.get_element_by_id("four").unwrap(),
    //   document.get_element_by_id("five").unwrap(),
    //   document.get_element_by_id("six").unwrap(),
    //   document.get_element_by_id("seven").unwrap(),
    //   document.get_element_by_id("eight").unwrap(),
    //   document.get_element_by_id("nine").unwrap(),
    //   document.get_element_by_id("asterisk").unwrap(),
    //   document.get_element_by_id("zero").unwrap(),
    //   document.get_element_by_id("hash").unwrap(),
    // );
    // keypad.init();

    Text::new(
	"HELLO",
	Point::new(10, 10),
	MonoTextStyle::new(&FONT_6X9, BinaryColor::Off)
    ).draw(&mut display).unwrap();
    display.flush().expect("Couldn't update");

    loop {
	console::log_1(&"down".into());
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());
    spawner.spawn(ticker()).unwrap();
}
