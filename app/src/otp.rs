use embassy_futures::select::Either;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    image::ImageDrawable,
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Line, PrimitiveStyle, Rectangle},
    text::{Alignment, Text},
};
use numtoa::NumToA;
use totp_rs::{Algorithm, Secret, TOTP};

use crate::keypad::{Button, Event, Keypad};

pub struct Otp<KEYPAD, DRAW_TARGET, F>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
    F: Fn() -> u64,
{
    keypad: KEYPAD,
    draw_target: DRAW_TARGET,
    first_draw: bool,
    time_fn: F,
}

impl<KEYPAD, DRAW_TARGET, F> Otp<KEYPAD, DRAW_TARGET, F>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
    F: Fn() -> u64,
{
    pub fn new(keypad: KEYPAD, draw_target: DRAW_TARGET, seed: u64, time_fn: F) -> Self {
        Self {
            keypad,
            draw_target,
            first_draw: true,
            time_fn,
        }
    }

    fn draw(&mut self) {
        self.draw_token();
        self.draw_countdown();
    }

    fn draw_countdown(&mut self) {
        // let width = self.draw_target.bounding_box().size.width;
        // let height = self.draw_target.bounding_box().size.height;
        // let bar_width = ((self.lib.ttl().unwrap() as u32 * width) / 30).try_into().unwrap();

        // let _ = Rectangle::new(
        //     Point::new(0, (height - 10).try_into().unwrap()),
        //     Size::new(width, 10)
        // ).into_styled(PrimitiveStyle::with_fill(BinaryColor::On)).draw(&mut self.draw_target);
        // let _ = Rectangle::new(
        //     Point::new(0, (height - 10).try_into().unwrap()),
        //     Size::new(bar_width, 10)
        // ).into_styled(PrimitiveStyle::with_fill(BinaryColor::Off)).draw(&mut self.draw_target);
    }

    fn draw_token(&mut self) {
        let secret = Secret::Encoded(heapless::String::<64>::try_from("YZ5CTEPRPXKMDO67").unwrap())
            .to_bytes()
            .unwrap();
        let lib = TOTP::new(Algorithm::SHA1, 6, 1, 30, &secret);

        let mut token = heapless::String::<16>::new();
        lib.generate((self.time_fn)(), &mut token);
        let width = self.draw_target.bounding_box().size.width;
        let height = self.draw_target.bounding_box().size.height;

        let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::Off);
        let t = Text::with_alignment(
            &token,
            Point::new(width as i32 / 2, height as i32 / 2),
            style,
            Alignment::Center,
        );
        let fill = PrimitiveStyle::with_fill(BinaryColor::On);
        let _ = t
            .bounding_box()
            .into_styled(fill)
            .draw(&mut self.draw_target);
        let _ = t.draw(&mut self.draw_target);
    }

    pub async fn process(&mut self) {
        self.draw();
        embassy_time::Timer::after_millis(100).await;
    }

    fn release(self) -> DRAW_TARGET {
        self.draw_target
    }
}

// #[cfg(test)]
// mod test {
//     use embedded_graphics::mock_display::MockDisplay;
//
//     use super::*;
//     struct TestKeypad;
//
//     impl Keypad for TestKeypad {
//         async fn event(&mut self) -> Event<Button> {
//             embassy_time::Timer::after_millis(100).await;
//             Event::Down(Button::Down)
//         }
//     }
//
//     #[test]
//     fn test_draw() {
//         let mut display = MockDisplay::new();
//         display.set_allow_out_of_bounds_drawing(true);
//         // TODO: set false to find overdraws
//         display.set_allow_overdraw(true);
//         let mut snake = Otp::new(TestKeypad, display, 0);
//         snake.grid.place_randomly(Cell::Food);
//         snake.draw();
//
//         snake.release().assert_pattern(&[
//             " #.##                                                           ",
//             " .#.#                                                           ",
//             " ...#                                                           ",
//             " .#.#                                                           ",
//             " #.##                                                           ",
//             " ####                                                           ",
//             "................................................................",
//             "                                                                ",
//             "................................................................",
//             ".                                                               ",
//             ". #.############################################################",
//             ". .#.###########################################################",
//             ". #.############################################################",
//             ". ##############################################################",
//             ". #.###.########################################################",
//             ". .#.#.#.#######################################################",
//             ". #.###.########################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". #####################################.########################",
//             ". ####################################.#.#######################",
//             ". #####################################.########################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ". ##############################################################",
//             ".                                                               ",
//             "................................................................",
//         ]);
//     }
// }
