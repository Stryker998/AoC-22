use std::fs;

#[derive(Debug)]
struct ArenaTree {
    arena: Vec<Node>
}

#[derive(Debug)]
struct Node {
    id: usize,
    parent: Option<usize>,
    children: Vec<usize>,
    size: u32,
    name: String,
    is_dir: bool
}

impl Node {
    fn new(id: usize, parent: Option<usize>, size: u32, name: String, is_dir: bool) -> Self {
        Self {
            id,
            parent,
            children: vec![],
            size,
            name,
            is_dir
        }
    }
}

impl ArenaTree {
    fn cd(&self, current_dir: usize, name: String) -> usize {
        let current_node = self.arena.get(current_dir).unwrap();    
        for id in &current_node.children {
            if self.arena.get(*id).unwrap().name == name {
                return *id
            }  
        }
        println!("current_dir: {}, name: {}", current_dir, name);
        self.print_all();
        panic!()
    }

    fn add(&mut self, name: String, parent: Option<usize>, size: u32, is_dir: bool) -> usize {
        let id = self.arena.len();
        let new_node = Node::new(id, parent, size, name, is_dir);
        if let Some(parent_id) = new_node.parent {
            self.arena.get_mut(parent_id).unwrap().children.push(id);
            self.calculate_size(new_node.size, parent_id);
        }
        self.arena.push(new_node);
        id
    }

    fn calculate_size(&mut self, size: u32, id: usize) {
        let node = self.arena.get_mut(id).unwrap();
        node.size += size;
        let parent_node = node.parent;
        if let Some(parent_id) = parent_node {
           self.calculate_size(size, parent_id);
        }
    }

    fn size(&self) -> u32 {
        let mut sum = 0;
        for node in &self.arena {
            if node.is_dir && node.size < 100000 {
                sum += node.size;
            }
        }
        sum
    }

    fn print_all(&self) {
        for node in &self.arena {
            println!("{:?}", node);
        }
    }

    fn get_required_min(&self, required_space: u32) -> u32 {
        let mut list_of_nodes = vec![];
        for node in &self.arena {
            if node.size > required_space {
                list_of_nodes.push(node.size);
            }
        };
        *list_of_nodes.iter().min().unwrap()
    }

    fn file_size(&self) -> u32 {
        let mut sum = 0;
        for node in &self.arena {
            if !node.is_dir {
                sum += node.size;
            }
        }
        sum
    }
}


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut ls_mode = false;
    let mut arena_tree = ArenaTree {
        arena: vec![]
    };
    let arena_ptr = &mut arena_tree;
    let mut current_dir: usize = 0; 

    for line in input.lines() {
        if line.contains("$") {
            ls_mode = false;
        }

        if line.contains("$ cd /") {
            current_dir = arena_ptr.add("/".to_owned(), None, 0, true);
            continue;
        }

        if ls_mode {
            if line.contains("dir") {
                let dir_name = line.get(4..).unwrap().to_owned();
                arena_ptr.add(dir_name, Some(current_dir), 0, true);
            } else {
                let file_size = line.chars()
                    .filter(|char| {
                        char.is_numeric()
                    }).map(|char| {
                        let value = char.to_digit(10).unwrap();
                        value
                    }).reduce(|acc, val| {
                        acc * 10 + val
                    }).unwrap_or_else(|| {
                        println!("{}",line);
                        panic!()
                    });
                let file_name: String = line.chars()
                    .filter(|char| {
                        !(char.is_numeric() || char.is_whitespace())
                    }).collect();
                arena_ptr.add(file_name, Some(current_dir), file_size, false);
            }
        } else {
            if line.contains("cd") {
                let dir_name = line.get(5..).unwrap().to_owned();
                current_dir = if dir_name.contains("..") {
                    let current_node = arena_ptr.arena.get(current_dir).unwrap();
                    current_node.parent.unwrap_or_else(|| {
                        println!("current_dir: {}", current_dir);
                        println!("line: {}", line);
                        println!("current_name: {}", current_node.name);
                        arena_ptr.print_all();
                        panic!()
                    })
                } else {
                    arena_ptr.cd(current_dir, dir_name)
                }
            } else {
                ls_mode = true; 
            }
        }
    }
    println!("{}", arena_ptr.size());

    let used_space = arena_ptr.arena.get(0).unwrap().size;
    let unused_space = 70000000 - used_space;
    let required_space = 30000000 - unused_space;
    
    println!("{}", arena_ptr.get_required_min(required_space));
}

