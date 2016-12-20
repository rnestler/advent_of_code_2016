extern crate puzzle20;

use puzzle20::Blacklist;

fn main() {
    let input = include_str!("../input.txt");
    let blacklist: Blacklist = input.parse().expect("Could not parse input");

    let lowest_ip = blacklist.lowest_ip();
    println!("part one: {}", lowest_ip);

    let number_of_allowed_ips = blacklist.number_of_allowed_ips();
    println!("part two: {}", number_of_allowed_ips);
}
