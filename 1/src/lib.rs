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

pub struct Position {
    pub direction: Direction,
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new() -> Position {
        Position{direction: Direction::North, x: 0, y:0}
    }

    pub fn apply_command(&mut self, command: Command) {
        self.direction.turn(command.turn);
        self.walk(command.distance);
    }

    pub fn walk(&mut self, distance: i32) {
        match self.direction {
            Direction::North => self.y += distance,
            Direction::East => self.x += distance,
            Direction::South => self.y -= distance,
            Direction::West => self.x -= distance,
        }
    }

    pub fn distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

pub fn puzzle(input: &str) -> i32 {
    let mut position = Position::new();
    for command in input.split(',') {
        let command: Command = command.trim().parse().expect("parsing failed");
        position.apply_command(command);
    }
    position.distance()
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
        let position = puzzle(input);
        assert_eq!(5, position);
    }

    #[test]
    fn test_sample_input_2() {
        let input = "R2, R2, R2";
        let position = puzzle(input);
        assert_eq!(2, position);
    }

    #[test]
    fn test_sample_input_3() {
        let input = "R5, L5, R5, R3";
        let position = puzzle(input);
        assert_eq!(12, position);
    }
}
