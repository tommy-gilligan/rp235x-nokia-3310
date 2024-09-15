use defmt::Format;
use embassy_futures::select::{select4, Either4};
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Input, Output};

use core::ascii::Char;
use multi_tap::*;

#[derive(Debug, PartialEq, Format, Copy, Clone)]
pub enum Key {
    Cancel,
    Select,
    Up,
    Down,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Asterisk,
    Zero,
    Hash,
}

impl From<Key> for Char {
    fn from(key: Key) -> Char {
        match key {
            Key::One => Char::Digit1,
            Key::Two => Char::CapitalA,
            Key::Three => Char::CapitalD,
            Key::Four => Char::CapitalG,
            Key::Five => Char::CapitalJ,
            Key::Six => Char::CapitalM,
            Key::Seven => Char::CapitalP,
            Key::Eight => Char::CapitalT,
            Key::Nine => Char::CapitalW,
            Key::Asterisk => Char::Asterisk,
            Key::Zero => Char::Space,
            Key::Hash => Char::NumberSign,
            Key::Cancel => Char::Backspace,
            _ => Char::Space,
        }
    }
}

pub struct Matrix<'a> {
    last_event: Option<multi_tap::keypad::Event<Key>>,
    row_a: Input<'a>,
    row_b: Input<'a>,
    row_c: Input<'a>,
    row_d: Input<'a>,
    col_a: Output<'a>,
    col_b: Output<'a>,
    col_c: Output<'a>,
}

impl<'a> Matrix<'a> {
    pub fn new(
        row_a: Input<'a>,
        row_b: Input<'a>,
        row_c: Input<'a>,
        row_d: Input<'a>,
        mut col_a: Output<'a>,
        mut col_b: Output<'a>,
        mut col_c: Output<'a>,
    ) -> Matrix<'a> {
        col_a.set_high();
        col_b.set_high();
        col_c.set_high();

        Matrix {
            last_event: None,
            row_a,
            row_b,
            row_c,
            row_d,
            col_a,
            col_b,
            col_c,
        }
    }

    async fn base_event(&mut self) -> Option<multi_tap::keypad::Event<Key>> {
        let mut result = None;

        match self.last_event {
            Some(multi_tap::keypad::Event::Down(b @ Key::One | b @ Key::Two | b @ Key::Three)) => {
                self.row_a.wait_for_low().await;
                self.last_event = Some(multi_tap::keypad::Event::Up(b));
                return self.last_event;
            }
            Some(multi_tap::keypad::Event::Down(b @ Key::Four | b @ Key::Five | b @ Key::Six)) => {
                self.row_b.wait_for_low().await;
                self.last_event = Some(multi_tap::keypad::Event::Up(b));
                return self.last_event;
            }
            Some(multi_tap::keypad::Event::Down(
                b @ Key::Seven | b @ Key::Eight | b @ Key::Nine,
            )) => {
                self.row_c.wait_for_low().await;
                self.last_event = Some(multi_tap::keypad::Event::Up(b));
                return self.last_event;
            }
            Some(multi_tap::keypad::Event::Down(
                b @ Key::Asterisk | b @ Key::Zero | b @ Key::Hash,
            )) => {
                self.row_d.wait_for_low().await;
                self.last_event = Some(multi_tap::keypad::Event::Up(b));
                return self.last_event;
            }
            _ => {}
        }

        match select4(
            self.row_a.wait_for_high(),
            self.row_b.wait_for_high(),
            self.row_c.wait_for_high(),
            self.row_d.wait_for_high(),
        )
        .await
        {
            Either4::First(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_a.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::One))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::One));
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_a.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Two))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Two));
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_a.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Three))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Three));
                    result = self.last_event;
                }
            }
            Either4::Second(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_b.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Four))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Four));
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_b.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Five))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Five));
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_b.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Six))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Six));
                    result = self.last_event;
                }
            }
            Either4::Third(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_c.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Seven))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Seven));
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_c.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Eight))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Eight));
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_c.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Nine))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Nine));
                    result = self.last_event;
                }
            }
            Either4::Fourth(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_d.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Asterisk))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Asterisk));
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_d.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Zero))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Zero));
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_d.is_high()
                    && self.last_event != Some(multi_tap::keypad::Event::Down(Key::Hash))
                {
                    self.last_event = Some(multi_tap::keypad::Event::Down(Key::Hash));
                    result = self.last_event;
                }
            }
        }

        self.col_a.set_high();
        self.col_b.set_high();

        result
    }
}

impl<'a> Keypad for Matrix<'a> {
    type Button = Key;

    async fn event(&mut self) -> multi_tap::keypad::Event<Key> {
        loop {
            if let Some(multi_tap::keypad::Event::Down(e)) = self.base_event().await {
                return multi_tap::keypad::Event::Down(e);
            }
        }
    }
}
