#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
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

#[derive(Copy, Clone, PartialEq)]
pub enum Collision {
    None,
    Food,
    Critter
}

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
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
pub struct Row(pub [Cell; 20]);

impl Default for Row {
    fn default() -> Self {
        Self([Cell::default(); 20])
    }
}

pub struct Grid(pub [Row; 9]);

impl Grid {
    pub fn new() -> Self {
        let mut grid: Grid = Grid([Row::default(); 9]);

        // keep tuple indices for tail and head
        grid.0[4].0[10] = Cell::Critter(Direction::Right, false);
        grid.0[4].0[9] = Cell::Critter(Direction::Right, false);
        grid.0[4].0[8] = Cell::Critter(Direction::Right, false);
        grid.0[4].0[7] = Cell::Critter(Direction::Right, false);
        grid.0[4].0[6] = Cell::Critter(Direction::Right, false);

        grid.0[4].0[13] = Cell::Food;
        grid
    }

    pub fn neighbour_index(&self, location: (usize, usize), direction: Direction) -> (usize, usize) {
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

pub struct World(pub Grid, pub (usize, usize), pub (usize, usize));

impl World {
    pub fn new() -> Self {
        World(Grid::new(), (4, 10), (4, 6))
    }

    pub fn detect_collision(&self) -> Collision {
        let head_index = self.1;
        if let Cell::Critter(head_direction, _) = self.0.0[head_index.0].0[head_index.1] {
            let next_index = self.0.neighbour_index(self.1, head_direction);
            match self.0.0[next_index.0].0[next_index.1] {
                Cell::Critter(_, _) => Collision::Critter,
                Cell::Food => Collision::Food,
                Cell::Empty => Collision::None
            }
        } else {
            Collision::None
        }
    }

    pub fn update(&mut self, direction: Direction) {
        match self.detect_collision() {
            Collision::Critter => { },
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

    pub fn update_head(&mut self, new_direction: Direction, food: bool) {
        // reject opposite direction
        let head_index = self.1;
        if let Cell::Critter(head_direction, _) = self.0.0[head_index.0].0[head_index.1] {
            let new_head_index = self.0.neighbour_index(head_index, head_direction);
            self.1 = new_head_index;
            self.0.0[new_head_index.0].0[new_head_index.1] = Cell::Critter(
                if new_direction.opposite() == head_direction {
                    head_direction
                } else {
                    new_direction
                },
                food
            );
        }
    }

    pub fn update_tail(&mut self) {
        let tail_index = self.2;
        if let Cell::Critter(tail_direction, _) = self.0.0[tail_index.0].0[tail_index.1] {
            let new_tail_index = self.0.neighbour_index(tail_index, tail_direction);
            self.2 = new_tail_index;
            self.0.0[tail_index.0].0[tail_index.1] = Cell::Empty;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neighbour_index() {
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
