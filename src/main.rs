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
use embassy_time::{Timer};
use embedded_hal_1::delay::DelayNs;
use defmt::Format;

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

#[derive(Debug, PartialEq, Format)]
enum Row {
    A,
    B,
    C,
    D
}

#[derive(Debug, PartialEq, Format)]
enum Col {
    A,
    B,
    C,
}

fn get_key(
    row_a: &Input,
    row_b: &Input,
    row_c: &Input,
    row_d: &Input,
    col_a: &mut Output,
    col_b: &mut Output,
    col_c: &mut Output
) -> Option<(Row, Col)> {
    col_a.set_high();
    col_b.set_low();
    col_c.set_low();
    Delay.delay_us(10);

    match (row_a.is_high(), row_b.is_high(), row_c.is_high(), row_d.is_high()) {
        (true, _, _, _) => return Some((Row::A, Col::A)),
        (_, true, _, _) => return Some((Row::B, Col::A)),
        (_, _, true, _) => return Some((Row::C, Col::A)),
        (_, _, _, true) => return Some((Row::D, Col::A)),
        _ => {}
    }

    col_a.set_low();
    col_b.set_high();
    col_c.set_low();
    Delay.delay_us(10);
    match (row_a.is_high(), row_b.is_high(), row_c.is_high(), row_d.is_high()) {
        (true, _, _, _) => return Some((Row::A, Col::B)),
        (_, true, _, _) => return Some((Row::B, Col::B)),
        (_, _, true, _) => return Some((Row::C, Col::B)),
        (_, _, _, true) => return Some((Row::D, Col::B)),
        _ => {}
    }

    col_a.set_low();
    col_b.set_low();
    col_c.set_high();
    Delay.delay_us(10);

    match (row_a.is_high(), row_b.is_high(), row_c.is_high(), row_d.is_high()) {
        (true, _, _, _) => return Some((Row::A, Col::C)),
        (_, true, _, _) => return Some((Row::B, Col::C)),
        (_, _, true, _) => return Some((Row::C, Col::C)),
        (_, _, _, true) => return Some((Row::D, Col::C)),
        _ => {}
    }
    return None;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut config = spi::Config::default();
    config.frequency = 4_000_000;
    let mut spi = Spi::new_blocking(p.SPI1, p.PIN_14, p.PIN_15, p.PIN_8, config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

    let mut display_spi = SpiDeviceWithConfig::new(
        &spi_bus,
        Output::new(p.PIN_13, Level::High),
        config.clone()
    );
    let mut pcd8544 = PCD8544::new(
        SPIInterface::new(display_spi, Output::new(p.PIN_11, Level::High)),
        Output::new(p.PIN_12, Level::High)
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


    let mut delay = Delay;
    let a = Input::new(p.PIN_9, Pull::Down);
    let b = Input::new(p.PIN_3, Pull::Down);
    let c = Input::new(p.PIN_4, Pull::Down);
    let d = Input::new(p.PIN_6, Pull::Down);
    let mut x = Output::new(p.PIN_7, Level::High);
    let mut y = Output::new(p.PIN_10, Level::High);
    let mut z = Output::new(p.PIN_5, Level::High);

    loop {
        let key = get_key(&a, &b, &c, &d, &mut x, &mut y, &mut z);
        delay.delay_ms(50);
        println!("{:?}", key);

        // let mut song = rtttl::Song::new(SONG_TEXT);
        // let time_at_start = Instant::now();

        // loop {
        //     let time_since_start = Instant::now().duration_since(time_at_start).as_micros() as u32;
        //     if let Some(note) = song.note_at(time_since_start) {
        //         match note.frequency() {
        //             Some(Ok(frequency)) => pwm.set_config(&pwm_config(frequency, 90)),
        //             Some(_) => break,
        //             None => { }
        //         }
        //     } else {
        //         break;
        //     }
        // }
    }
}
