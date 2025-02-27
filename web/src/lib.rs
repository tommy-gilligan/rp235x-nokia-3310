#![allow(unexpected_cfgs)]
mod buzzer;
mod display;
mod keypad;
mod rtc;
mod vibration_motor;

use embassy_executor::Spawner;
use shared::Application;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let svg = document.get_element_by_id("svg1").unwrap();
    let mut vibration_motor = vibration_motor::Motor::new(svg);

    let svg = document.get_element_by_id("nokia").unwrap();
    let mut buzzer = buzzer::Buzzer::new(svg);
    let mut rtc = rtc::Clock::new();
    let mut beepy = hardware_test::HardwareTest::default();

    let svg = document.get_element_by_id("display").unwrap();
    let mut display = display::Display::new(svg);

    let mut keypad = keypad::DomKeypad::new(
        "cancel", "select", "up", "down", "one", "two", "three", "four", "five", "six", "seven",
        "eight", "nine", "asterisk", "zero", "hash",
    );

    loop {
        // decide your time budgets
        // 'trust' application takes at most 750ms
        // force pre-emption at 1500ms
        // how do you progress things inside app that take longer than 750?
        // special kind of timer?
        // forced pre-emption should be signalled back to application + print log entry
        match embassy_time::with_timeout(
            embassy_time::Duration::from_millis(1000),
            beepy.run(
                &mut vibration_motor,
                &mut buzzer,
                &mut display,
                &mut keypad,
                &mut rtc,
                None,
            ),
        )
        .await
        {
            Ok(Ok(None)) => {}
            Err(embassy_time::TimeoutError) => {
                println!("timed out");
            }
            _ => {
                unimplemented!()
            }
        }
    }
}
