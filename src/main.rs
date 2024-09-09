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
use embassy_futures::select::{select4, Either4};
use embedded_hal_async::digital::Wait;

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

#[derive(Debug, PartialEq, Format, Copy, Clone)]
enum ButtonEvent {
    Down(Button),
    Up(Button),
    None
}

struct Matrix<'a> {
    last_event: ButtonEvent,
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
        col_a.set_high();
        col_b.set_high();
        col_c.set_high();

        Matrix {
            last_event: ButtonEvent::None,
            row_a,
            row_b,
            row_c,
            row_d,
            col_a,
            col_b,
            col_c
        }
    }

    async fn event(&mut self) -> ButtonEvent {
        let mut result = ButtonEvent::None;

        match self.last_event {
            ButtonEvent::Down(b @ Button::One | b @ Button::Two | b @ Button::Three) => {
                self.row_a.wait_for_low().await;
                self.last_event = ButtonEvent::Up(b);
                return self.last_event;
            },
            ButtonEvent::Down(b @ Button::Four | b @ Button::Five | b @ Button::Six) => {
                self.row_b.wait_for_low().await;
                self.last_event = ButtonEvent::Up(b);
                return self.last_event;
            },
            ButtonEvent::Down(b @ Button::Seven | b @ Button::Eight | b @ Button::Nine) => {
                self.row_c.wait_for_low().await;
                self.last_event = ButtonEvent::Up(b);
                return self.last_event;
            },
            ButtonEvent::Down(b @ Button::Asterisk | b @ Button::Zero | b @ Button::Hash) => {
                self.row_d.wait_for_low().await;
                self.last_event = ButtonEvent::Up(b);
                return self.last_event;
            },
            _ => {}
        }

        match select4(
            self.row_a.wait_for_high(),
            self.row_b.wait_for_high(),
            self.row_c.wait_for_high(),
            self.row_d.wait_for_high(),
        ).await {
            Either4::First(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_micros(10).await;

                if self.row_a.is_high() && self.last_event != ButtonEvent::Down(Button::One) {
                    self.last_event = ButtonEvent::Down(Button::One);
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_micros(10).await;

                if self.row_a.is_high() && self.last_event != ButtonEvent::Down(Button::Two) {
                    self.last_event = ButtonEvent::Down(Button::Two);
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_micros(10).await;

                if self.row_a.is_high() && self.last_event != ButtonEvent::Down(Button::Three) {
                    self.last_event = ButtonEvent::Down(Button::Three);
                    result = self.last_event;
                }
            },
            Either4::Second(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_micros(10).await;

                if self.row_b.is_high() && self.last_event != ButtonEvent::Down(Button::Four) {
                    self.last_event = ButtonEvent::Down(Button::Four);
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_micros(10).await;

                if self.row_b.is_high() && self.last_event != ButtonEvent::Down(Button::Five) {
                    self.last_event = ButtonEvent::Down(Button::Five);
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_micros(10).await;

                if self.row_b.is_high() && self.last_event != ButtonEvent::Down(Button::Six) {
                    self.last_event = ButtonEvent::Down(Button::Six);
                    result = self.last_event;
                }
            },
            Either4::Third(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_micros(10).await;

                if self.row_c.is_high() && self.last_event != ButtonEvent::Down(Button::Seven) {
                    self.last_event = ButtonEvent::Down(Button::Seven);
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_micros(10).await;

                if self.row_c.is_high() && self.last_event != ButtonEvent::Down(Button::Eight) {
                    self.last_event = ButtonEvent::Down(Button::Eight);
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_micros(10).await;

                if self.row_c.is_high() && self.last_event != ButtonEvent::Down(Button::Nine) {
                    self.last_event = ButtonEvent::Down(Button::Nine);
                    result = self.last_event;
                }
            },
            Either4::Fourth(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_micros(10).await;

                if self.row_d.is_high() && self.last_event != ButtonEvent::Down(Button::Asterisk) {
                    self.last_event = ButtonEvent::Down(Button::Asterisk);
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_micros(10).await;

                if self.row_d.is_high() && self.last_event != ButtonEvent::Down(Button::Zero) {
                    self.last_event = ButtonEvent::Down(Button::Zero);
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_micros(10).await;

                if self.row_d.is_high() && self.last_event != ButtonEvent::Down(Button::Hash) {
                    self.last_event = ButtonEvent::Down(Button::Hash);
                    result = self.last_event;
                }
            }
        }

        self.col_a.set_high();
        self.col_b.set_high();

        result
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
        match matrix.event().await {
            ButtonEvent::Down(button) => println!("down {:?}", button),
            ButtonEvent::Up(button) => println!("up {:?}", button),
            _ => {}
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
