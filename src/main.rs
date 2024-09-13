#![no_std]
#![no_main]
#![feature(ascii_char)]
#![feature(ascii_char_variants)]
#![feature(trivial_bounds)]
#![feature(let_chains)]

use core::ascii::Char;
use usbd_hid::descriptor::KeyboardUsage;

use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
};
use defmt::*;
use display_interface_spi::SPIInterface;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig; use embassy_executor::Spawner; use embassy_futures::join::join;
use embassy_rp::{
    gpio::{Input, Level, Output, Pull},
    pwm::{Config, Pwm},
    spi::{self, Spi},
    bind_interrupts,
    peripherals::USB,
    usb::{Driver, InterruptHandler},
};
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use embassy_time::Delay;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    Drawable,
};
use embassy_usb::{
    class::hid::{HidReaderWriter, ReportId, RequestHandler, State},
    control::OutResponse,
    Builder,
    Handler
};
use {defmt_rtt as _, panic_probe as _};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use embassy_usb::class::hid::HidWriter;
use embassy_time::Timer;

mod buzzer;
mod text_input;
mod usb;
mod matrix;

use pcd8544::Driver as PCD8544;
use buzzer::*;
use multi_tap::MultiTap;
use text_input::TextInput;
use crate::text_input::Model;
use matrix::*;

const SONG_TEXT: &str = "Wannabe:d=4, o=5, b=125:16g, 16g, 16g, 16g, 8g, 8a, 8g, 8e, 8p, 16c, 16d, 16c, 8d, 8d, 8c, e, p, 8g, 8g, 8g, 8a, 8g, 8e, 8p, c6, 8c6, 8b, 8g, 8a, 16b, 16a, g";

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

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

    let mut buffer: [Option<multi_tap::Event>; 80] = [Default::default(); 80];
    let mut model = Model::new(&mut buffer);

    let mut multi_tap = MultiTap::new(matrix);
    let mut text_input = TextInput::new(
        &mut model,
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::On)
            .background_color(BinaryColor::Off)
            .build(),
        MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build()
    );

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut state = State::new();
    let driver = Driver::new(p.USB, Irqs);
    let mut device_handler = usb::MultiTapKeyboard::new();
    let (mut usb, (reader, mut writer)) = usb::new(
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
        driver,
        &mut state,
        &mut device_handler
    );

    join(
        usb.run(),
        join(
            async {
                loop {
                    let event = multi_tap.event(Timer::after_secs(2)).await;

                    if let multi_tap::Event::Decided(c) = event {
                        match writer.write_serialize(&KeyboardReport {
                            keycodes: [0, 0, 0, 0, 0, 0],
                            leds: 0,
                            modifier: 0x02,
                            reserved: 0,
                        }).await {
                            Ok(()) => {}
                            Err(e) => warn!("Failed to send report: {:?}", e),
                        };

                        match writer.write_serialize(&KeyboardReport {
                            keycodes: [usb::char_to_keycode(c) as u8, 0, 0, 0, 0, 0],
                            leds: 0,
                            modifier: 0x02,
                            reserved: 0,
                        }).await {
                            Ok(()) => {}
                            Err(e) => warn!("Failed to send report: {:?}", e),
                        };

                        match writer.write_serialize(&KeyboardReport {
                            keycodes: [0, 0, 0, 0, 0, 0],
                            leds: 0,
                            modifier: 0x00,
                            reserved: 0,
                        }).await {
                            Ok(()) => {}
                            Err(e) => warn!("Failed to send report: {:?}", e),
                        };
                    }

                    text_input.update(event);
                    text_input.draw(&mut pcd8544).unwrap();
                    pcd8544.flush().unwrap();
                }
            },
            async {
                reader.run(false, &mut usb::MultitapHandler {}).await;
            }
        )
    ).await;
}
