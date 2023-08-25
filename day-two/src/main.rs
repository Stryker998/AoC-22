mod part1;
mod part2;

use std::fs;

fn main() {
    let string = fs::read_to_string("input.txt").unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // let score = part1::solution(&string);
    let score = part2::solution(&string);
    println!("{}", score);
}
