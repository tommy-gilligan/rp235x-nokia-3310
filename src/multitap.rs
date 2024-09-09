use defmt::Format;
use embassy_futures::select::{select, Either};
use embassy_futures::select::{select4, Either4};
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Input, Output};

#[derive(Debug, PartialEq, Format, Copy, Clone)]
pub enum Button {
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

#[derive(Debug, PartialEq, Format, Copy, Clone)]
pub enum ButtonEvent {
    Down(Button),
    Up(Button),
    None,
}

pub struct Matrix<'a> {
    last_event: ButtonEvent,
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
            last_event: ButtonEvent::None,
            row_a,
            row_b,
            row_c,
            row_d,
            col_a,
            col_b,
            col_c,
        }
    }

    pub async fn event(&mut self) -> ButtonEvent {
        let mut result = ButtonEvent::None;

        match self.last_event {
            ButtonEvent::Down(b @ Button::One | b @ Button::Two | b @ Button::Three) => {
                self.row_a.wait_for_low().await;
                self.last_event = ButtonEvent::Up(b);
                return self.last_event;
            }
            ButtonEvent::Down(b @ Button::Four | b @ Button::Five | b @ Button::Six) => {
                self.row_b.wait_for_low().await;
                self.last_event = ButtonEvent::Up(b);
                return self.last_event;
            }
            ButtonEvent::Down(b @ Button::Seven | b @ Button::Eight | b @ Button::Nine) => {
                self.row_c.wait_for_low().await;
                self.last_event = ButtonEvent::Up(b);
                return self.last_event;
            }
            ButtonEvent::Down(b @ Button::Asterisk | b @ Button::Zero | b @ Button::Hash) => {
                self.row_d.wait_for_low().await;
                self.last_event = ButtonEvent::Up(b);
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

                if self.row_a.is_high() && self.last_event != ButtonEvent::Down(Button::One) {
                    self.last_event = ButtonEvent::Down(Button::One);
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_a.is_high() && self.last_event != ButtonEvent::Down(Button::Two) {
                    self.last_event = ButtonEvent::Down(Button::Two);
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_a.is_high() && self.last_event != ButtonEvent::Down(Button::Three) {
                    self.last_event = ButtonEvent::Down(Button::Three);
                    result = self.last_event;
                }
            }
            Either4::Second(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_b.is_high() && self.last_event != ButtonEvent::Down(Button::Four) {
                    self.last_event = ButtonEvent::Down(Button::Four);
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_b.is_high() && self.last_event != ButtonEvent::Down(Button::Five) {
                    self.last_event = ButtonEvent::Down(Button::Five);
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_b.is_high() && self.last_event != ButtonEvent::Down(Button::Six) {
                    self.last_event = ButtonEvent::Down(Button::Six);
                    result = self.last_event;
                }
            }
            Either4::Third(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_c.is_high() && self.last_event != ButtonEvent::Down(Button::Seven) {
                    self.last_event = ButtonEvent::Down(Button::Seven);
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_c.is_high() && self.last_event != ButtonEvent::Down(Button::Eight) {
                    self.last_event = ButtonEvent::Down(Button::Eight);
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_c.is_high() && self.last_event != ButtonEvent::Down(Button::Nine) {
                    self.last_event = ButtonEvent::Down(Button::Nine);
                    result = self.last_event;
                }
            }
            Either4::Fourth(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_d.is_high() && self.last_event != ButtonEvent::Down(Button::Asterisk) {
                    self.last_event = ButtonEvent::Down(Button::Asterisk);
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_d.is_high() && self.last_event != ButtonEvent::Down(Button::Zero) {
                    self.last_event = ButtonEvent::Down(Button::Zero);
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_d.is_high() && self.last_event != ButtonEvent::Down(Button::Hash) {
                    self.last_event = ButtonEvent::Down(Button::Hash);
                    result = self.last_event;
                }
            }
        }

        self.col_a.set_high();
        self.col_b.set_high();

        result
    }
}

use core::ascii::Char;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MultiTapEvent {
    Tentative(Char),
    Decided(Char),
}

fn button_to_char(button: Button) -> Char {
    match button {
        Button::One => Char::Digit1,
        Button::Two => Char::CapitalA,
        Button::Three => Char::CapitalD,
        Button::Four => Char::CapitalG,
        Button::Five => Char::CapitalJ,
        Button::Six => Char::CapitalM,
        Button::Seven => Char::CapitalP,
        Button::Eight => Char::CapitalT,
        Button::Nine => Char::CapitalW,
        Button::Asterisk => Char::Asterisk,
        Button::Zero => Char::Space,
        Button::Hash => Char::NumberSign,
    }
}

fn next_char(c: Char) -> Char {
    match c {
        Char::CapitalA => Char::CapitalB,
        Char::CapitalB => Char::CapitalC,
        Char::CapitalC => Char::CapitalA,
        Char::CapitalD => Char::CapitalE,
        Char::CapitalE => Char::CapitalF,
        Char::CapitalF => Char::CapitalD,
        Char::CapitalG => Char::CapitalH,
        Char::CapitalH => Char::CapitalI,
        Char::CapitalI => Char::CapitalG,
        Char::CapitalJ => Char::CapitalK,
        Char::CapitalK => Char::CapitalL,
        Char::CapitalL => Char::CapitalJ,
        Char::CapitalM => Char::CapitalN,
        Char::CapitalN => Char::CapitalO,
        Char::CapitalO => Char::CapitalM,
        Char::CapitalP => Char::CapitalQ,
        Char::CapitalQ => Char::CapitalR,
        Char::CapitalR => Char::CapitalS,
        Char::CapitalS => Char::CapitalP,
        Char::CapitalT => Char::CapitalU,
        Char::CapitalU => Char::CapitalV,
        Char::CapitalV => Char::CapitalT,
        Char::CapitalW => Char::CapitalX,
        Char::CapitalX => Char::CapitalY,
        Char::CapitalY => Char::CapitalZ,
        Char::CapitalZ => Char::CapitalW,
        Char::Digit1 => Char::Digit2,
        Char::Digit2 => Char::Digit3,
        Char::Digit3 => Char::Digit4,
        Char::Digit4 => Char::Digit5,
        Char::Digit5 => Char::Digit6,
        Char::Digit6 => Char::Digit7,
        Char::Digit7 => Char::Digit8,
        Char::Digit8 => Char::Digit9,
        Char::Digit9 => Char::Digit0,
        Char::Digit0 => Char::Digit1,
        e => e,
    }
}

pub struct MultiTap<'a>(
    Matrix<'a>,
    Option<MultiTapEvent>,
    Option<Button>,
    Option<MultiTapEvent>,
);

impl<'a> MultiTap<'a> {
    pub fn new(matrix: Matrix<'a>) -> Self {
        Self(matrix, None, None, None)
    }

    pub async fn event(&mut self) -> Option<MultiTapEvent> {
        if let Some(c) = self.3 {
            let result = c;
            self.3 = None;
            return Some(result);
        }

        match select(Timer::after_secs(2), self.0.event()).await {
            Either::First(_) => match self.1 {
                Some(MultiTapEvent::Tentative(e)) => {
                    self.1 = None;
                    self.2 = None;
                    Some(MultiTapEvent::Decided(e))
                }
                None => {
                    self.1 = None;
                    self.2 = None;
                    None
                }
                Some(MultiTapEvent::Decided(_)) => None,
            },
            Either::Second(ButtonEvent::Down(e)) => {
                if let Some(p) = self.2
                    && p != e
                {
                    match self.1 {
                        Some(MultiTapEvent::Tentative(f)) => {
                            self.1 = Some(MultiTapEvent::Tentative(button_to_char(e)));
                            self.2 = Some(e);

                            self.3 = Some(MultiTapEvent::Tentative(button_to_char(e)));

                            Some(MultiTapEvent::Decided(f))
                        }
                        _ => None,
                    }
                } else {
                    self.2 = Some(e);

                    self.1 = match self.1 {
                        Some(MultiTapEvent::Tentative(c)) => {
                            Some(MultiTapEvent::Tentative(next_char(c)))
                        }
                        Some(MultiTapEvent::Decided(_)) => None,
                        None => Some(MultiTapEvent::Tentative(button_to_char(e))),
                    };

                    self.1
                }
            }
            _ => None,
        }
    }
}
