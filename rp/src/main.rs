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

use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_rp::block::ImageDef;
use panic_probe as _;
use shared::Application;

// use core::cell::RefCell;
// use defmt::*;
// use display_interface_spi::SPIInterface;
// use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
// use embedded_graphics::{
//     draw_target::DrawTarget,
//     mono_font::{ascii::FONT_10X20, MonoTextStyle},
//     pixelcolor::BinaryColor,
//     prelude::Point,
//     text::Text,
//     Drawable,
// };
// use static_cell::StaticCell;
// use crate::spi::Blocking;

mod button;
mod buzzer;
mod clock;
mod vibration_motor;

// static mut CORE1_STACK: Stack<4096> = Stack::new();
// static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

// #[embassy_executor::task]
// async fn core1_task(
//     spi0: SPI0,
//     reset: PIN_33,
//     dc: PIN_36,
//     display_cs: PIN_37,
//     clk: PIN_38,
//     mosi: PIN_39,
//     miso: PIN_20,
// ) {
//     let mut display_config = spi::Config::default();
//     display_config.frequency = 4_000_000;
//
//     let spi: Spi<'_, _, Blocking> =
//         Spi::new_blocking(spi0, clk, mosi, miso, display_config.clone());
//     let spi_bus: Mutex<NoopRawMutex, _> = Mutex::new(RefCell::new(spi));
//
//     let display_spi = SpiDeviceWithConfig::new(
//         &spi_bus,
//         Output::new(display_cs, Level::High),
//         display_config,
//     );
//
//     let mut pcd8544: pcd8544::Driver<
//         SPIInterface<
//             embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<
//                 '_,
//                 NoopRawMutex,
//                 embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>,
//                 Output<'_>,
//             >,
//             Output<'_>,
//         >,
//         Output<'_>,
//         core::convert::Infallible,
//     > = pcd8544::Driver::new(
//         SPIInterface::new(display_spi, Output::new(dc, Level::High)),
//         Output::new(reset, Level::High),
//     );
//
//     loop {
//         pcd8544.init(&mut Delay).unwrap();
//         pcd8544.set_contrast(64).unwrap();
//         pcd8544.invert_display(true).unwrap();
//         pcd8544.clear(BinaryColor::Off).unwrap();
//
//         let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
//         Text::new("ABC", Point::new(20, 30), style)
//             .draw(&mut pcd8544)
//             .unwrap();
//         pcd8544.flush().unwrap();
//
//         Timer::after_millis(1000).await;
//     }
// }

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    // let spi0 = p.SPI0;
    // let reset = p.PIN_33;
    // let dc = p.PIN_36;
    // let display_cs = p.PIN_37;
    // let clk = p.PIN_38;
    // let mosi = p.PIN_39;
    // let miso = p.PIN_20;

    // spawn_core1(
    //     p.CORE1,
    //     unsafe { &mut *core::ptr::addr_of_mut!(CORE1_STACK) },
    //     move || {
    //         let executor1 = EXECUTOR1.init(Executor::new());
    //         executor1.run(|spawner| {
    //             unwrap!(spawner.spawn(core1_task(spi0, reset, dc, display_cs, clk, mosi, miso)))
    //         });
    //     },
    // );

    let _button = button::Button::new(p.PIN_28);

    shared::Beepy::run(
        vibration_motor::Motor::new(p.PIN_2),
        buzzer::Beeper::new(p.PWM_SLICE2, p.PIN_21),
        clock::Clock::new(p.I2C1, p.PIN_46, p.PIN_47),
    )
    .await;
}
