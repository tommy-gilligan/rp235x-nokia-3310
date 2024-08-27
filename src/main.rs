#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Input, Level, Output, Pull};
use {defmt_rtt as _, panic_probe as _};
use embassy_futures::join::join;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::USB;
use embassy_rp::usb::{Driver, InterruptHandler};
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::{Builder, Config, Handler};
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs);

    // Create embassy-usb Config
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("HID keyboard example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    // You can also add a Microsoft OS descriptor.
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
        Input::new(p.PIN_21, Pull::Down),
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

    // Build the builder.
    let mut usb = builder.build();

    // Run the USB device.
    let usb_fut = usb.run();

    // Set up the signal pin that will be used to trigger the keyboard.
    let mut signal_pin = Input::new(p.PIN_16, Pull::Down);

    // Enable the schmitt trigger to slightly debounce.
    signal_pin.set_schmitt(true);

    let (reader, mut writer) = hid.split();

    // Do stuff with the class!
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
