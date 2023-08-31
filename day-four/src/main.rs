mod part1;
mod part2;

use std::fs;

fn main() {
    let string = fs::read_to_string("input.txt").unwrap_or_else(|e| {
        panic!("{e}");
    });

    let part1_solution = part1::solution(&string);
    println!("part1_solution: {}", part1_solution);

    let part2_solution = part2::solution(&string);
    println!("part2_solution: {}", part2_solution);


}
