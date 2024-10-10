#![no_std]
#![no_main]

#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: ImageDef = ImageDef::secure_exe();

// Program metadata for `picotool info`.
// This isn't needed, but it's recomended to have these minimal entries.
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(
        c"This example tests the RP Pico on board LED, connected to gpio 25"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

use core::cell::RefCell;

use embassy_time::Timer;
use defmt::*;
use defmt_rtt as _;
use display_interface_spi::SPIInterface;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_executor::Spawner;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    pwm::{Config, Pwm},
    spi::{self, Spi},
    block::ImageDef,
};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use embassy_time::Delay;
use panic_probe as _;

mod buzzer;
mod matrix;
mod usb;

use buzzer::*;
use matrix::*;
use pcd8544::Driver as PCD8544;

const SONG_TEXT: &str = "Wannabe:d=4, o=5, b=125:16g, 16g, 16g, 16g, 8g, 8a, 8g, 8e, 8p, 16c, 16d, 16c, 8d, 8d, 8c, e, p, 8g, 8g, 8g, 8a, 8g, 8e, 8p, c6, 8c6, 8b, 8g, 8a, 16b, 16a, g";

// use embassy_rp::rtc::{DateTime, DayOfWeek, Rtc};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // let mut rtc = Rtc::new(p.RTC);

    // if !rtc.is_running() {
    //     let now = chrono::naive::NaiveDateTime::from_timestamp(1727617276744 / 1000, 0);
    //     rtc.set_datetime(now).unwrap();
    // }


    let mut config = spi::Config::default();
    config.frequency = 4_000_000;
    let spi = Spi::new_blocking(p.SPI1, p.PIN_14, p.PIN_15, p.PIN_8, config.clone());
    let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));

    let display_spi =
        SpiDeviceWithConfig::new(&spi_bus, Output::new(p.PIN_13, Level::High), config.clone());
    let mut pcd8544 = PCD8544::new(
        SPIInterface::new(display_spi, Output::new(p.PIN_11, Level::High)),
        Output::new(p.PIN_12, Level::High),
    );
    pcd8544.init(&mut Delay).unwrap();
    pcd8544.set_contrast(64).unwrap();
    pcd8544.invert_display(true);

    let mut _buzzer = Buzzer::new(Pwm::new_output_a(p.PWM_SLICE1, p.PIN_2, Config::default()));

    let matrix = Matrix::new(
        Input::new(p.PIN_9, Pull::Down),
        Input::new(p.PIN_3, Pull::Down),
        Input::new(p.PIN_4, Pull::Down),
        Input::new(p.PIN_6, Pull::Down),
        Output::new(p.PIN_7, Level::High),
        Output::new(p.PIN_10, Level::High),
        Output::new(p.PIN_5, Level::High),
    );

    // let c = ;
    let mut menu = app::otp::Otp::new(matrix, pcd8544, 0, || 0);

    // rtc.now().unwrap().and_utc().timestamp().try_into().unwrap()
    loop {
        menu.process().await
    }
}
