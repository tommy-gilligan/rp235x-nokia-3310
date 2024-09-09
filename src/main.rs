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

#[derive(Debug, PartialEq, Format, Copy, Clone)]
enum Button {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Asterisk,
    Zero,
    Hash,
}

struct Matrix<'a> {
    active: Option<Button>,
    row_a: Input<'a>,
    row_b: Input<'a>,
    row_c: Input<'a>,
    row_d: Input<'a>,
    col_a: Output<'a>,
    col_b: Output<'a>,
    col_c: Output<'a>
}

impl <'a>Matrix<'a> {
    pub fn new(row_a: Input<'a>, row_b: Input<'a>, row_c: Input<'a>, row_d: Input<'a>, mut col_a: Output<'a>, mut col_b: Output<'a>, mut col_c: Output<'a>) -> Matrix<'a> {
        col_a.set_low();
        col_b.set_low();
        col_c.set_low();

        Matrix {
            active: None,
            row_a,
            row_b,
            row_c,
            row_d,
            col_a,
            col_b,
            col_c
        }
    }

    async fn button_down(&mut self) -> Option<Button> {
        self.col_a.set_high();
        self.col_b.set_low();
        self.col_c.set_low();
        Delay.delay_us(10);

        let col_a = match (self.row_a.is_high(), self.row_b.is_high(), self.row_c.is_high(), self.row_d.is_high()) {
            (true, _, _, _) => Some(Button::One),
            (_, true, _, _) => Some(Button::Four),
            (_, _, true, _) => Some(Button::Seven),
            (_, _, _, true) => Some(Button::Asterisk),
            _ => None
        };

        self.col_a.set_low();
        self.col_b.set_high();
        self.col_c.set_low();
        Delay.delay_us(10);
        let col_b = match (self.row_a.is_high(), self.row_b.is_high(), self.row_c.is_high(), self.row_d.is_high()) {
            (true, _, _, _) => Some(Button::Two),
            (_, true, _, _) => Some(Button::Five),
            (_, _, true, _) => Some(Button::Eight),
            (_, _, _, true) => Some(Button::Zero),
            _ => None
        };

        self.col_a.set_low();
        self.col_b.set_low();
        self.col_c.set_high();
        Delay.delay_us(10);

        let col_c = match (self.row_a.is_high(), self.row_b.is_high(), self.row_c.is_high(), self.row_d.is_high()) {
            (true, _, _, _) => Some(Button::Three),
            (_, true, _, _) => Some(Button::Six),
            (_, _, true, _) => Some(Button::Nine),
            (_, _, _, true) => Some(Button::Hash),
            _ => None
        };

        let result = col_a.or(col_b).or(col_c);
        if self.active != result {
            self.active = result;
            return result;
        } else {
            return None;
        }
    }
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
    let mut matrix = Matrix::new(
        Input::new(p.PIN_9, Pull::Down),
        Input::new(p.PIN_3, Pull::Down),
        Input::new(p.PIN_4, Pull::Down),
        Input::new(p.PIN_6, Pull::Down),
        Output::new(p.PIN_7, Level::High),
        Output::new(p.PIN_10, Level::High),
        Output::new(p.PIN_5, Level::High),
    );

    loop {
        if let Some(button) = matrix.button_down().await {
            println!("{:?}", button);
        }

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
