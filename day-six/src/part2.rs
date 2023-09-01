use std::collections::HashSet;

pub fn solution(string: &String) -> usize {
    let mut solution = 0; 
    let max_limit = string.len();

    for i in 13..max_limit {
        let mut set = HashSet::new();

        for j in 0..14 {
            set.insert(string.get(i - j..i + 1 - j));
        };
        if set.len() == 14 {
            solution = i + 1;
            break;
        }
    };
    solution
}