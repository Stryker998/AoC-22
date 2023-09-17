use std::{collections::HashSet, array};

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

#[derive(Debug)]
struct Knot {
    id: usize,
    x: i32,
    y: i32
}


pub fn solution(input: &String) -> usize{
    
    let mut rope: [Knot; 10] = array::from_fn(|i| Knot { id: i, x: 0, y: 0 });

    let mut visited_once = HashSet::new();
    let mut counter = 0;

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
           
            let mut rope_iter = rope.iter_mut();
            let mut head = rope_iter.next().unwrap();
            
            match direction {
                Direction::Up => head.y += 1,
                Direction::Down => head.y -= 1,
                Direction::Left => head.x -= 1,
                Direction::Right => head.x += 1,
                _ => panic!()
            };

            for knot in rope_iter {
                let (has_not_touched, direction_to_mutate) = check_touch(head.x, head.y, knot.x, knot.y, direction, &mut counter);
                if has_not_touched {
                    match direction_to_mutate {
                        Direction::Up => {
                            knot.y += 1;
                        },
                        Direction::Down => {
                            knot.y -= 1;
                        },
                        Direction::Left => {
                            knot.x -= 1;
                        },
                        Direction::Right => {
                            knot.x += 1;
                        },
                        Direction::RightUp => {
                            knot.x += 1;
                            knot.y += 1;
                        },
                        Direction::LeftUp => {
                            knot.x -= 1;
                            knot.y += 1;
                        },
                        Direction::RightDown => {
                            knot.x += 1;
                            knot.y -= 1;
                        },
                        Direction::LeftDown => {
                            knot.x -= 1;
                            knot.y -= 1;
                        },
                    }
                };
                head = knot;
            }

            visited_once.insert((head.x, head.y));
            // println!("visited_once: {:?}", visited_once);
        }; 
    }


    visited_once.iter().count() 
}

fn check_touch(x_head: i32, y_head: i32, x_tail: i32, y_tail: i32, direction: Direction, counter: &mut i32) -> (bool, Direction) {
    *counter += 1;
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
        if x_head - x_tail < 0 {
            result.1 = Direction::Left; 
        } else if x_head - x_tail > 0 { 
            result.1 = Direction::Right;
        } else if y_head - y_tail < 0 {
            result.1 = Direction::Down;
        } else if y_head - y_tail > 0 {
            result.1 = Direction::Up;
        };

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
    
    println!("{}", counter);
    println!("x_head: {}, y_head: {}, x_tail: {}, y_tail: {}", x_head, y_head, x_tail, y_tail);
    panic!();
}
