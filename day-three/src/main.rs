mod part1;
mod part2;

use std::fs;

fn main() {
    let string = fs::read_to_string("input.txt").unwrap_or_else(|err| {
        panic!("{}", err);
    });
    let sum_part_one = part1::solution(&string);
    println!("part 1 : {}", sum_part_one);

    let sum_part_two = part2::solution(&string);
    println!("part 2 : {}", sum_part_two);
}
