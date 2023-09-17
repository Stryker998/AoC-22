use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    RightUp,
    LeftUp,
    RightDown,
    LeftDown
}


pub fn solution(input: &String) -> usize{
    let mut x_head = 0;
    let mut y_head = 0;
    let mut x_tail = 0;
    let mut y_tail = 0;


    let mut visited_once = HashSet::new();


    for line in input.lines() {

        let mut char_iter = line.chars();
        let direction: Direction;

        match char_iter.next().unwrap() {
            'U' => direction = Direction::Up,
            'D' => direction = Direction::Down,
            'L' => direction = Direction::Left,
            'R' => direction = Direction::Right,
            _ => panic!()
        };

        char_iter.next();

        for _ in 0..char_iter.fold(0, |acc, val| acc * 10 + val.to_digit(10).unwrap()) {
            
            match direction {
                Direction::Up => y_head += 1,
                Direction::Down => y_head -= 1,
                Direction::Left => x_head -= 1,
                Direction::Right => x_head += 1,
                _ => panic!()
            };

            let (has_not_touched, direction_to_mutate) = check_touch(x_head, y_head, x_tail, y_tail, direction);

            if has_not_touched {
                match direction_to_mutate {
                    Direction::Up => {
                        y_tail += 1;
                    },
                    Direction::Down => {
                        y_tail -= 1;
                    },
                    Direction::Left => {
                        x_tail -= 1;
                    },
                    Direction::Right => {
                        x_tail += 1;
                    },
                    Direction::RightUp => {
                        x_tail += 1;
                        y_tail += 1;
                    },
                    Direction::LeftUp => {
                        x_tail -= 1;
                        y_tail += 1;
                    },
                    Direction::RightDown => {
                        x_tail += 1;
                        y_tail -= 1;
                    },
                    Direction::LeftDown => {
                        x_tail -= 1;
                        y_tail -= 1;
                    },
                }
            }


            visited_once.insert((x_tail, y_tail));

//            println!("visited_once: {:?}", visited_once);
        }; 
    }

    visited_once.iter().count() 
}

fn check_touch(x_head: i32, y_head: i32, x_tail: i32, y_tail: i32, direction: Direction) -> (bool, Direction) {

    let mut result: (bool, Direction) = (false, direction);

    let up_head = y_head + 1;
    let down_head = y_head - 1;
    let left_head = x_head - 1;
    let right_head = x_head + 1;

    let check_array = [
        (x_head, y_head),
        (x_head, up_head),
        (x_head, down_head),
        (left_head, y_head),
        (right_head, y_head),
        (left_head,up_head),
        (left_head, down_head),
        (right_head, up_head),
        (right_head, down_head)
    ];

    for (x, y) in check_array.iter() {
        if x_tail == *x && y_tail == *y {
            return result;
        };
    };

    result.0 = true;

    if x_head == x_tail || y_head == y_tail {
        return result;
    };

    let up_tail = y_tail + 1;
    let down_tail = y_tail - 1;
    let left_tail = x_tail - 1;
    let right_tail = x_tail + 1;

    let check_array_tail = [
        (left_tail, up_tail, Direction::LeftUp),
        (left_tail, down_tail, Direction::LeftDown),
        (right_tail, up_tail, Direction::RightUp),
        (right_tail, down_tail, Direction::RightDown)
    ];

    for (x, y, dir) in check_array_tail.into_iter() {
        if check_array.contains(&(x,y)) {
            result.1 = dir;
            return result;
        };
    };

    panic!();
}
