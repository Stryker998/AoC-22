use std::collections::HashSet;

pub fn solution(string: &String) -> i32 {
    let mut sum = 0;
    let mut counter = 1;
    let mut rucksacks = Vec::with_capacity(2);
    for line in string.lines() {
        if counter != 3 {
            let rucksack: HashSet<char> = HashSet::from_iter(line.chars());
            rucksacks.push(rucksack);
            counter += 1;
        } else {
            counter = 1;
            let mut common_char: Option<i32> = Option::None;
            let intersection = rucksacks[0].intersection(&rucksacks[1]);
            for char in intersection {
                if line.contains(*char) {
                    let mut value = *char as i32;
                    if value >= 97 {
                        value -= 96;
                    } else {
                        value -= 38;
                    };
                    common_char = Option::Some(value);
                    break;
                };
            };
            sum += common_char.expect("For loop didn't work");
            rucksacks.clear();
        };
    };
    sum
}