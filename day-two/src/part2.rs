
struct PickedOption {
    win: i32,
    draw: i32,
    loss: i32
}

impl PickedOption {
    fn rock() -> PickedOption {
        PickedOption { win: 2, draw: 1, loss: 3 }
    }
    fn paper() -> PickedOption {
        PickedOption { win: 3, draw: 2, loss: 1 }
    }
    fn scissor() -> PickedOption {
        PickedOption { win: 1, draw: 3, loss: 2 }
    }
}

pub fn solution(string: &String) -> i32 {
    let mut score = 0;
    for line in string.lines() {

        let option = if line.contains('A') {
            PickedOption::rock()
        } else if line.contains('B') {
            PickedOption::paper()
        } else if line.contains('C'){
            PickedOption::scissor()
        } else {
            panic!()
        };

        if line.contains('X') {
            score += &option.loss;
        } else if line.contains('Y') {
            score += &option.draw + 3;
        } else if line.contains('Z'){
            score += &option.win + 6;
        };
        
    }
    score 
}