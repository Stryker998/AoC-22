use std::fs;

mod part1;
mod part2;

fn main() {

    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = part1::solution(&input);
    println!("{}", part1);
    let part2 = part2::solution(&input);
    println!("{}", part2);
}

