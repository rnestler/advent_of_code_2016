extern crate puzzle1;

use puzzle1::puzzle;

fn main() {
    let input = include_str!("../input.txt");
    println!("{}", puzzle(input));
}
