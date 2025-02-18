use core::cell::RefCell;
use display_interface_spi::SPIInterface;
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_rp::spi::Blocking;
use embassy_rp::spi::Spi;
use embassy_rp::spi;
use embassy_sync::mutex::Mutex;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_time::Delay;
use embedded_graphics::pixelcolor::BinaryColor;
use embassy_sync::blocking_mutex;
use core::convert::Infallible;
use embedded_graphics::draw_target::DrawTarget;

use embassy_rp::{
    gpio::{Level, Output},
    peripherals::{
        SPI0,
        PIN_33,
        PIN_36,
        PIN_37,
        PIN_38,
        PIN_39,
        PIN_20,
    },
};

pub struct Display(
    // pcd8544::Driver<
    //     SPIInterface<embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig<'a, NoopRawMutex, embassy_rp::spi::Spi<'a, SPI0, embassy_rp::spi::Blocking>, Output<'a>>, Output<'a>>, Output<'a>, core::convert::Infallible
    // >
    embassy_sync::blocking_mutex::Mutex<(), RefCell<embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>>>
);

impl Display {
    pub fn new(
        spi0: SPI0,
        reset: PIN_33,
        dc: PIN_36,
        display_cs: PIN_37,
        clk: PIN_38,
        mosi: PIN_39,
        miso: PIN_20,
    ) -> Self {
        let mut display_config = spi::Config::default();
        display_config.frequency = 4_000_000;
        let spi: Spi<'_, _, Blocking> =
            Spi::new_blocking(spi0, clk, mosi, miso, display_config.clone());
        let spi_bus: embassy_sync::blocking_mutex::Mutex<(), RefCell<embassy_rp::spi::Spi<'_, SPI0, embassy_rp::spi::Blocking>>> = embassy_sync::blocking_mutex::Mutex::new(RefCell::new(spi));

        // let display_spi = SpiDeviceWithConfig::new(
        //     &spi_bus,
        //     Output::new(display_cs, Level::High),
        //     display_config,
        // );
        // let mut pcd8544 = pcd8544::Driver::new(
        //     SPIInterface::new(display_spi, Output::new(dc, Level::High)),
        //     Output::new(reset, Level::High),
        // );
        // pcd8544.init(&mut Delay).unwrap();
        // pcd8544.set_contrast(64).unwrap();
        // pcd8544.invert_display(true).unwrap();
        // pcd8544.clear(BinaryColor::Off).unwrap();

        Self(spi_bus)
    }
}
