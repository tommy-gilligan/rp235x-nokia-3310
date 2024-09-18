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

pub struct Snake<KEYPAD, DRAW_TARGET>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
{
    keypad: KEYPAD,
    draw_target: DRAW_TARGET,
    world: World
}

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone, PartialEq)]
enum Collision {
    None,
    Food,
    Critter
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

impl<KEYPAD, DRAW_TARGET> Snake<KEYPAD, DRAW_TARGET>
where
    KEYPAD: Keypad,
    DRAW_TARGET: DrawTarget<Color = BinaryColor>,
{
    pub fn new(keypad: KEYPAD, draw_target: DRAW_TARGET) -> Self {
        let world = World::new();
        Self {
            keypad,
            draw_target,
            world
        }
    }

    fn draw(&mut self) {
	for (row_index, row) in self.world.0.into_iter().enumerate() {
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
        if let Cell::Critter(head_direction, _) = self.world.0[row_index].0[column_index] {
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
        let direction = match self.keypad.event().await {
            Event::Down(Button::Two) => Direction::Up,
            Event::Down(Button::Four) => Direction::Left,
            Event::Down(Button::Six) => Direction::Right,
            Event::Down(Button::Eight) => Direction::Down,
            _ => {
                let head_index = self.world.1;
                if let Cell::Critter(head_direction, _) = self.world.0[head_index.0].0[head_index.1] {
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

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Empty,
    Critter(Direction, bool),
    Food
}

impl Default for Cell {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Copy, Clone, PartialEq)]
struct Row([Cell; 20]);

impl Default for Row {
    fn default() -> Self {
        Self([Cell::default(); 20])
    }
}

// head then tail
struct World([Row; 9], (usize, usize), (usize, usize), usize);

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

impl World {
    // New for populated
    fn new() -> Self {
        let mut world: World = World([Row::default(); 9], (4, 10), (4, 6), 0);

        // keep tuple indices for tail and head
        world.0[4].0[10] = Cell::Critter(Direction::Right, false);
        world.0[4].0[9] = Cell::Critter(Direction::Right, false);
        world.0[4].0[8] = Cell::Critter(Direction::Right, false);
        world.0[4].0[7] = Cell::Critter(Direction::Right, false);
        world.0[4].0[6] = Cell::Critter(Direction::Right, false);

        world.0[4].0[13] = Cell::Food;
        world
    }

    fn detect_collision(&self) -> Collision {
        let head_index = self.1;
        if let Cell::Critter(head_direction, _) = self.0[head_index.0].0[head_index.1] {
            let next_index = self.neighbour(self.1, head_direction);
            match self.0[next_index.0].0[next_index.1] {
                Cell::Critter(_, _) => Collision::Critter,
                Cell::Food => Collision::Food,
                Cell::Empty => Collision::None
            }
        } else {
            Collision::None
        }
    }

    fn die(&mut self) {
    }

    fn eat(&mut self) {
    }

    fn update(&mut self, direction: Direction) {
        match self.detect_collision() {
            Collision::Critter => self.die(),
            Collision::Food => {
                self.update_head(direction, true);
                self.update_tail();
            },
            Collision::None => {
                self.update_head(direction, false);
                self.update_tail();
            }
        }
    }

    fn update_head(&mut self, new_direction: Direction, food: bool) {
        // reject opposite direction
        let head_index = self.1;
        if let Cell::Critter(head_direction, _) = self.0[head_index.0].0[head_index.1] {
            let new_head_index = self.neighbour(head_index, head_direction);
            self.1 = new_head_index;
            self.0[new_head_index.0].0[new_head_index.1] = Cell::Critter(
                if new_direction.opposite() == head_direction {
                    head_direction
                } else {
                    new_direction
                },
                food
            );
        }
    }

    fn update_tail(&mut self) {
        let tail_index = self.2;
        if let Cell::Critter(tail_direction, _) = self.0[tail_index.0].0[tail_index.1] {
            let new_tail_index = self.neighbour(tail_index, tail_direction);
            self.2 = new_tail_index;
            self.0[tail_index.0].0[tail_index.1] = Cell::Empty;
        }
    }

    fn neighbour(&self, location: (usize, usize), direction: Direction) -> (usize, usize) {
        match direction {
            Direction::Up => {
                if location.0 == 0 {
                    (self.0.len() - 1, location.1)
                } else {
                    (location.0 - 1, location.1)
                }
            },
            Direction::Down => {
                if location.0 == (self.0.len() - 1) {
                    (0, location.1)
                } else {
                    (location.0 + 1, location.1)
                }
            },
            Direction::Right => {
                if location.1 == (self.0[0].0.len() - 1) {
                    (location.0, 0)
                } else {
                    (location.0, location.1 + 1)
                }
            },
            Direction::Left => {
                if location.1 == 0 {
                    (location.0, self.0[0].0.len() - 1)
                } else {
                    (location.0, location.1 - 1)
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neighbour() {
        let world = World::default();
        
        // indices are row, column
        // middle-ish
        assert_eq!(
            world.neighbour(
                (4, 10),
                Direction::Up
            ),
	    (3, 10),
        );
        assert_eq!(
            world.neighbour(
                (4, 10),
                Direction::Left
            ),
	    (4, 9),
        );
        assert_eq!(
            world.neighbour(
                (4, 10),
                Direction::Down
            ),
	    (5, 10),
        );
        assert_eq!(
            world.neighbour(
                (4, 10),
                Direction::Right
            ),
	    (4, 11),
        );

        // left edge
        assert_eq!(
            world.neighbour(
                (4, 0),
                Direction::Up
            ),
	    (3, 0),
        );
        assert_eq!(
            world.neighbour(
                (4, 0),
                Direction::Left
            ),
	    (4, 19),
        );
        assert_eq!(
            world.neighbour(
                (4, 0),
                Direction::Down
            ),
	    (5, 0),
        );
        assert_eq!(
            world.neighbour(
                (4, 0),
                Direction::Right
            ),
	    (4, 1),
        );

        // right edge
        assert_eq!(
            world.neighbour(
                (4, 19),
                Direction::Up
            ),
	    (3, 19),
        );
        assert_eq!(
            world.neighbour(
                (4, 19),
                Direction::Left
            ),
	    (4, 18),
        );
        assert_eq!(
            world.neighbour(
                (4, 19),
                Direction::Down
            ),
	    (5, 19),
        );
        assert_eq!(
            world.neighbour(
                (4, 19),
                Direction::Right
            ),
	    (4, 0),
        );

        // top edge
        assert_eq!(
            world.neighbour(
                (0, 10),
                Direction::Up
            ),
	    (8, 10),
        );
        assert_eq!(
            world.neighbour(
                (0, 10),
                Direction::Left
            ),
	    (0, 9),
        );
        assert_eq!(
            world.neighbour(
                (0, 10),
                Direction::Down
            ),
	    (1, 10),
        );
        assert_eq!(
            world.neighbour(
                (0, 10),
                Direction::Right
            ),
	    (0, 11),
        );

        // bottom edge
        assert_eq!(
            world.neighbour(
                (8, 10),
                Direction::Up
            ),
	    (7, 10),
        );
        assert_eq!(
            world.neighbour(
                (8, 10),
                Direction::Left
            ),
	    (8, 9),
        );
        assert_eq!(
            world.neighbour(
                (8, 10),
                Direction::Down
            ),
	    (0, 10),
        );
        assert_eq!(
            world.neighbour(
                (8, 10),
                Direction::Right
            ),
	    (8, 11),
        );
    }
}
