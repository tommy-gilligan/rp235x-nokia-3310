mod buzzer;
mod display;
mod keypad;
mod stub;

use embassy_executor::Spawner;

#[embassy_executor::task]
async fn ticker() {
    let mut menu = app::menu::Menu::new(
        keypad::DomKeypad::new(
            "cancel", "select", "up", "down", "one", "two", "three", "four", "five", "six",
            "seven", "eight", "nine", "asterisk", "zero", "hash",
        ),
        display::Display::new(),
    );

    loop {
        menu.process().await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    wasm_logger::init(wasm_logger::Config::default());
    spawner.spawn(ticker()).unwrap();
}
