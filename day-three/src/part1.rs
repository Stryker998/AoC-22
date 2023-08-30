// for line in string.lines() {
//     let half_length = line.len() / 2;
//     let (one, two) = line.split_at(half_length);
//     let compartment_one: HashSet<char> = HashSet::from_iter(one.chars());
//     let compartment_two: HashSet<char> = HashSet::from_iter(two.chars());
//     let intersection = compartment_one.intersection(&compartment_two);

//     let sum: i32 = intersection.map(|char| {
//         let mut value = *char as i32;
//         if value >= 97 {
//             value -= 96;
//         } else {
//             value -= 38;
//         };
//         value
//     }).sum();

//     final_sum += sum;
// }

use std::collections::HashSet;

pub fn solution(string: &String) -> i32 {
    let mut final_sum = 0;
    for line in string.lines() {
        let half_length = line.len() / 2;
        let (one, two) = line.split_at(half_length);
        let compartment_one: HashSet<char> = HashSet::from_iter(one.chars());
        let compartment_two: HashSet<char> = HashSet::from_iter(two.chars());
        let intersection = compartment_one.intersection(&compartment_two);

        let sum: i32 = intersection
            .map(|char| {
                let mut value = *char as i32;
                if value >= 97 {
                    value -= 96;
                } else {
                    value -= 38;
                };
                value
            })
            .sum();

        final_sum += sum;
    }
    final_sum
}
