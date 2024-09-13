#![no_std]

// TODO: documentation
// TODO: test
// TODO: eliminate buffer

use display_interface::{DataFormat, DisplayError, WriteOnlyDataCommand};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    primitives::rectangle::Rectangle,
    Pixel,
};
use embedded_hal::{
    delay::DelayNs,
    digital::OutputPin,
};
use core::cmp;

const WIDTH: usize = 84;
const HEIGHT: usize = 48;
const EXTENDED_INSTRUCTION: u8 = 0x01;
const DISPLAY_NORMAL: u8 = 0x4;
const FUNCTION_SET: u8 = 0x20;
const DISPLAY_CONTROL: u8 = 0x08;
const SET_Y_ADDR: u8 = 0x40;
const SET_X_ADDR: u8 = 0x80;
const SET_BIAS: u8 = 0x10;
const SET_VOP: u8 = 0x80;
const DISPLAY_INVERTED: u8 = 0x5;

#[derive(Debug)]
pub enum Error<PinE> {
    DisplayError,
    Pin(PinE),
}

pub struct Driver<DI, RST, PinE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin<Error = PinE>,
{
    display_interface: DI,
    reset: RST,
    buffer: [u8; WIDTH * HEIGHT >> 3],
}

impl<DI, RST, PinE> Driver<DI, RST, PinE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin<Error = PinE>,
{
    pub fn new(mut display_interface: DI, reset: RST) -> Self {
        Self {
            display_interface,
            reset,
            buffer: [0x00; WIDTH * HEIGHT >> 3],
        }
    }

    pub fn set_bias(&mut self, val: u8) -> Result<(), DisplayError> {
        self.display_interface.send_commands(DataFormat::U8(&[FUNCTION_SET | EXTENDED_INSTRUCTION]))?;
        self.display_interface.send_commands(DataFormat::U8(&[SET_BIAS | cmp::min(0x07, val)]))?;
        self.display_interface.send_commands(DataFormat::U8(&[FUNCTION_SET]))?;
        Ok(())
    }

    pub fn set_contrast(&mut self, val: u8) -> Result<(), DisplayError> {
        self.display_interface.send_commands(DataFormat::U8(&[FUNCTION_SET | EXTENDED_INSTRUCTION]))?;
        self.display_interface.send_commands(DataFormat::U8(&[SET_VOP | cmp::min(val, 0x7f)]))?;
        self.display_interface.send_commands(DataFormat::U8(&[FUNCTION_SET]))?;
        Ok(())
    }

    pub fn invert_display(&mut self, i: bool) -> Result<(), DisplayError> {
      self.display_interface.send_commands(DataFormat::U8(&[FUNCTION_SET]))?;
      if i {
        self.display_interface.send_commands(DataFormat::U8(&[DISPLAY_CONTROL | DISPLAY_INVERTED]))?;
      } else {
        self.display_interface.send_commands(DataFormat::U8(&[DISPLAY_CONTROL | DISPLAY_NORMAL]))?;
      }
      Ok(())
    }

    pub fn init(&mut self, delay_source: &mut impl DelayNs) -> Result<(), DisplayError> {
        let _ = self.reset.set_low();
        delay_source.delay_us(1);
        let _ = self.reset.set_high();

        self.set_bias(0x04)?;
        self.set_contrast(75)?;

        // normal mode
        self.display_interface.send_commands(DataFormat::U8(&[FUNCTION_SET]))?;
        // Set display to Normal
        self.display_interface.send_commands(DataFormat::U8(&[DISPLAY_CONTROL | DISPLAY_NORMAL]))?;
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), DisplayError> {
        for page in 0..(HEIGHT >> 3) {
            self.display_interface.send_commands(DataFormat::U8(&[SET_Y_ADDR | (page as u8)]))?;
            self.display_interface.send_commands(DataFormat::U8(&[SET_X_ADDR]))?;
            self.display_interface.send_data(DataFormat::U8(&self.buffer[(WIDTH * page)..(WIDTH * (page + 1))]))?;
        }
        self.display_interface.send_commands(DataFormat::U8(&[SET_Y_ADDR]))?;
        Ok(())
    }
}

// 83 x 5

impl<DI, RST, PinE> DrawTarget for Driver<DI, RST, PinE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin<Error = PinE>,
{
    type Color = BinaryColor;

    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels.into_iter() {
            let mask = !(0x01 << (coord.y % 8));

            self.buffer[coord.x as usize + ((coord.y as usize) >> 3) * WIDTH] &= mask;

            if color.is_on() {
                self.buffer[coord.x as usize + ((coord.y as usize) >> 3) * WIDTH] |= 0x01 << ((coord.y as usize) % 8);
            }
        }

        Ok(())
    }
}

impl<DI, RST, PinE> Dimensions for Driver<DI, RST, PinE>
where
    DI: WriteOnlyDataCommand,
    RST: OutputPin<Error = PinE>,
{
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(84, 48))
    }
}

#[test]
fn test_draw() {
}
