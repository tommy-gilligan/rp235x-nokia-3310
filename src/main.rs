#![no_std]
#![no_main]

use core::{
    sync::atomic::{AtomicBool, Ordering},
    cell::RefCell
};
use defmt::*;
use display_interface_spi::SPIInterface;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_rp::{bind_interrupts, gpio, peripherals::USB, spi::Spi, spi, spi::Blocking, usb::{Driver, InterruptHandler}};
use embassy_time::Timer;
use embassy_usb::{
    class::hid::{HidReaderWriter, ReportId, RequestHandler, State},
    control::OutResponse,
    Builder, Config, Handler
};
use gpio::{Input, Level, Output, Pull};
use pcd8544::Driver as PCD8544;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};
use {defmt_rtt as _, panic_probe as _};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDeviceWithConfig;
use embassy_sync::blocking_mutex::{raw::NoopRawMutex, Mutex};
use embassy_embedded_hal::shared_bus::blocking::spi::SpiDevice;
use embedded_hal_1::spi::{Operation, SpiDevice as OtherSpiDevice};
use embassy_time::Delay;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::Circle;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::Primitive;
use embedded_graphics::Drawable;

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

const LCDWIDTH: usize = 84;  ///< LCD is 84 pixels wide
const LCDHEIGHT: usize = 48; ///< 48 pixels high
const PCD8544_POWERDOWN: u8 = 0x04; ///< Function set, Power down mode
const PCD8544_ENTRYMODE: u8 = 0x02; ///< Function set, Entry mode
const PCD8544_EXTENDEDINSTRUCTION: u8 = 0x01;
const PCD8544_DISPLAYBLANK: u8 = 0x0;    ///< Display control, blank
const PCD8544_DISPLAYNORMAL: u8 = 0x4;   ///< Display control, normal mode
const PCD8544_DISPLAYALLON: u8 = 0x1;    ///< Display control, all segments on
const PCD8544_DISPLAYINVERTED: u8 = 0x5; ///< Display control, inverse mode
const PCD8544_FUNCTIONSET: u8 = 0x20; ///< Basic instruction set
const PCD8544_DISPLAYCONTROL: u8 = 0x08; ///< Basic instruction set - Set display configuration
const PCD8544_SETYADDR: u8 = 0x40; ///< Basic instruction set - Set Y address of RAM, 0 <= Y <= 5
const PCD8544_SETXADDR: u8 = 0x80; ///< Basic instruction set - Set X address of RAM, 0 <= X <= 83
const PCD8544_SETTEMP: u8 = 0x04; ///< Extended instruction set - Set temperature coefficient
const PCD8544_SETBIAS: u8 = 0x10; ///< Extended instruction set - Set bias system
const PCD8544_SETVOP: u8 = 0x80; ///< Extended instruction set - Write Vop to register

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

    let driver = Driver::new(p.USB, Irqs);

    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("HID keyboard example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut request_handler = MyRequestHandler {};
    let mut device_handler = MyDeviceHandler::new();

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    builder.handler(&mut device_handler);

    let mut keypad = embassy_keypad::Keypad::new(
        Input::new(p.PIN_4, Pull::Down),
        Input::new(p.PIN_11, Pull::Down),
        Input::new(p.PIN_9, Pull::Down),
        Input::new(p.PIN_10, Pull::Down),
        Input::new(p.PIN_8, Pull::Down),
        Input::new(p.PIN_7, Pull::Down),
        Input::new(p.PIN_22, Pull::Down),
        Input::new(p.PIN_6, Pull::Down),
        Input::new(p.PIN_5, Pull::Down),
        Input::new(p.PIN_27, Pull::Down),
        Input::new(p.PIN_2, Pull::Down),
        Input::new(p.PIN_3, Pull::Down),
        Input::new(p.PIN_28, Pull::Down),
        // Input::new(p.PIN_21, Pull::Down),
        // allows pins 0 and 1 to be used for serial debugging
        Input::new(p.PIN_12, Pull::Down),
        Input::new(p.PIN_13, Pull::Down),
        Input::new(p.PIN_26, Pull::Down),
    );

    // Create classes on the builder.
    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };
    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state, config);

    let mut usb = builder.build();
    let usb_fut = usb.run();

    let (reader, mut writer) = hid.split();

    let in_fut = async {
        loop {
            let scan_code = match keypad.key_down().await {
                embassy_keypad::KeyPress::Select => 0x29,
                embassy_keypad::KeyPress::Cancel => 0x29,
                embassy_keypad::KeyPress::Up => 0x52,
                embassy_keypad::KeyPress::Down => 0x51,
                embassy_keypad::KeyPress::One => 0x1e ,
                embassy_keypad::KeyPress::Two => 0x1f,
                embassy_keypad::KeyPress::Three => 0x20 ,
                embassy_keypad::KeyPress::Four => 0x21 ,
                embassy_keypad::KeyPress::Five => 0x22,
                embassy_keypad::KeyPress::Six => 0x23,
                embassy_keypad::KeyPress::Seven => 0x24,
                embassy_keypad::KeyPress::Eight => 0x25,
                embassy_keypad::KeyPress::Nine => 0x26,
                embassy_keypad::KeyPress::Asterisk => 0x55,
                embassy_keypad::KeyPress::Zero => 0x27,
                embassy_keypad::KeyPress::Hash => 0x32,
            };

            // Create a report with the A key pressed. (no shift modifier)
            let report = KeyboardReport {
                keycodes: [scan_code, 0, 0, 0, 0, 0],
                leds: 0,
                modifier: 0,
                reserved: 0,
            };
            // Send the report.
            match writer.write_serialize(&report).await {
                Ok(()) => {}
                Err(e) => warn!("Failed to send report: {:?}", e),
            };
            let report = KeyboardReport {
                keycodes: [0, 0, 0, 0, 0, 0],
                leds: 0,
                modifier: 0,
                reserved: 0,
            };
            match writer.write_serialize(&report).await {
                Ok(()) => {}
                Err(e) => warn!("Failed to send report: {:?}", e),
            };
        }
    };

    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };

    // Run everything concurrently.
    // If we had made everything `'static` above instead, we could do this using separate tasks instead.
    join(usb_fut, join(in_fut, out_fut)).await;
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!("Device configured, it may now draw up to the configured current limit from Vbus.")
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
