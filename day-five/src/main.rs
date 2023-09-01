mod part1;
mod part2;

use std::fs;

fn main() {
    let string = fs::read_to_string("input.txt").expect("Should haver read from input.txt");
    
    let part1_solution: String = part1::solution(&string);
    println!("{}", part1_solution);
    let part2_solution: String = part2::solution(&string);
    println!("{}", part2_solution);


}
