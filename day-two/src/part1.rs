use itertools::Itertools;

#[derive(PartialEq)]
enum Options {
    Rock,
    Paper,
    Scissor
}

struct PickedOption {
    value: i32,
    current: Options,
    next: Options
}

impl PickedOption {
    fn rock() -> PickedOption {
        PickedOption { value: 1, current: Options::Rock, next: Options::Paper }
    }
    fn paper() -> PickedOption {
        PickedOption { value: 2, current: Options::Paper, next: Options::Scissor }
    }
    fn scissor() -> PickedOption {
        PickedOption { value: 3, current: Options::Scissor, next: Options::Rock }
    }
}

pub fn solution(string: &String) -> i32 {
    let mut score = 0;
    for line in string.lines() {
        let mut picked_options: Vec<PickedOption> = vec![];
        for char in line.chars() {
            let option = match char {
                'A' | 'X' => PickedOption::rock(),
                'B' | 'Y' => PickedOption::paper(),
                'C' | 'Z' => PickedOption::scissor(),
                _ => continue
            };
            picked_options.push(option);
        };
        let (option1, option2) = picked_options.iter().collect_tuple().unwrap();
        if option2.current == option1.current {
            score += option2.value + 3;
        } else if option2.next == option1.current {
            score += option2.value;
        } else {
            score += option2.value + 6;
        };
    };
    score
}