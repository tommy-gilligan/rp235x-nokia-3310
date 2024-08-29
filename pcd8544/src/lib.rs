#![no_std]

use display_interface::{WriteOnlyDataCommand, DataFormat, DisplayError};
use embedded_graphics_core::{
    Pixel,
    draw_target::DrawTarget,
    geometry::{Point, Size, Dimensions},
    pixelcolor::BinaryColor,
    primitives::rectangle::Rectangle
};
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

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

#[derive(Debug)]
pub enum Error<PinE> {
    DisplayError,
    Pin(PinE),
}

pub struct Driver<DI, RST, PinE> where DI: WriteOnlyDataCommand, RST: OutputPin<Error = PinE> {
    display_interface: DI,
    reset: RST,
    buffer: [u8; LCDWIDTH * LCDHEIGHT / 8]
}

impl <DI, RST, PinE>Driver<DI, RST, PinE> where DI: WriteOnlyDataCommand, RST: OutputPin<Error = PinE> {
    pub fn new(mut display_interface: DI, reset: RST) -> Self {
        Self { display_interface, reset, buffer: [0x00; LCDWIDTH * LCDHEIGHT / 8] }
    }

    pub fn init(&mut self, delay_source: &mut impl DelayNs) {
        self.reset.set_low();
        delay_source.delay_us(1);
        self.reset.set_high();

        self.display_interface.send_commands(DataFormat::U8(&[PCD8544_FUNCTIONSET | PCD8544_EXTENDEDINSTRUCTION])).unwrap();
        self.display_interface.send_commands(DataFormat::U8(&[PCD8544_SETBIAS | 0x04])).unwrap();
        self.display_interface.send_commands(DataFormat::U8(&[PCD8544_FUNCTIONSET])).unwrap();
        self.display_interface.send_commands(DataFormat::U8(&[PCD8544_FUNCTIONSET | PCD8544_EXTENDEDINSTRUCTION])).unwrap();
        self.display_interface.send_commands(DataFormat::U8(&[PCD8544_SETVOP | 75])).unwrap();
        self.display_interface.send_commands(DataFormat::U8(&[PCD8544_FUNCTIONSET])).unwrap();
        self.display_interface.send_commands(DataFormat::U8(&[PCD8544_FUNCTIONSET])).unwrap();
        self.display_interface.send_commands(DataFormat::U8(&[PCD8544_DISPLAYCONTROL | PCD8544_DISPLAYNORMAL])).unwrap();
    }

    pub fn flush(&mut self) {
        for page in 0..(LCDHEIGHT / 8) {
          self.display_interface.send_commands(DataFormat::U8(&[PCD8544_SETYADDR | (page as u8)])).unwrap();

          self.display_interface.send_commands(DataFormat::U8(&[PCD8544_SETXADDR])).unwrap();

          self.display_interface.send_data(DataFormat::U8(&self.buffer[
              (LCDWIDTH * page)..((LCDWIDTH * (page + 1)))
          ])).unwrap();
        }
        self.display_interface.send_commands(DataFormat::U8(&[PCD8544_SETYADDR])).unwrap();
    }
}

impl<DI, RST, PinE> DrawTarget for Driver<DI, RST, PinE> where DI: WriteOnlyDataCommand, RST: OutputPin<Error = PinE> {
  type Color = BinaryColor;

  type Error = core::convert::Infallible;

  fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error> where I: IntoIterator<Item = Pixel<Self::Color>> {
    for Pixel(coord, color) in pixels.into_iter() {
        let mask = !(0x01 << (coord.y % 8));

        self.buffer[coord.x as usize + ((coord.y as usize) / 8) * LCDWIDTH] &= mask;

        if color.is_on() {
            self.buffer[coord.x as usize + ((coord.y as usize) / 8) * LCDWIDTH] |= (0x01 << ((coord.y as usize) % 8));
        }
    }

    Ok(())
  }
}

impl<DI, RST, PinE> Dimensions for Driver<DI, RST, PinE> where DI: WriteOnlyDataCommand, RST: OutputPin<Error = PinE> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(
            Point::new(0, 0),
            Size::new(84, 48)
        )
    }
}
