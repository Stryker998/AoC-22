
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

        // How I did it earlier lol
        // let mut counter = 0;
        // let (mut is_whitespace, mut left_bracket, mut right_bracket, mut empty_stack) = (true, false, false, false);
        // line.chars().map(|char| {
        //     if char == ' ' {
        //         if is_whitespace {
        //             left_bracket = true;
        //             is_whitespace = false;
        //             let y = 'w';
        //             y
        //         } else if left_bracket {
        //             empty_stack = true;
        //             left_bracket = false;
        //             let y = '[';
        //             y
        //         } else if empty_stack {
        //             right_bracket = true;
        //             empty_stack = false;
        //             char
        //         } else if right_bracket {
        //             is_whitespace = true;
        //             right_bracket = false;
        //             let y = ']';
        //             y
        //         } else {
        //             panic!()
        //         }
        //     } else {
        //         (is_whitespace, left_bracket, right_bracket, empty_stack) = (true, false, false, false);
        //         char
        //     }
        // }).filter(|char| {
        //     !(*char == '[' || *char == ']' || *char == 'w')
        // }).for_each(|char| {
        //   if char != ' ' {
        //     stacks_array[counter].push(char);
        //   };
        //   counter += 1;
        // });

        } else {
            let mut instructions: Vec<usize> = line.chars().filter(|char| {
                char.is_numeric()
            }).map(|char| {
                char.to_digit(10).expect("Should contain only digits from 0-9") as usize
            }).collect();
            
            let (to, from) = (instructions.pop().unwrap() - 1, instructions.pop().unwrap() - 1);
            let amount = instructions.into_iter().reduce(|acc, num| acc * 10 + num).unwrap();

            for _ in 0..amount {
                let stack = stacks_array[from].pop().unwrap();
                stacks_array[to].push(stack);
            };
        };
    };
    let mut remaining_stacks: [char; 9] = [' '; 9];

    for i in 0..9 {
        let stack = stacks_array[i].pop().unwrap();
        remaining_stacks[i] = stack;
    };
    remaining_stacks.iter().collect()
}
