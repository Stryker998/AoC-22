use std::collections::HashSet;

pub fn solution(string: &String) -> u32 {
    let mut counter: u32 = 0;
    for line in string.lines() {
        let assignments: Vec<&str> = line.split(',').collect();
        
        let mut first_elf: HashSet<u32> = HashSet::new();
        let mut second_elf: HashSet<u32> = HashSet::new();

        let first_limits: Vec<&str> = assignments[0].split('-').collect();
        let second_limits: Vec<&str> = assignments[1].split('-').collect();

        let lower_first = first_limits[0].parse::<u32>().expect("Lower first limit should be a valid u32");
        let upper_first = first_limits[1].parse::<u32>().expect("Upper first limit should be a valid u32");
        let lower_second = second_limits[0].parse::<u32>().expect("Lower second limit should be a valid u32");
        let upper_second = second_limits[1].parse::<u32>().expect("Upper second limit should be a valid u32");

        for i in lower_first..(upper_first + 1) {
            first_elf.insert(i);
        };
        for i in lower_second..(upper_second + 1) {
            second_elf.insert(i);
        };

        if first_elf.is_subset(&second_elf) {
            counter += 1;
        } else if first_elf.is_superset(&second_elf) {
            counter += 1;
        }
    };
    counter
}