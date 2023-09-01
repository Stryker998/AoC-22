use std::fs;

mod part1;
mod part2;
fn main() {
    let string = fs::read_to_string("input.txt").unwrap();
    let part1_solution: usize = part1::solution(&string);
    println!("part 1: {}", part1_solution);
    let part2_solution = part2::solution(&string);
    println!("part 2: {}", part2_solution);

}
