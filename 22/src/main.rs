extern crate puzzle22;

use puzzle22::*;

fn main() {
    let input = include_str!("../input.txt");
    let viable_pairs = puzzle(input);
    println!("{}", viable_pairs);
}
