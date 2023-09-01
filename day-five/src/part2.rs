pub fn solution(string: &String) -> String {
    let mut instructions_start = false;
    let mut stacks_array: [Vec<char>; 9] = [Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for line in string.lines() {
        if line.contains("1") && !instructions_start {
            for i in 0..9 {
                stacks_array[i].reverse();
            };
            instructions_start = true;
            continue;
        };

        if line.chars().count() == 0 {
            continue;
        };

        if !instructions_start {
            let mut counter = 0;
            let mut filter = 1;
            line.char_indices().filter(|(index, _)| {
                if *index == filter {
                    filter += 4;
                    true
                } else {
                    false
                }
            }
        ).for_each(|(_, char)| {
            if char != ' ' {
                stacks_array[counter].push(char);
              };
              counter += 1;  
        });
        } else {
            let mut instructions: Vec<usize> = line.chars().filter(|char| {
                char.is_numeric()
            }).map(|char| {
                char.to_digit(10).expect("Should contain only digits from 0-9") as usize
            }).collect();
            
            let (to, from) = (instructions.pop().unwrap() - 1, instructions.pop().unwrap() - 1);
            let amount = instructions.into_iter().reduce(|acc, num| acc * 10 + num).unwrap();

            let mut stacks: Vec<char> = Vec::new(); 
            for _ in 0..amount {
                let stack = stacks_array[from].pop().unwrap();
                stacks.push(stack);
            };
            stacks.reverse();
            stacks_array[to].extend(stacks.iter());
        };
    };
    let mut remaining_stacks: [char; 9] = [' '; 9];

    for i in 0..9 {
        let stack = stacks_array[i].pop().unwrap();
        remaining_stacks[i] = stack;
    };
    remaining_stacks.iter().collect()
}