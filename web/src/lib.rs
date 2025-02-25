#![allow(unexpected_cfgs)]
mod buzzer;
mod clock;
mod vibration_motor;

use embassy_executor::Spawner;
use shared::Application;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let svg = document.get_element_by_id("svg1").unwrap();
    let vibration_motor = vibration_motor::Motor::new(svg);

    let svg = document.get_element_by_id("svg1").unwrap();
    let buzzer = buzzer::Buzzer::new(svg);

    shared::Beepy::run(vibration_motor, buzzer, clock::Clock::new()).await;
}
