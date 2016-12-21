extern crate puzzle1;

use puzzle1::{puzzle_part1, puzzle_part2};

fn main() {
    let input = include_str!("../input.txt");
    println!("part one: {}", puzzle_part1(input));
    println!("part two: {}", puzzle_part2(input));
}
