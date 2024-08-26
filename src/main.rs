#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Input, Level, Output, Pull};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut keypad = embassy_keypad::Keypad::new(
        Input::new(p.PIN_4, Pull::Down),
        Input::new(p.PIN_11, Pull::Down),
        Input::new(p.PIN_9, Pull::Down),
        Input::new(p.PIN_10, Pull::Down),
        Input::new(p.PIN_8, Pull::Down),
        Input::new(p.PIN_7, Pull::Down),
        Input::new(p.PIN_22, Pull::Down),
        Input::new(p.PIN_6, Pull::Down),
        Input::new(p.PIN_5, Pull::Down),
        Input::new(p.PIN_27, Pull::Down),
        Input::new(p.PIN_2, Pull::Down),
        Input::new(p.PIN_3, Pull::Down),
        Input::new(p.PIN_21, Pull::Down),
        // allows pins 0 and 1 to be used for serial debugging
        Input::new(p.PIN_12, Pull::Down),
        Input::new(p.PIN_13, Pull::Down),
        Input::new(p.PIN_26, Pull::Down),
    );

    loop {
	match keypad.key_down().await {
	    embassy_keypad::KeyPress::Select => info!("Select"),
	    embassy_keypad::KeyPress::Cancel => info!("Cancel"),
	    embassy_keypad::KeyPress::Up => info!("Up"),
	    embassy_keypad::KeyPress::Down => info!("Down"),
	    embassy_keypad::KeyPress::One => info!("One"),
	    embassy_keypad::KeyPress::Two => info!("Two"),
	    embassy_keypad::KeyPress::Three => info!("Three"),
	    embassy_keypad::KeyPress::Four => info!("Four"),
	    embassy_keypad::KeyPress::Five => info!("Five"),
	    embassy_keypad::KeyPress::Six => info!("Six"),
	    embassy_keypad::KeyPress::Seven => info!("Seven"),
	    embassy_keypad::KeyPress::Eight => info!("Eight"),
	    embassy_keypad::KeyPress::Nine => info!("Nine"),
	    embassy_keypad::KeyPress::Asterisk => info!("Asterisk"),
	    embassy_keypad::KeyPress::Zero => info!("Zero"),
	    embassy_keypad::KeyPress::Hash => info!("Hash"),
	}
        Timer::after_millis(20).await;
    }
}
