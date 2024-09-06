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
use embassy_rp::i2c::Config as SConfig;
use embassy_rp::spi::{Spi, self};
use core::cell::RefCell;
use display_interface_spi::SPIInterface;
use embassy_time::Delay;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::Circle;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::Drawable;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embedded_hal_1::spi::{Operation, SpiDevice as OtherSpiDevice};
use pcd8544::Driver as PCD8544;

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

    let mut config = spi::Config::default();
    config.frequency = 4_000_000;
    let mut spi = Spi::new_blocking(p.SPI0, p.PIN_18, p.PIN_19, p.PIN_16, config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

    let mut display_spi = SpiDeviceWithConfig::new(
        &spi_bus,
        Output::new(p.PIN_17, Level::High),
        config.clone()
    );
    let mut pcd8544 = PCD8544::new(
        SPIInterface::new(display_spi, Output::new(p.PIN_20, Level::High)),
        Output::new(p.PIN_21, Level::High)
    );
    pcd8544.init(&mut Delay);

    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);
    Circle::new(Point::new(20, 20), 10)
        .into_styled(thin_stroke)
        .draw(&mut pcd8544).unwrap();

    pcd8544.flush();

    let mut pwm = Pwm::new_output_a(
        p.PWM_SLICE1,
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
