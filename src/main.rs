#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::pwm::{Config, Pwm};
use embassy_time::Instant;
use {defmt_rtt as _, panic_probe as _};
use embassy_rp::gpio;
use gpio::{Input, Level, Output, Pull};
use fixed::traits::ToFixed;

// 440Hz 90%
fn pwm_config(frequency: u32, duty: u32) -> Config {
    let top: u32 = 1000000 / frequency - 1;
    let level: u16 = ((top + 1) * duty / 100 - 1).try_into().unwrap();

    let mut c: Config = Default::default();
    c.divider = 125.0.to_fixed();
    c.top = top.try_into().unwrap();
    c.compare_a = level;
    c
}

const SONG_TEXT: &'static str = "Wannabe:d=4, o=5, b=125:16g, 16g, 16g, 16g, 8g, 8a, 8g, 8e, 8p, 16c, 16d, 16c, 8d, 8d, 8c, e, p, 8g, 8g, 8g, 8a, 8g, 8e, 8p, c6, 8c6, 8b, 8g, 8a, 16b, 16a, g";

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut pwm = Pwm::new_output_a(
        p.PWM_SLICE2,
        p.PIN_2,
        pwm_config(440, 90)
    );

    loop {
        let mut song = rtttl::Song::new(SONG_TEXT);
        let time_at_start = Instant::now();

        loop {
            let time_since_start = Instant::now().duration_since(time_at_start).as_micros() as u32;
            if let Some(note) = song.note_at(time_since_start) {
                match note.frequency() {
                    Some(Ok(frequency)) => pwm.set_config(&pwm_config(frequency, 90)),
                    Some(_) => break,
                    None => { }
                }
            } else {
                break;
            }
        }
    }
}
