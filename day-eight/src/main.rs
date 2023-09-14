use std::fs;

struct ArenaGraph {
    arena: Vec<Node>,
}

struct Node {
    id: usize,
    up: Option<usize>,
    down: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
    height: u32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Node {
    fn get_next_node(&self, direction: &Direction) -> Option<usize> {
        match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

impl ArenaGraph {
    fn add(&mut self, height: u32, count: usize, edge: bool) -> usize {
        let id = self.arena.len();
        let mut up = None;
        let down = None;
        let mut left = None;
        let right = None;

        // Setting right of previous and left of current
        if !edge {
            if let Some(prev) = self.arena.last_mut() {
                left = Some(prev.id);
                prev.right = Some(id);
            };
        };
        // Setting down of previous and up of current
        if let Some(pos) = id.checked_sub(count) {
            if let Some(above) = self.arena.get_mut(pos) {
                up = Some(above.id);
                above.down = Some(id);
            };
        };

        self.arena.push(Node {
            id,
            up,
            down,
            left,
            right,
            height,
        });
        id
    }

    fn part1(&self) -> Vec<usize> {
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        let mut visible_trees = vec![];
        let default = Node {
            id: 0,
            up: None,
            down: None,
            left: None,
            right: None,
            height: 0,
        };

        for node in &self.arena {
            for direction in directions.iter() {
                let mut visible = true;
                let mut current = Some(node);
                while let Some(next) = current.unwrap_or(&default).get_next_node(direction) {
                    let next_node = self.arena.get(next).unwrap();
                    if node.height <= next_node.height {
                        visible = false;
                        current = None;
                    } else {
                        current = Some(next_node);
                        visible = true;
                    }
                }
                if visible {
                    visible_trees.push(node.id);
                    break;
                };
            }
        }
        visible_trees
    }

    fn part2(&self) -> i32 {
        let directions = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        let default = Node {
            id: 0,
            up: None,
            down: None,
            left: None,
            right: None,
            height: 0,
        };
        let mut final_score = 0;
        for node in &self.arena {
            let mut node_score = 1;
            for direction in directions.iter() {
                let mut current = Some(node);
                let mut count = 0;
                while let Some(next) = current.unwrap_or(&default).get_next_node(direction) {
                    count += 1;
                    let next_node = self.arena.get(next).unwrap();
                    if node.height <= next_node.height {
                        current = None;
                    } else {
                        current = Some(next_node);
                    };
                }
                node_score *= count;
            }
            if final_score < node_score {
                final_score = node_score;
            }
        }
        final_score
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let count = input.lines().next().unwrap().chars().count();
    let mut arena_graph = ArenaGraph { arena: vec![] };

    println!("{}", count);

    for line in input.lines() {
        line.char_indices().for_each(|(index, height)| {
            let edge = index == 0;
            arena_graph.add(height.to_digit(10).unwrap(), count, edge);
        });
    }

    let part1 = arena_graph.part1();
    let part2 = arena_graph.part2();
    println!("{}", part1.iter().count());
    println!("{}", part2);
}
