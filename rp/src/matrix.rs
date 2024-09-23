use app::keypad::{Button, Keypad};
use embassy_futures::select::{select4, Either4};
use embassy_rp::gpio;
use embassy_time::Timer;
use gpio::{Input, Output};

pub struct Matrix<'a> {
    last_event: Option<app::keypad::Event<Button>>,
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

    async fn base_event(&mut self) -> Option<app::keypad::Event<Button>> {
        let mut result = None;

        match self.last_event {
            Some(app::keypad::Event::Down(
                b @ Button::One | b @ Button::Two | b @ Button::Three,
            )) => {
                self.row_a.wait_for_low().await;
                self.last_event = Some(app::keypad::Event::Up(b));
                return self.last_event;
            }
            Some(app::keypad::Event::Down(
                b @ Button::Four | b @ Button::Five | b @ Button::Six,
            )) => {
                self.row_b.wait_for_low().await;
                self.last_event = Some(app::keypad::Event::Up(b));
                return self.last_event;
            }
            Some(app::keypad::Event::Down(
                b @ Button::Seven | b @ Button::Eight | b @ Button::Nine,
            )) => {
                self.row_c.wait_for_low().await;
                self.last_event = Some(app::keypad::Event::Up(b));
                return self.last_event;
            }
            Some(app::keypad::Event::Down(
                b @ Button::Asterisk | b @ Button::Zero | b @ Button::Hash,
            )) => {
                self.row_d.wait_for_low().await;
                self.last_event = Some(app::keypad::Event::Up(b));
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
                    && self.last_event != Some(app::keypad::Event::Down(Button::One))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::One));
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_a.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Two))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Two));
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_a.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Three))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Three));
                    result = self.last_event;
                }
            }
            Either4::Second(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_b.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Four))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Four));
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_b.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Five))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Five));
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_b.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Six))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Six));
                    result = self.last_event;
                }
            }
            Either4::Third(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_c.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Seven))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Seven));
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_c.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Eight))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Eight));
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_c.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Nine))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Nine));
                    result = self.last_event;
                }
            }
            Either4::Fourth(_) => {
                self.col_b.set_low();
                self.col_c.set_low();
                Timer::after_nanos(10).await;

                if self.row_d.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Asterisk))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Asterisk));
                    result = self.last_event;
                }

                self.col_a.set_low();
                self.col_b.set_high();
                Timer::after_nanos(10).await;

                if self.row_d.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Zero))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Zero));
                    result = self.last_event;
                }

                self.col_b.set_low();
                self.col_c.set_high();
                Timer::after_nanos(10).await;

                if self.row_d.is_high()
                    && self.last_event != Some(app::keypad::Event::Down(Button::Hash))
                {
                    self.last_event = Some(app::keypad::Event::Down(Button::Hash));
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
    async fn event(&mut self) -> app::keypad::Event<Button> {
        loop {
            if let Some(app::keypad::Event::Down(e)) = self.base_event().await {
                return app::keypad::Event::Down(e);
            }
        }
    }
}
