extern crate puzzle23;

fn main() {
    let input = include_str!("../input.txt");
    let result = puzzle23::puzzle(input, 7);
    println!("part one: {}", result);

    let result = puzzle23::puzzle(input, 12);
    println!("part two: {}", result);
}
