use crate::keypad::{Keypad, Button, Event};
use crate::text_input::{Model, TextInput};
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::Rectangle;
use embedded_graphics::{
    mono_font::{ascii::FONT_4X6, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use embedded_graphics::Pixel;
use embedded_graphics::geometry::Point;
use embedded_graphics::primitives::Line;
use embedded_graphics::text::TextStyleBuilder;
use embassy_futures::select::{Either, select};
mod model;
use model::Direction;
use model::Cell;

pub struct Snake<KEYPAD, DRAW_TARGET>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
{
    keypad: KEYPAD,
    draw_target: DRAW_TARGET,
    world: model::World
}

impl<KEYPAD, DRAW_TARGET> Snake<KEYPAD, DRAW_TARGET>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
{
    pub fn new(keypad: KEYPAD, draw_target: DRAW_TARGET) -> Self {
        let world = model::World::new();
        Self {
            keypad,
            draw_target,
            world
        }
    }

    fn draw(&mut self) {
	for (row_index, row) in self.world.0.0.into_iter().enumerate() {
            for (column_index, cell) in row.0.into_iter().enumerate() {
                let fill = PrimitiveStyle::with_fill(BinaryColor::On);
                Rectangle::new(
                    Point::new(
                        4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2,
                        4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 10
                    ),
                    Size::new(4, 4)
                ).into_styled(fill).draw(&mut self.draw_target);

                let color = match cell {
                    Cell::Empty => { },
                    Cell::Critter(direction, _) => {
                        match direction {
                            Direction::Left => {
                                Rectangle::new(
                                    Point::new(
                                        4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 0,
                                        4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11
                                    ),
                                    Size::new(4, 2)
                                ).into_styled(
                                    PrimitiveStyle::with_fill(BinaryColor::Off)
                                ).draw(&mut self.draw_target);
                            },
                            Direction::Right => {
                                Rectangle::new(
                                    Point::new(
                                        4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 0,
                                        4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11
                                    ),
                                    Size::new(4, 2)
                                ).into_styled(
                                    PrimitiveStyle::with_fill(BinaryColor::Off)
                                ).draw(&mut self.draw_target);
                            },
                            Direction::Up => {
                                Rectangle::new(
                                    Point::new(
                                        4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2,
                                        4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 9
                                    ),
                                    Size::new(2, 4)
                                ).into_styled(
                                    PrimitiveStyle::with_fill(BinaryColor::Off)
                                ).draw(&mut self.draw_target);
                            },
                            Direction::Down => {
                                Rectangle::new(
                                    Point::new(
                                        4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2,
                                        4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 9
                                    ),
                                    Size::new(2, 4)
                                ).into_styled(
                                    PrimitiveStyle::with_fill(BinaryColor::Off)
                                ).draw(&mut self.draw_target);
                            }
                        }
                        Rectangle::new(
                            Point::new(
                                4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2,
                                4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11
                            ),
                            Size::new(2, 2)
                        ).into_styled(
                            PrimitiveStyle::with_fill(BinaryColor::Off)
                        ).draw(&mut self.draw_target);
                    },
                    Cell::Food => {
                        let _ = Pixel(
                            Point::new(
		        	4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 4i32,
		        	4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 12i32
                            ),
                            BinaryColor::Off
                        ).draw(&mut self.draw_target);
                        let _ = Pixel(
                            Point::new(
		        	4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 4i32,
		        	4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 10i32
                            ),
                            BinaryColor::Off
                        ).draw(&mut self.draw_target);
                        let _ = Pixel(
                            Point::new(
		        	4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 3i32,
		        	4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11i32
                            ),
                            BinaryColor::Off
                        ).draw(&mut self.draw_target);
                        let _ = Pixel(
                            Point::new(
		        	4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 5i32,
		        	4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11i32
                            ),
                            BinaryColor::Off
                        ).draw(&mut self.draw_target);
                    }
                };
            }
        }

        self.draw_border();
        self.draw_score();
        self.draw_head();
    }

    fn draw_head(&mut self) {
        let head_index = self.world.1;
        let row_index = head_index.0;
        let column_index = head_index.1;
        let fill = PrimitiveStyle::with_fill(BinaryColor::Off);
        if let Cell::Critter(head_direction, _) = self.world.0.0[row_index].0[column_index] {
            match head_direction {
                Direction::Left => {
                    Rectangle::new(
                        Point::new(
                            4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2,
                            4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11
                        ),
                        Size::new(4, 2)
                    ).into_styled(fill).draw(&mut self.draw_target);
                    let _ = Pixel(
                        Point::new(
		    	    4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 4i32,
		    	    4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11i32
                        ),
                        BinaryColor::On
                    ).draw(&mut self.draw_target);
                    let _ = Pixel(
                        Point::new(
		    	    4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 4i32,
		    	    4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 10i32
                        ),
                        BinaryColor::Off
                    ).draw(&mut self.draw_target);
                },
                Direction::Right => {
                    Rectangle::new(
                        Point::new(
                            4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2,
                            4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11
                        ),
                        Size::new(4, 2)
                    ).into_styled(fill).draw(&mut self.draw_target);
                    let _ = Pixel(
                        Point::new(
		    	    4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 3i32,
		    	    4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11i32
                        ),
                        BinaryColor::On
                    ).draw(&mut self.draw_target);
                    let _ = Pixel(
                        Point::new(
		    	    4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 3i32,
		    	    4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 10i32
                        ),
                        BinaryColor::Off
                    ).draw(&mut self.draw_target);
                },
                Direction::Down => {
                    Rectangle::new(
                        Point::new(
                            4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2,
                            4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 10
                        ),
                        Size::new(2, 4)
                    ).into_styled(fill).draw(&mut self.draw_target);
                    let _ = Pixel(
                        Point::new(
		    	    4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2i32,
		    	    4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11i32
                        ),
                        BinaryColor::On
                    ).draw(&mut self.draw_target);
                    let _ = Pixel(
                        Point::new(
		    	    4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 1i32,
		    	    4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 11i32
                        ),
                        BinaryColor::Off
                    ).draw(&mut self.draw_target);
                },
                Direction::Up => {
                    Rectangle::new(
                        Point::new(
                            4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2,
                            4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 10
                        ),
                        Size::new(2, 4)
                    ).into_styled(fill).draw(&mut self.draw_target);
                    let _ = Pixel(
                        Point::new(
		    	    4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 2i32,
		    	    4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 12i32
                        ),
                        BinaryColor::On
                    ).draw(&mut self.draw_target);
                    let _ = Pixel(
                        Point::new(
		    	    4 * <usize as TryInto<i32>>::try_into(column_index).unwrap() + 1i32,
		    	    4 * <usize as TryInto<i32>>::try_into(row_index).unwrap() + 12i32
                        ),
                        BinaryColor::Off
                    ).draw(&mut self.draw_target);
                },
            }
        }
    }

    fn draw_score(&mut self) {
        let style = MonoTextStyle::new(&FONT_4X6, BinaryColor::Off);
        let text = Text::new("8056", Point::new(1, 4), style)
            .draw(&mut self.draw_target);
    }

    fn draw_border(&mut self) {
        let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::Off, 1);

        Rectangle::new(
            Point::new(0, 8),
            Size::new(84, 40)
        ).into_styled(thin_stroke).draw(&mut self.draw_target);

        Line::new(Point::new(0, 6), Point::new(83, 6))
            .into_styled(thin_stroke)
            .draw(&mut self.draw_target);
    }

    pub async fn process(&mut self) {
        let event_future = self.keypad.event();
        let timeout_future = embassy_time::Timer::after_millis(100);

        let direction = match embassy_futures::select::select(event_future, timeout_future).await {
            Either::First(Event::Down(Button::Two)) => Direction::Up,
            Either::First(Event::Down(Button::Four)) => Direction::Left,
            Either::First(Event::Down(Button::Six)) => Direction::Right,
            Either::First(Event::Down(Button::Eight)) => Direction::Down,
            _ => {
                let head_index = self.world.1;
                if let Cell::Critter(head_direction, _) = self.world.0.0[head_index.0].0[head_index.1] {
                    head_direction
                } else {
                    Direction::Down
                }
            }
        };
        self.world.update(direction);
        self.draw();
    }
}

fn pattern(x: i32, y: i32, c: BinaryColor) -> BinaryColor {
    if c == BinaryColor::Off {
        if ((y % 3) + x) % 3 == 0 {
            BinaryColor::On
        } else {
            BinaryColor::Off
        }
    } else {
        BinaryColor::On
    }
}
