use std::str::FromStr;
use std::error::Error;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Command {
    turn: Turn,
    distance: i32,
}

impl FromStr for Command {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (turn, distance) = s.split_at(1);
        let turn = match turn {
            "R" => Turn::Right,
            "L" => Turn::Left,
            _ => panic!("Unknown Turn: {} in command", turn),
        };
        let distance = distance.parse()?;

        Ok(Command{turn: turn, distance: distance})
    }
}

impl Direction {
    pub fn turn(&mut self, turn: Turn) {
        *self = match turn {
            Turn::Left => self.turn_left(),
            Turn::Right => self.turn_right(),
        };
    }

    pub fn turn_right(self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    pub fn turn_left(self) -> Self {
        use Direction::*;
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub direction: Direction,
    pub x: i32,
    pub y: i32,
    pub visited_positions: Vec<(i32, i32)>,
    pub first_revisited_pos: Option<(i32, i32)>,
}

impl Position {
    pub fn new() -> Position {
        Position {
            direction: Direction::North,
            x: 0, y:0,
            visited_positions: vec![(0,0)],
            first_revisited_pos: None,
        }
    }

    pub fn apply_command(&mut self, command: Command) {
        self.direction.turn(command.turn);
        self.walk(command.distance);
    }

    pub fn walk(&mut self, distance: i32) {
        for _ in 0..distance {
            self.step();
        }
    }

    pub fn step(&mut self) {
        match self.direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        }
        if self.first_revisited_pos.is_none() && self.has_revisited() {
            self.first_revisited_pos = Some((self.x, self.y));
        }
        self.visited_positions.push((self.x, self.y));
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn has_revisited(&self) -> bool {
        self.visited_positions.iter().any(|v| self.compare_x_y(&v))
    }

    pub fn compare_x_y(&self, other: &(i32, i32)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

pub fn puzzle_part1(input: &str) -> i32 {
    let mut position = Position::new();
    for command in input.split(',') {
        let command: Command = command.trim().parse().expect("parsing failed");
        position.apply_command(command);
    }
    position.distance()
}

pub fn puzzle_part2(input: &str) -> i32 {
    let mut position = Position::new();
    for command in input.split(',') {
        let command: Command = command.trim().parse().expect("parsing failed");
        position.apply_command(command);
        // check if we revisit a position
        if let Some(first_revisited_pos) = position.first_revisited_pos {
            return first_revisited_pos.0.abs() + first_revisited_pos.1.abs();
        }
    }
    0
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn turn_right() {
        let direction = Direction::North;
        assert_eq!(direction.turn_right(), Direction::East);
    }

    #[test]
    fn turn_left() {
        let direction = Direction::North;
        assert_eq!(direction.turn_left(), Direction::West);
    }

    #[test]
    fn test_parse_command() {
        let input = "R2";
        let command: Command = input.parse().unwrap();
        assert_eq!(command.turn, Turn::Right);
        assert_eq!(command.distance, 2);
    }

    #[test]
    fn test_sample_input() {
        let input = "R2, L3";
        let position = puzzle_part1(input);
        assert_eq!(5, position);
    }

    #[test]
    fn test_sample_input_2() {
        let input = "R2, R2, R2";
        let position = puzzle_part1(input);
        assert_eq!(2, position);
    }

    #[test]
    fn test_sample_input_3() {
        let input = "R5, L5, R5, R3";
        let position = puzzle_part1(input);
        assert_eq!(12, position);
    }

    #[test]
    fn test_sample_input_part_two() {
        let input = "R8, R4, R4, R8";
        let position = puzzle_part2(input);
        assert_eq!(4, position);
    }
}
