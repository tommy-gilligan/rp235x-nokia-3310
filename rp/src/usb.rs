// use core::ascii::Char;
use embassy_usb::{
    class::hid::{HidReader, HidReaderWriter, HidWriter},
    driver::Driver,
    Builder, Config, UsbDevice,
};
use usbd_hid::descriptor::{KeyboardReport, KeyboardUsage, SerializedDescriptor};

mod handler;
pub use handler::*;

const fn config<'a>() -> Config<'a> {
    let mut config = Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("HID keyboard example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;
    config
}

pub fn new<'a, D>(
    config_descriptor: &'a mut [u8],
    bos_descriptor: &'a mut [u8],
    msos_descriptor: &'a mut [u8],
    control_buf: &'a mut [u8],
    driver: D,
    state: &'a mut embassy_usb::class::hid::State<'a>,
    device_handler: &'a mut MultiTapKeyboard,
) -> (UsbDevice<'a, D>, (HidReader<'a, D, 1>, HidWriter<'a, D, 8>))
where
    D: Driver<'a>,
{
    let mut builder = Builder::new(
        driver,
        config(),
        config_descriptor,
        bos_descriptor,
        msos_descriptor,
        control_buf,
    );

    builder.handler(device_handler);

    let hid = HidReaderWriter::<'a, D, 1, 8>::new(
        &mut builder,
        state,
        embassy_usb::class::hid::Config {
            report_descriptor: KeyboardReport::desc(),
            request_handler: None,
            poll_ms: 60,
            max_packet_size: 64,
        },
    );

    (builder.build(), hid.split())
}

// pub fn char_to_keycode(event: Char) -> KeyboardUsage {
//     match event {
//         Char::CapitalA => KeyboardUsage::KeyboardAa,
//         Char::CapitalB => KeyboardUsage::KeyboardBb,
//         Char::CapitalC => KeyboardUsage::KeyboardCc,
//         Char::CapitalD => KeyboardUsage::KeyboardDd,
//         Char::CapitalE => KeyboardUsage::KeyboardEe,
//         Char::CapitalF => KeyboardUsage::KeyboardFf,
//         Char::CapitalG => KeyboardUsage::KeyboardGg,
//         Char::CapitalH => KeyboardUsage::KeyboardHh,
//         Char::CapitalI => KeyboardUsage::KeyboardIi,
//         Char::CapitalJ => KeyboardUsage::KeyboardJj,
//         Char::CapitalK => KeyboardUsage::KeyboardKk,
//         Char::CapitalL => KeyboardUsage::KeyboardLl,
//         Char::CapitalM => KeyboardUsage::KeyboardMm,
//         Char::CapitalN => KeyboardUsage::KeyboardNn,
//         Char::CapitalO => KeyboardUsage::KeyboardOo,
//         Char::CapitalP => KeyboardUsage::KeyboardPp,
//         Char::CapitalQ => KeyboardUsage::KeyboardQq,
//         Char::CapitalR => KeyboardUsage::KeyboardRr,
//         Char::CapitalS => KeyboardUsage::KeyboardSs,
//         Char::CapitalT => KeyboardUsage::KeyboardTt,
//         Char::CapitalU => KeyboardUsage::KeyboardUu,
//         Char::CapitalV => KeyboardUsage::KeyboardVv,
//         Char::CapitalW => KeyboardUsage::KeyboardWw,
//         Char::CapitalX => KeyboardUsage::KeyboardXx,
//         Char::CapitalY => KeyboardUsage::KeyboardYy,
//         Char::CapitalZ => KeyboardUsage::KeyboardZz,
//         Char::Space => KeyboardUsage::KeyboardSpacebar,
//         _ => KeyboardUsage::KeyboardQq,
//     }
// }
