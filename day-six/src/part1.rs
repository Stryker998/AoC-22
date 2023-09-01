use std::collections::HashSet;

pub fn solution(string: &String) -> usize{
    let mut solution = 0; 
    let max_limit = string.len();
    
    for i in 3..max_limit {
        let mut set = HashSet::new();
        set.insert(string.get(i..i + 1).unwrap());
        set.insert(string.get(i - 1..i).unwrap());
        set.insert(string.get(i - 2..i - 1).unwrap());
        set.insert(string.get(i - 3..i -  2).unwrap());
        if set.len() == 4 {
            solution = i + 1;
            break;
        }
    };
    solution
}